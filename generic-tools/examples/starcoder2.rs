use generic_tools::tool_starcoder2::starcoder2::starcoder2;
use std::error::Error;
use std::time::SystemTime;
use tokio::time::Instant;
// prompt taken from: https://huggingface.co/docs/diffusers/using-diffusers/kandinsky?text-to-image=Kandinsky+3

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let prompt = "Write a rust server using axum that is serving product data for a webshop.";
    let start = Instant::now();
    starcoder2(prompt.to_string()).expect("starcoder should work");
    let duration = start.elapsed();

    println!("starcoder2 finished after {}secs", duration.as_secs());

    Ok(())
}
