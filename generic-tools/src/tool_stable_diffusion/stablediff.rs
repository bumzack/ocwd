#[cfg(feature = "accelerate")]
extern crate accelerate_src;

use candle_core::{DType, IndexOp, Tensor};
use candle_transformers::models::mmdit::model::{Config as MMDiTConfig, MMDiT};
use std::error::Error;

use crate::candle_tools::candletools::{get_device, save_image};
use crate::tool_stable_diffusion::clip::StableDiffusion3TripleClipWithTokenizer;
use crate::tool_stable_diffusion::sampling;
use crate::tool_stable_diffusion::vae::{build_sd3_vae_autoencoder, sd3_vae_vb_rename};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum StableDiffusionWhich {
    V3Medium,
    V3_5Large,
    V3_5LargeTurbo,
    V3_5Medium,
}

impl StableDiffusionWhich {
    fn is_3_5(&self) -> bool {
        match self {
            Self::V3Medium => false,
            Self::V3_5Large | Self::V3_5LargeTurbo | Self::V3_5Medium => true,
        }
    }
}

struct Args {
    /// The prompt to be used for image generation.
    prompt: String,
    uncond_prompt: String,
    cpu: bool,
    /// Use flash_attn to accelerate attention operation in the MMDiT.
    use_flash_attn: bool,
    /// The height in pixels of the generated image.
    height: usize,
    /// The width in pixels of the generated image.
    width: usize,
    /// The model to use.
    which: StableDiffusionWhich,
    /// The seed to use when generating random samples.
    num_inference_steps: Option<usize>,
    /// CFG scale.
    cfg_scale: Option<f64>,
    /// Time shift factor (alpha).
    time_shift: f64,
    /// Use Skip Layer Guidance (SLG) for the sampling.
    /// Currently only supports Stable Diffusion 3.5 Medium.
    use_slg: bool,
    /// The seed to use when generating random samples.
    seed: Option<u64>,
}

