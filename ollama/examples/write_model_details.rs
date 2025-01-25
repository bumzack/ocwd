use ollama::api::OllamaImpl;
use ollama::error::OllamaError;
use ollama::models::{Ollama, OllamaInformation};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<(), OllamaError> {
    let o = Ollama::new("http://10.0.0.48:11435".to_string()).expect("Couldn't open old sdk SDK");

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let mut local_models = o.local_models().await?;
    local_models.sort_by(|a, b| a.model.cmp(&b.model));

    let mut res = BTreeMap::new();

    for local_model in local_models.iter() {
        let details = o.details(&local_model.model).await?;
        res.insert(local_model.model.clone(), details);
    }

    let x = ModelDetails { details: res };
    let details = serde_json::to_string_pretty(&x).expect("couldn't serialize details");

    let filename = format!(
        "{}_{}.json",
        "model_details",
        format!("{:?}", since_the_epoch)
    );
    fs::write(filename, &details).expect("Unable to write file");
    Ok(())
}

#[derive(Serialize)]
struct ModelDetails {
    details: BTreeMap<String, OllamaInformation>,
}
