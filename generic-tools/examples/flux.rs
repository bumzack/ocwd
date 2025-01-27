use generic_tools::tool_flux::flux::{run_flux, WhichFlux};
use rand::Rng;
use std::error::Error;
use std::time::SystemTime;
use tokio::time::Instant;
// prompt taken from: https://huggingface.co/docs/diffusers/using-diffusers/kandinsky?text-to-image=Kandinsky+3

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Error: Metal error Metal seed must be less than or equal to u32::MAX
    let seed: u64 = rand::rng().random_range(0..u32::MAX - 10) as u64;


    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let filename = format!("flux_schnell_{}", ts);

    let start = Instant::now();
    run_flux(
        "A alien cheeseburger creature eating itself, claymation, cinematic, moody lighting"
            .to_string(),
        true,
        1280,
        720,
        "./".to_string(),
        filename,
        WhichFlux::Schnell,
        seed,
    )?;
    let duration = start.elapsed();

    println!("flux_schnell finished after {}secs", duration.as_secs());

    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let filename = format!("flux_dev_{}", ts);

    let start = Instant::now();
    let prompt =
        "A alien cheeseburger creature eating itself, claymation, cinematic, moody lighting"
            .to_string();

    let seed: u64 = rand::rng().random_range(0..u32::MAX - 10) as u64;
    run_flux(
        prompt,
        true,
        1280,
        720,
        "./".to_string(),
        filename,
        WhichFlux::Dev,
        seed,
    )?;
    let duration = start.elapsed();

    println!("flux_dev finished after {}secs", duration.as_secs());

    Ok(())
}