fn stable_diffusion_internal(
    filename: String,
    extension: String,
    args: Args,
) -> Result<(), Box<dyn Error>> {
    // use tracing_chrome::ChromeLayerBuilder;
    // use tracing_subscriber::prelude::*;

    let Args {
        prompt,
        uncond_prompt,
        cpu,
        use_flash_attn,
        height,
        width,
        num_inference_steps,
        cfg_scale,
        time_shift,
        seed,
        which,
        use_slg,
    } = args;

    let device = get_device(false)?;
    println!("stable diffusion device {:?}", device);

    let default_inference_steps = match which {
        StableDiffusionWhich::V3_5Large => 28,
        StableDiffusionWhich::V3_5LargeTurbo => 4,
        StableDiffusionWhich::V3_5Medium => 28,
        StableDiffusionWhich::V3Medium => 28,
    };

    let num_inference_steps = num_inference_steps.unwrap_or(default_inference_steps);
    let default_cfg_scale = match which {
        StableDiffusionWhich::V3_5Large => 4.0,
        StableDiffusionWhich::V3_5LargeTurbo => 1.0,
        StableDiffusionWhich::V3_5Medium => 4.0,
        StableDiffusionWhich::V3Medium => 4.0,
    };
    let cfg_scale = cfg_scale.unwrap_or(default_cfg_scale);

    let api = candle_hf_hub::api::sync::Api::new()?;
    let (mmdit_config, mut triple, vb) = if which.is_3_5() {
        let sai_repo_for_text_encoders = {
            let name = match which {
                StableDiffusionWhich::V3_5Large => "stabilityai/stable-diffusion-3.5-large",
                StableDiffusionWhich::V3_5LargeTurbo => {
                    "stabilityai/stable-diffusion-3.5-large-turbo"
                }

                // Unfortunately, stabilityai/tool_stable_diffusion-3.5-medium doesn't have the monolithic text encoders that's usually
                // placed under the text_encoders directory, like the case in stabilityai/tool_stable_diffusion-3.5-large and -large-turbo.
                // To make things worse, it currently only has partitioned model.fp16-00001-of-00002.safetensors and model.fp16-00002-of-00002.safetensors
                // under the text_encoder_3 directory, for the t5xxl_fp16.safetensors model. This means that we need to merge the two partitions
                // to get the monolithic text encoders. This is not a trivial task.
                // Since the situation can change, we do not want to spend efforts to handle the uniqueness of stabilityai/tool_stable_diffusion-3.5-medium,
                // which involves different paths and merging the two partitions files for t5xxl_fp16.safetensors.
                // so for now, we'll use the text encoder models from the stabilityai/tool_stable_diffusion-3.5-large repository.
                // TODO: Change to "stabilityai/tool_stable_diffusion-3.5-medium" once the maintainers of the repository add back the monolithic text encoders.
                StableDiffusionWhich::V3_5Medium => "stabilityai/stable-diffusion-3.5-large",
                StableDiffusionWhich::V3Medium => unreachable!(),
            };
            api.repo(candle_hf_hub::Repo::model(name.to_string()))
        };
        let sai_repo_for_mmdit = {
            let name = match which {
                StableDiffusionWhich::V3_5Large => "stabilityai/stable-diffusion-3.5-large",
                StableDiffusionWhich::V3_5LargeTurbo => {
                    "stabilityai/stable-diffusion-3.5-large-turbo"
                }
                StableDiffusionWhich::V3_5Medium => "stabilityai/stable-diffusion-3.5-medium",
                StableDiffusionWhich::V3Medium => unreachable!(),
            };
            api.repo(candle_hf_hub::Repo::model(name.to_string()))
        };
        let clip_g_file = sai_repo_for_text_encoders.get("text_encoders/clip_g.safetensors")?;
        let clip_l_file = sai_repo_for_text_encoders.get("text_encoders/clip_l.safetensors")?;
        let t5xxl_file = sai_repo_for_text_encoders.get("text_encoders/t5xxl_fp16.safetensors")?;
        let model_file = {
            let model_file = match which {
                StableDiffusionWhich::V3_5Large => "sd3.5_large.safetensors",
                StableDiffusionWhich::V3_5LargeTurbo => "sd3.5_large_turbo.safetensors",
                StableDiffusionWhich::V3_5Medium => "sd3.5_medium.safetensors",
                StableDiffusionWhich::V3Medium => unreachable!(),
            };
            sai_repo_for_mmdit.get(model_file)?
        };
        let triple = StableDiffusion3TripleClipWithTokenizer::new_split(
            &clip_g_file,
            &clip_l_file,
            &t5xxl_file,
            &device,
        )?;
        let vb = unsafe {
            candle_nn::VarBuilder::from_mmaped_safetensors(&[model_file], DType::F16, &device)?
        };
        match which {
            StableDiffusionWhich::V3_5Large => (MMDiTConfig::sd3_5_large(), triple, vb),
            StableDiffusionWhich::V3_5LargeTurbo => (MMDiTConfig::sd3_5_large(), triple, vb),
            StableDiffusionWhich::V3_5Medium => (MMDiTConfig::sd3_5_medium(), triple, vb),
            StableDiffusionWhich::V3Medium => unreachable!(),
        }
    } else {
        let sai_repo = {
            let name = "stabilityai/stable-diffusion-3-medium";
            api.repo(candle_hf_hub::Repo::model(name.to_string()))
        };
        let model_file = sai_repo.get("sd3_medium_incl_clips_t5xxlfp16.safetensors")?;
        let vb = unsafe {
            candle_nn::VarBuilder::from_mmaped_safetensors(&[&model_file], DType::F16, &device)?
        };
        let triple = StableDiffusion3TripleClipWithTokenizer::new(vb.pp("text_encoders"))?;
        (MMDiTConfig::sd3_medium(), triple, vb)
    };
    let (context, y) = triple.encode_text_to_embedding(prompt.as_str(), &device)?;
    let (context_uncond, y_uncond) =
        triple.encode_text_to_embedding(uncond_prompt.as_str(), &device)?;
    // Drop the text model early to avoid using too much memory.
    drop(triple);
    let context = Tensor::cat(&[context, context_uncond], 0)?;
    let y = Tensor::cat(&[y, y_uncond], 0)?;

    // candle 0.8.2 -> can't set the seed when using CPU (anyhow::bail)
    if !cpu {
        if let Some(seed) = seed {
            device.set_seed(seed)?;
        }
    }

    let slg_config = if use_slg {
        match which {
            // https://github.com/Stability-AI/sd3.5/blob/4e484e05308d83fb77ae6f680028e6c313f9da54/sd3_infer.py#L388-L394
            StableDiffusionWhich::V3_5Medium => Some(sampling::SkipLayerGuidanceConfig {
                scale: 2.5,
                start: 0.01,
                end: 0.2,
                layers: vec![7, 8, 9],
            }),
            _ => {
                let msg = "--use-slg can only be used with 3.5-medium".to_string();
                return *Box::new(Err(msg.into()));
            }
        }
    } else {
        None
    };

    let start_time = std::time::Instant::now();
    let x = {
        let mmdit = MMDiT::new(
            &mmdit_config,
            use_flash_attn,
            vb.pp("model.diffusion_model"),
        )?;
        sampling::euler_sample(
            &mmdit,
            &y,
            &context,
            num_inference_steps,
            cfg_scale,
            time_shift,
            height,
            width,
            slg_config,
        )?
    };
    let dt = start_time.elapsed().as_secs_f32();
    println!(
        "Sampling done. {num_inference_steps} steps. {:.2}s. Average rate: {:.2} iter/s",
        dt,
        num_inference_steps as f32 / dt
    );

    let img = {
        let vb_vae = vb.rename_f(sd3_vae_vb_rename).pp("first_stage_model");
        let autoencoder = build_sd3_vae_autoencoder(vb_vae)?;

        // Apply TAESD3 scale factor. Seems to be significantly improving the quality of the image.
        // https://github.com/comfyanonymous/ComfyUI/blob/3c60ecd7a83da43d694e26a77ca6b93106891251/nodes.py#L721-L723
        autoencoder.decode(&((x / 1.5305)? + 0.0609)?)?
    };
    let img = ((img.clamp(-1f32, 1f32)? + 1.0)? * 127.5)?.to_dtype(DType::U8)?;
    let filename = format!("{}.{}", filename, extension);
    let res = save_image(&img.i(0)?, &filename);
    match res {
        Ok(_) => {
            println!("Saved image to {}", &filename);
            Ok(())
        }
        Err(e) => {
            let msg = format!("error saving image: {}", e);
            *Box::new(Err(msg.into()))
        }
    }
}

pub fn stable_diffusion(
    prompt: String,
    filename: String,
    extension: String,
    which: StableDiffusionWhich,
) -> Result<(), Box<dyn Error>> {
    let args = Args {
        prompt,
        uncond_prompt: "".to_string(),
        cpu: true,
        use_flash_attn: false,
        height: 512,
        width: 512,
        num_inference_steps: None,
        cfg_scale: None,
        time_shift: 3.0,
        seed: Some(23),
        which,
        use_slg: false,
    };

    stable_diffusion_internal(filename, extension, args)
}
