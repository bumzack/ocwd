use generic_tools::tool_parler::parler::{run_parler, ParlerArgs, WhichParler};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let prompt = "Hey, how are you doing today?".to_string();
    let description =  "A female speaker delivers a slightly expressive and animated speech with a moderate speed and pitch. The recording is of very high quality, with the speaker's voice sounding clear and very close up.".to_string();
    let sample_len = 5000;
    let repeat_penalty = 1.0;
    let repeat_last_n = 4;
    let max_steps = 512;
    let out_file = "out.wav".to_string();

    let args = ParlerArgs {
        verbose_prompt: false,
        prompt,
        description,
        temperature: 0.0,
        top_p: None,
        seed: 0,
        sample_len,
        repeat_penalty,
        repeat_last_n,
        model_id: None,
        revision: None,
        quantized: false,
        f16: false,
        model_file: None,
        tokenizer_file: None,
        config_file: None,
        max_steps,
        out_file,
        which: WhichParler::LargeV1,
    };

    let p = run_parler(args);
    match p {
        Ok(_) => {
            println!("successfully created wav file")
        }
        Err(e) => {
            println!("error creating wav file: {:?}", e)
        }
    }
    Ok(())
}
