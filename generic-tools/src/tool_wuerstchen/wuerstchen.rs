#[cfg(feature = "accelerate")]
extern crate accelerate_src;

use candle_transformers::models::stable_diffusion;
use candle_transformers::models::wuerstchen;

use crate::candle_tools::candletools::{get_device, save_image};
use anyhow::{Error as E, Result};
use candle_core::IndexOp;
use candle_core::{DType, Device, Tensor};
use candle_transformers::models::mimi::candle;
use tokenizers::Tokenizer;

const PRIOR_GUIDANCE_SCALE: f64 = 4.0;
const RESOLUTION_MULTIPLE: f64 = 42.67;
const LATENT_DIM_SCALE: f64 = 10.67;
const PRIOR_CIN: usize = 16;
const DECODER_CIN: usize = 4;

struct Args {
    /// The prompt to be used for image generation.
    prompt: String,
    uncond_prompt: String,
    _cpu: bool,
    use_flash_attn: bool,
    height: Option<usize>,
    width: Option<usize>,
    decoder_weights: Option<String>,
    clip_weights: Option<String>,
    prior_clip_weights: Option<String>,
    prior_weights: Option<String>,
    vqgan_weights: Option<String>,
    tokenizer: Option<String>,
    prior_tokenizer: Option<String>,
    num_samples: i64,
    final_image: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModelFile {
    Tokenizer,
    PriorTokenizer,
    Clip,
    PriorClip,
    Decoder,
    VqGan,
    Prior,
}

impl ModelFile {
    fn get(&self, filename: Option<String>) -> Result<std::path::PathBuf> {
        use candle_hf_hub::api::sync::Api;
        match filename {
            Some(filename) => Ok(std::path::PathBuf::from(filename)),
            None => {
                let repo_main = "warp-ai/wuerstchen";
                let repo_prior = "warp-ai/wuerstchen-prior";
                let (repo, path) = match self {
                    Self::Tokenizer => (repo_main, "tokenizer/tokenizer.json"),
                    Self::PriorTokenizer => (repo_prior, "tokenizer/tokenizer.json"),
                    Self::Clip => (repo_main, "text_encoder/model.safetensors"),
                    Self::PriorClip => (repo_prior, "text_encoder/model.safetensors"),
                    Self::Decoder => (repo_main, "decoder/diffusion_pytorch_model.safetensors"),
                    Self::VqGan => (repo_main, "vqgan/diffusion_pytorch_model.safetensors"),
                    Self::Prior => (repo_prior, "prior/diffusion_pytorch_model.safetensors"),
                };
                let filename = Api::new()?.model(repo.to_string()).get(path)?;
                Ok(filename)
            }
        }
    }
}

fn output_filename(
    basename: &str,
    sample_idx: i64,
    num_samples: i64,
    timestep_idx: Option<usize>,
) -> String {
    let filename = if num_samples > 1 {
        match basename.rsplit_once('.') {
            None => format!("{basename}.{sample_idx}.png"),
            Some((filename_no_extension, extension)) => {
                format!("{filename_no_extension}.{sample_idx}.{extension}")
            }
        }
    } else {
        basename.to_string()
    };
    let filename = filename.as_str();
    match timestep_idx {
        None => filename.to_string(),
        Some(timestep_idx) => match filename.rsplit_once('.') {
            None => format!("{filename}-{timestep_idx:03}.png"),
            Some((filename_no_extension, extension)) => {
                format!("{filename_no_extension}-{timestep_idx:03}.{extension}")
            }
        },
    }
}

fn encode_prompt(
    prompt: &str,
    uncond_prompt: Option<&str>,
    tokenizer: std::path::PathBuf,
    clip_weights: std::path::PathBuf,
    clip_config: stable_diffusion::clip::Config,
    device: &Device,
) -> Result<Tensor> {
    let tokenizer = Tokenizer::from_file(tokenizer).map_err(E::msg)?;
    let pad_id = match &clip_config.pad_with {
        Some(padding) => *tokenizer.get_vocab(true).get(padding.as_str()).unwrap(),
        None => *tokenizer.get_vocab(true).get("<|endoftext|>").unwrap(),
    };
    println!("Running with prompt \"{prompt}\".");
    let mut tokens = tokenizer
        .encode(prompt, true)
        .map_err(E::msg)?
        .get_ids()
        .to_vec();
    let tokens_len = tokens.len();
    while tokens.len() < clip_config.max_position_embeddings {
        tokens.push(pad_id)
    }
    let tokens = Tensor::new(tokens.as_slice(), device)?.unsqueeze(0)?;

    println!("Building the clip transformer.");
    let text_model =
        stable_diffusion::build_clip_transformer(&clip_config, clip_weights, device, DType::F32)?;
    let text_embeddings = text_model.forward_with_mask(&tokens, tokens_len - 1)?;
    match uncond_prompt {
        None => Ok(text_embeddings),
        Some(uncond_prompt) => {
            let mut uncond_tokens = tokenizer
                .encode(uncond_prompt, true)
                .map_err(E::msg)?
                .get_ids()
                .to_vec();
            let uncond_tokens_len = uncond_tokens.len();
            while uncond_tokens.len() < clip_config.max_position_embeddings {
                uncond_tokens.push(pad_id)
            }
            let uncond_tokens = Tensor::new(uncond_tokens.as_slice(), device)?.unsqueeze(0)?;

            let uncond_embeddings =
                text_model.forward_with_mask(&uncond_tokens, uncond_tokens_len - 1)?;
            let text_embeddings = Tensor::cat(&[text_embeddings, uncond_embeddings], 0)?;
            Ok(text_embeddings)
        }
    }
}

fn run(args: Args) -> Result<()> {
    // use tracing_chrome::ChromeLayerBuilder;
    // use tracing_subscriber::prelude::*;

    println!(
        "avx: {}, neon: {}, simd128: {}, f16c: {}",
        candle::utils::with_avx(),
        candle::utils::with_neon(),
        candle::utils::with_simd128(),
        candle::utils::with_f16c()
    );

    let Args {
        prompt,
        uncond_prompt,
        height,
        width,
        tokenizer,
        final_image,
        num_samples,
        clip_weights,
        prior_weights,
        vqgan_weights,
        decoder_weights,
        ..
    } = args;

    let device = get_device(false)?;
    println!("wuerstchen device {:?}", device);

    let height = height.unwrap_or(1024);
    let width = width.unwrap_or(1024);

    let prior_text_embeddings = {
        let tokenizer = ModelFile::PriorTokenizer.get(args.prior_tokenizer)?;
        let weights = ModelFile::PriorClip.get(args.prior_clip_weights)?;
        encode_prompt(
            prompt.as_str(),
            Some(uncond_prompt.as_str()),
            tokenizer.clone(),
            weights,
            stable_diffusion::clip::Config::wuerstchen_prior(),
            &device,
        )?
    };
    println!("generated prior text embeddings {prior_text_embeddings:?}");

    let text_embeddings = {
        let tokenizer = ModelFile::Tokenizer.get(tokenizer)?;
        let weights = ModelFile::Clip.get(clip_weights)?;
        encode_prompt(
            prompt.as_str(),
            None,
            tokenizer.clone(),
            weights,
            stable_diffusion::clip::Config::wuerstchen(),
            &device,
        )?
    };
    println!("generated text embeddings {text_embeddings:?}");

    println!("Building the prior.");
    let b_size = 1;
    let image_embeddings = {
        // https://huggingface.co/warp-ai/wuerstchen-prior/blob/main/prior/config.json
        let latent_height = (height as f64 / RESOLUTION_MULTIPLE).ceil() as usize;
        let latent_width = (width as f64 / RESOLUTION_MULTIPLE).ceil() as usize;
        let mut latents = Tensor::randn(
            0f32,
            1f32,
            (b_size, PRIOR_CIN, latent_height, latent_width),
            &device,
        )?;

        let prior = {
            let file = ModelFile::Prior.get(prior_weights)?;
            let vb = unsafe {
                candle_nn::VarBuilder::from_mmaped_safetensors(&[file], DType::F32, &device)?
            };
            wuerstchen::prior::WPrior::new(
                /* c_in */ PRIOR_CIN,
                /* c */ 1536,
                /* c_cond */ 1280,
                /* c_r */ 64,
                /* depth */ 32,
                /* nhead */ 24,
                args.use_flash_attn,
                vb,
            )?
        };
        let prior_scheduler = wuerstchen::ddpm::DDPMWScheduler::new(60, Default::default())?;
        let timesteps = prior_scheduler.timesteps();
        let timesteps = &timesteps[..timesteps.len() - 1];
        println!("prior denoising");
        for (index, &t) in timesteps.iter().enumerate() {
            let start_time = std::time::Instant::now();
            let latent_model_input = Tensor::cat(&[&latents, &latents], 0)?;
            let ratio = (Tensor::ones(2, DType::F32, &device)? * t)?;
            let noise_pred = prior.forward(&latent_model_input, &ratio, &prior_text_embeddings)?;
            let noise_pred = noise_pred.chunk(2, 0)?;
            let (noise_pred_text, noise_pred_uncond) = (&noise_pred[0], &noise_pred[1]);
            let noise_pred = (noise_pred_uncond
                + ((noise_pred_text - noise_pred_uncond)? * PRIOR_GUIDANCE_SCALE)?)?;
            latents = prior_scheduler.step(&noise_pred, t, &latents)?;
            let dt = start_time.elapsed().as_secs_f32();
            println!("step {}/{} done, {:.2}s", index + 1, timesteps.len(), dt);
        }
        ((latents * 42.)? - 1.)?
    };

    println!("Building the vqgan.");
    let vqgan = {
        let file = ModelFile::VqGan.get(vqgan_weights)?;
        let vb = unsafe {
            candle_nn::VarBuilder::from_mmaped_safetensors(&[file], DType::F32, &device)?
        };
        wuerstchen::paella_vq::PaellaVQ::new(vb)?
    };

    println!("Building the decoder.");

    // https://huggingface.co/warp-ai/wuerstchen/blob/main/decoder/config.json
    let decoder = {
        let file = ModelFile::Decoder.get(decoder_weights)?;
        let vb = unsafe {
            candle_nn::VarBuilder::from_mmaped_safetensors(&[file], DType::F32, &device)?
        };
        wuerstchen::diffnext::WDiffNeXt::new(
            /* c_in */ DECODER_CIN,
            /* c_out */ DECODER_CIN,
            /* c_r */ 64,
            /* c_cond */ 1024,
            /* clip_embd */ 1024,
            /* patch_size */ 2,
            args.use_flash_attn,
            vb,
        )?
    };

    println!("creating samples.  {}", num_samples);

    for idx in 0..num_samples {
        // https://huggingface.co/warp-ai/wuerstchen/blob/main/model_index.json
        let latent_height = (image_embeddings.dim(2)? as f64 * LATENT_DIM_SCALE) as usize;
        let latent_width = (image_embeddings.dim(3)? as f64 * LATENT_DIM_SCALE) as usize;

        let mut latents = Tensor::randn(
            0f32,
            1f32,
            (b_size, DECODER_CIN, latent_height, latent_width),
            &device,
        )?;

        println!("diffusion process with prior {image_embeddings:?}");
        let scheduler = wuerstchen::ddpm::DDPMWScheduler::new(12, Default::default())?;
        let timesteps = scheduler.timesteps();
        let timesteps = &timesteps[..timesteps.len() - 1];
        for (index, &t) in timesteps.iter().enumerate() {
            let start_time = std::time::Instant::now();
            let ratio = (Tensor::ones(1, DType::F32, &device)? * t)?;
            let noise_pred =
                decoder.forward(&latents, &ratio, &image_embeddings, Some(&text_embeddings))?;
            latents = scheduler.step(&noise_pred, t, &latents)?;
            let dt = start_time.elapsed().as_secs_f32();
            println!("step {}/{} done, {:.2}s", index + 1, timesteps.len(), dt);
        }
        println!(
            "Generating the final image for sample {}/{}.",
            idx + 1,
            num_samples
        );
        let image = vqgan.decode(&(&latents * 0.3764)?)?;
        let image = (image.clamp(0f32, 1f32)? * 255.)?
            .to_dtype(DType::U8)?
            .i(0)?;
        let image_filename = output_filename(&final_image, idx + 1, num_samples, None);
        match save_image(&image, image_filename.clone()) {
            Ok(_) => {
                println!("image saved successfully: {}", image_filename);
            }
            Err(e) => {
                println!("error saving image {}. error:  {:?}", image_filename, e)
            }
        }
    }
    Ok(())
}

pub fn run_wuerstchen(prompt: String, file_name: String) -> Result<()> {
    let final_image = format!("final_{}", file_name);
    let args = Args {
        prompt,
        uncond_prompt: "".to_string(),
        _cpu: false,
        use_flash_attn: false,
        height: Some(512),
        width: Some(512),
        decoder_weights: None,
        clip_weights: None,
        prior_clip_weights: None,
        prior_weights: None,
        vqgan_weights: None,
        tokenizer: None,
        prior_tokenizer: None,
        num_samples: 8,
        final_image,
    };
    run(args)
}
