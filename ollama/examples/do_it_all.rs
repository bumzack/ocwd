use futures::{StreamExt, TryStreamExt};
use ollama::api::OllamaImpl;
use ollama::error::OllamaError;
use ollama::models::{ChatRequest, Ollama};


#[tokio::main]
async fn main() -> Result<(), OllamaError> {
    let o = Ollama::new("http://localhost:11434".to_string()).expect("Couldn't open old sdk SDK");

    let local_models = o.local_models().await?;

    local_models
        .iter()
        .for_each(|lm| println!("local models: {:?}", lm));

    let loaded_models = o.local_models().await?;

    loaded_models
        .iter()
        .for_each(|lm| println!("loaded model: {:?}", lm));

    let model = local_models.first().expect("No model found");


    let request = ChatRequest {
        model: model.name.to_string(),
        prompt: "Write a fibonacci function in rust".to_string(),
        stream: true,
        options: None,
        messages: None,
        format: None,
    };

    let mut r = o.chat(&request).await?;

    while let Some(msg) = r.next().await {
        println!("next msg {:?}", msg);
    }

    if loaded_models.len() > 0 {
        let model = loaded_models.first().expect("no mdoel found");
        let model = &model.name;
        o.unload(model).await?;
    }

    Ok(())
}
