#[cfg(feature = "accelerate")]
extern crate accelerate_src;
#[cfg(feature = "mkl")]
extern crate intel_mkl_src;

use crate::candle_tools::candletools::{get_device, hub_load_safetensors, normalize_loudness};
use crate::candle_tools::wav;
use anyhow::Error as E;
use candle_core::{DType, IndexOp, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::parler_tts::{Config, Model};
use tokenizers::Tokenizer;

#[derive(Debug)]
pub struct ParlerArgs {
    /// Display the token for the specified prompt.
    pub verbose_prompt: bool,
    pub prompt: String,
    pub description: String,
    /// The temperature used to generate samples.
    pub temperature: f64,
    /// Nucleus sampling probability cutoff.
    pub top_p: Option<f64>,
    /// The seed to use when generating random samples.
    pub seed: u64,
    pub sample_len: usize,
    /// Penalty to be applied for repeating tokens, 1. means no penalty.
    pub repeat_last_n: usize,
    pub model_id: Option<String>,
    pub revision: Option<String>,
    pub repeat_penalty: f32,
    /// The context size to consider for the repeat penalty.
    pub quantized: bool,
    /// Use f16 precision for all the computations rather than f32.
    pub f16: bool,
    pub model_file: Option<String>,
    pub tokenizer_file: Option<String>,
    pub config_file: Option<String>,
    pub max_steps: usize,
    /// The output wav file.
    pub out_file: String,
    pub which: WhichParler,
}

#[derive(Debug)]
pub enum WhichParler {
    LargeV1,
    MiniV1,
}

pub fn run_parler(args: ParlerArgs) -> anyhow::Result<()> {
    println!("arg {:?}", args);
    println!(
        "avx: {}, neon: {}, simd128: {}, f16c: {}",
        candle_core::utils::with_avx(),
        candle_core::utils::with_neon(),
        candle_core::utils::with_simd128(),
        candle_core::utils::with_f16c()
    );
    println!(
        "temp: {:.2} repeat-penalty: {:.2} repeat-last-n: {}",
        args.temperature, args.repeat_penalty, args.repeat_last_n
    );

    let start = std::time::Instant::now();
    let api = candle_hf_hub::api::sync::Api::new()?;
    let model_id = match args.model_id {
        Some(model_id) => model_id.to_string(),
        None => match args.which {
            WhichParler::LargeV1 => "parler-tts/parler-tts-large-v1".to_string(),
            WhichParler::MiniV1 => "parler-tts/parler-tts-mini-v1".to_string(),
        },
    };
    let revision = match args.revision {
        Some(r) => r,
        None => "main".to_string(),
    };
    let repo = api.repo(candle_hf_hub::Repo::with_revision(
        model_id,
        candle_hf_hub::RepoType::Model,
        revision,
    ));
    let model_files = match args.model_file {
        Some(m) => vec![m.into()],
        None => match args.which {
            WhichParler::MiniV1 => vec![repo.get("model.safetensors")?],
            WhichParler::LargeV1 => hub_load_safetensors(&repo, "model.safetensors.index.json")?,
        },
    };
    let config = match args.config_file {
        Some(m) => m.into(),
        None => repo.get("config.json")?,
    };
    let tokenizer = match args.tokenizer_file {
        Some(m) => m.into(),
        None => repo.get("tokenizer.json")?,
    };
    println!("retrieved the files in {:?}", start.elapsed());
    let tokenizer = Tokenizer::from_file(tokenizer).map_err(E::msg)?;

    let start = std::time::Instant::now();
    let device = get_device(false)?;
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&model_files, DType::F32, &device)? };
    let config: Config = serde_json::from_reader(std::fs::File::open(config)?)?;
    let mut model = Model::new(&config, vb)?;
    println!("loaded the model in {:?}", start.elapsed());

    let description_tokens = tokenizer
        .encode(args.description, true)
        .map_err(E::msg)?
        .get_ids()
        .to_vec();
    let description_tokens = Tensor::new(description_tokens, &device)?.unsqueeze(0)?;
    let prompt_tokens = tokenizer
        .encode(args.prompt, true)
        .map_err(E::msg)?
        .get_ids()
        .to_vec();
    let prompt_tokens = Tensor::new(prompt_tokens, &device)?.unsqueeze(0)?;
    let lp = candle_transformers::generation::LogitsProcessor::new(
        args.seed,
        Some(args.temperature),
        args.top_p,
    );
    println!("starting generation...");
    let codes = model.generate(&prompt_tokens, &description_tokens, lp, args.max_steps)?;
    println!("generated codes\n{codes}");
    let codes = codes.to_dtype(DType::I64)?;
    codes.save_safetensors("codes", "out.safetensors")?;
    let codes = codes.unsqueeze(0)?;
    let pcm = model
        .audio_encoder
        .decode_codes(&codes.to_device(&device)?)?;
    println!("{pcm}");
    let pcm = pcm.i((0, 0))?;
    let pcm = normalize_loudness(&pcm, 24_000, true)?;
    let pcm = pcm.to_vec1::<f32>()?;
    let mut output = std::fs::File::create(&args.out_file)?;
    wav::write_pcm_as_wav(&mut output, &pcm, config.audio_encoder.sampling_rate)?;

    Ok(())
}
