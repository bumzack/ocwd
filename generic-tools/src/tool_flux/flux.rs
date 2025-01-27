#[cfg(feature = "accelerate")]
extern crate accelerate_src;

use candle_transformers::models::{clip, flux, t5};
use std::error::Error;

use crate::candle_tools::candletools::{get_device, save_image};
use anyhow::{Error as E, Result};
use candle_core::{IndexOp, Module, Tensor};
use candle_nn::VarBuilder;
use tokenizers::Tokenizer;

struct Args {
    /// The prompt to be used for image generation.
    prompt: String,
    /// Use the quantized model.
    quantized: bool,
    /// The height in pixels of the generated image.
    height: Option<usize>,
    /// The width in pixels of the generated image.
    width: Option<usize>,
    decode_only: Option<String>,
    // default = schnell
    model: WhichFlux,
    /// The seed to use when generating random samples.
    seed: Option<u64>,
    file_path: String,
    file_name: String,
    use_cpu: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhichFlux {
    Schnell,
    Dev,
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let Args {
        prompt,
        height,
        width,
        decode_only,
        model,
        quantized,
        ..
    } = args;

    let width = width.unwrap_or(1360);
    let height = height.unwrap_or(768);

    let api = candle_hf_hub::api::sync::Api::new()?;
    let bf_repo = {
        let name = match model {
            WhichFlux::Dev => "black-forest-labs/FLUX.1-dev",
            WhichFlux::Schnell => "black-forest-labs/FLUX.1-schnell",
        };
        api.repo(candle_hf_hub::Repo::model(name.to_string()))
    };
    let device = get_device(args.use_cpu)?;
    println!("flux using device {:?}", device);

    if let Some(seed) = args.seed {
        // if using CPU setting seed does not work ¯\_(ツ)_/
        if !args.use_cpu {
            device.set_seed(seed)?;
        }
    }

    let dtype = device.bf16_default_to_f32();
    let img = match decode_only {
        None => {
            let t5_emb = {
                let repo = api.repo(candle_hf_hub::Repo::with_revision(
                    "google/t5-v1_1-xxl".to_string(),
                    candle_hf_hub::RepoType::Model,
                    "refs/pr/2".to_string(),
                ));
                let model_file = repo.get("model.safetensors")?;
                let vb =
                    unsafe { VarBuilder::from_mmaped_safetensors(&[model_file], dtype, &device)? };
                let config_filename = repo.get("config.json")?;
                let config = std::fs::read_to_string(config_filename)?;
                let config: t5::Config = serde_json::from_str(&config)?;
                let mut model = t5::T5EncoderModel::load(vb, &config)?;
                let tokenizer_filename = api
                    .model("lmz/mt5-tokenizers".to_string())
                    .get("t5-v1_1-xxl.tokenizer.json")?;

                let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;
                let mut tokens = tokenizer
                    .encode(prompt.as_str(), true)
                    .map_err(E::msg)?
                    .get_ids()
                    .to_vec();

                tokens.resize(256, 0);
                let input_token_ids = Tensor::new(&tokens[..], &device)?.unsqueeze(0)?;
                //  println!("{input_token_ids}");
                model.forward(&input_token_ids)?
            };
            // println!("T5\n{t5_emb}");
            let clip_emb = {
                let repo = api.repo(candle_hf_hub::Repo::model(
                    "openai/clip-vit-large-patch14".to_string(),
                ));
                let model_file = repo.get("model.safetensors")?;
                let vb =
                    unsafe { VarBuilder::from_mmaped_safetensors(&[model_file], dtype, &device)? };
                // https://huggingface.co/openai/clip-vit-large-patch14/blob/main/config.json
                let config = clip::text_model::ClipTextConfig {
                    vocab_size: 49408,
                    projection_dim: 768,
                    activation: clip::text_model::Activation::QuickGelu,
                    intermediate_size: 3072,
                    embed_dim: 768,
                    max_position_embeddings: 77,
                    pad_with: None,
                    num_hidden_layers: 12,
                    num_attention_heads: 12,
                };
                let model =
                    clip::text_model::ClipTextTransformer::new(vb.pp("text_model"), &config)?;
                let tokenizer_filename = repo.get("tokenizer.json")?;
                let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;
                let tokens = tokenizer
                    .encode(prompt.as_str(), true)
                    .map_err(E::msg)?
                    .get_ids()
                    .to_vec();
                let input_token_ids = Tensor::new(&tokens[..], &device)?.unsqueeze(0)?;
                //  println!("{input_token_ids}");
                model.forward(&input_token_ids)?
            };
            // println!("CLIP\n{clip_emb}");
            let img = {
                let cfg = match model {
                    WhichFlux::Dev => flux::model::Config::dev(),
                    WhichFlux::Schnell => flux::model::Config::schnell(),
                };
                let img = flux::sampling::get_noise(1, height, width, &device)?.to_dtype(dtype)?;
                let state = if quantized {
                    flux::sampling::State::new(
                        &t5_emb.to_dtype(candle_core::DType::F32)?,
                        &clip_emb.to_dtype(candle_core::DType::F32)?,
                        &img.to_dtype(candle_core::DType::F32)?,
                    )?
                } else {
                    flux::sampling::State::new(&t5_emb, &clip_emb, &img)?
                };
                let timesteps = match model {
                    WhichFlux::Dev => {
                        flux::sampling::get_schedule(50, Some((state.img.dim(1)?, 0.5, 1.15)))
                    }
                    WhichFlux::Schnell => flux::sampling::get_schedule(4, None),
                };
                //  println!("{state:?}");
                // println!("{timesteps:?}");
                if quantized {
                    let model_file = match model {
                        WhichFlux::Schnell => api
                            .repo(candle_hf_hub::Repo::model("lmz/candle-flux".to_string()))
                            .get("flux1-schnell.gguf")?,
                        WhichFlux::Dev => todo!(),
                    };
                    let vb = candle_transformers::quantized_var_builder::VarBuilder::from_gguf(
                        model_file, &device,
                    )?;

                    let model = flux::quantized_model::Flux::new(&cfg, vb)?;
                    flux::sampling::denoise(
                        &model,
                        &state.img,
                        &state.img_ids,
                        &state.txt,
                        &state.txt_ids,
                        &state.vec,
                        &timesteps,
                        4.,
                    )?
                    .to_dtype(dtype)?
                } else {
                    let model_file = match model {
                        WhichFlux::Schnell => bf_repo.get("flux1-schnell.safetensors")?,
                        WhichFlux::Dev => bf_repo.get("flux1-dev.safetensors")?,
                    };
                    let vb = unsafe {
                        VarBuilder::from_mmaped_safetensors(&[model_file], dtype, &device)?
                    };
                    let model = flux::model::Flux::new(&cfg, vb)?;
                    flux::sampling::denoise(
                        &model,
                        &state.img,
                        &state.img_ids,
                        &state.txt,
                        &state.txt_ids,
                        &state.vec,
                        &timesteps,
                        4.,
                    )?
                }
            };
            flux::sampling::unpack(&img, height, width)?
        }
        Some(file) => {
            let mut st = candle_core::safetensors::load(file, &device)?;
            st.remove("img").unwrap().to_dtype(dtype)?
        }
    };
    // println!("latent img\n{img}");

    let img = {
        let model_file = bf_repo.get("ae.safetensors")?;
        let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[model_file], dtype, &device)? };
        let cfg = match model {
            WhichFlux::Dev => flux::autoencoder::Config::dev(),
            WhichFlux::Schnell => flux::autoencoder::Config::schnell(),
        };
        let model = flux::autoencoder::AutoEncoder::new(&cfg, vb)?;
        model.decode(&img)?
    };
    // println!("img\n{img}");
    let img = ((img.clamp(-1f32, 1f32)? + 1.0)? * 127.5)?.to_dtype(candle_core::DType::U8)?;
    let filename = match args.seed {
        None => format!("{}/{}.png", args.file_path, args.file_name),
        Some(s) => format!("{}/{}-{s}.png", args.file_path, args.file_name),
    };

    // error saving file flux_dev_1737845077/./.jpg. error The image format could not be determined
    println!("flux filename: {filename}");
    let res = save_image(&img.i(0)?, &filename);
    match res {
        Ok(_) => {
            println!("flux saved image to {}", &filename);
            Ok(())
        }
        Err(e) => {
            let msg = format!("flux error saving image: {}", e);
            *Box::new(Err(msg.into()))
        }
    }
}

pub fn run_flux(
    prompt: String,
    use_dmmv: bool,
    width: Option<usize>,
    height: Option<usize>,
    file_path: String,
    file_name: String,
    which: WhichFlux,
    seed: u64,
    use_cpu: bool,
) -> std::result::Result<(), Box<dyn Error>> {
    #[cfg(feature = "cuda")]
    candle_core::quantized::cuda::set_force_dmmv(use_dmmv);

    let args = Args {
        prompt,
        quantized: false,
        height,
        width,
        decode_only: None,
        model: which,
        seed: Some(seed),
        file_path,
        file_name,
        use_cpu,
    };

    run(args)
}
