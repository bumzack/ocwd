use generic_tools::tool_stable_diffusion::stablediff::{stable_diffusion, StableDiffusionWhich};
use std::error::Error;
use std::time::SystemTime;
use tokio::time::Instant;

// prompt taken from: https://huggingface.co/docs/diffusers/using-diffusers/kandinsky?text-to-image=Kandinsky+3

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let filename = format!("stable_diffusion_alien_cheeseburger_{}", ts);

    let start = Instant::now();
    stable_diffusion(
        "A alien cheeseburger creature eating itself, claymation, cinematic, moody lighting"
            .to_string(),
        filename,
        "png".to_string(),
        StableDiffusionWhich::V3_5Large,
    )?;
    let duration = start.elapsed();

    println!("stable_diffusion finished after {}secs", duration.as_secs());

    Ok(())
}
