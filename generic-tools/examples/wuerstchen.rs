use generic_tools::tool_wuerstchen::wuerstchen::run_wuerstchen;
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
    let filename = format!("wuerstchen_alien_cheeseburger_{}", ts);

    let start = Instant::now();
    run_wuerstchen(
        "A alien cheeseburger creature eating itself, claymation, cinematic, moody lighting"
            .to_string(),
        filename,
    )?;
    let duration = start.elapsed();

    println!("wuerstchen finished after {}secs", duration.as_secs());

    Ok(())
}
