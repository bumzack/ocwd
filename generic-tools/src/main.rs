use crate::tool_postgres::postgres::{tokio_postgres, PostgresUtilRequest};
use crate::tool_stable_diffusion::stablediff::{stable_diffusion, StableDiffusionWhich};
use std::error::Error;
use tokio::time::Instant;

mod candle_tools;
mod tool_postgres;
mod tool_stable_diffusion;
mod tool_wuerstchen;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let query = "SELECT * from ollama_chat_queue".to_string();
    let req = PostgresUtilRequest { query };
    let values = tokio_postgres(&req).await;
    let value = match values {
        Ok(val) => format!("{:?}", val),
        Err(e) => {
            println!("error {:?}", e);
            return Err(Box::from(format!(
                "error getting data from database with error: {:?}",
                e
            )));
        }
    };

    // run_wuerstchen(
    //     "a dolphin crashing into a truck.".to_string(),
    //     "dolphin_wuerstchen".to_string(),
    // )?;

    println!("starting stable_diffusion");
    let start = Instant::now();
    stable_diffusion(
        "a dolphin crashing into a truck.".to_string(),
        "dolphin_stable_diffusion".to_string(),
        StableDiffusionWhich::V3_5Large,
    )?;
    let duration = start.elapsed();
    println!("finished stable_diffusion. duration {}secs", duration.as_secs());

    let json = serde_json::to_string_pretty(&value).unwrap();
    println!("finished. json \n{}\n", json);

    Ok(())
}
