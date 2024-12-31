use crate::error::OllamaError;
use crate::models::{
    ChatRequest, ChatResponse, ListModel, ListModelResponse, Ollama, OllamaUnloadRequest,
    RunningModel, RunningModelResponse,
};
use reqwest::{Client, Response};
use serde_json::json;
use std::time::Instant;

//  using async fn in a trait, which could be a problem for multithreaded use cases
// if problems arise, read again here: https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html
pub trait OllamaImpl {
    async fn local_models(&self) -> Result<Vec<ListModel>, OllamaError>;
    async fn loaded_models(&self) -> Result<Vec<RunningModel>, OllamaError>;
    async fn chat(&self, request: &ChatRequest) -> Result<ChatResponse, OllamaError>;
    async fn chat_streaming(&self, request: &ChatRequest) -> Result<Response, OllamaError>;
    async fn unload(&self, model: &str) -> Result<(), OllamaError>;
}

impl Ollama {
    pub fn new(url: String) -> Result<Ollama, OllamaError> {
        let client = get_client(3000)?;
        Ok(Ollama { client, url })
    }
}

impl OllamaImpl for Ollama {
    async fn local_models(&self) -> Result<Vec<ListModel>, OllamaError> {
        let url = format!("{}/api/tags", self.url);

        let res = self
            .client
            .get(url)
            .send()
            .await
            .map_err(OllamaError::from)
            .map_err(OllamaError::from)?;

        let body = res.text().await.map_err(OllamaError::from)?;

        let models = serde_json::from_str::<ListModelResponse>(&body).map_err(OllamaError::from)?;

        Ok(models.models)
    }

    async fn loaded_models(&self) -> Result<Vec<RunningModel>, OllamaError> {
        let url = format!("{}/api/ps", self.url);

        let res = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| {
                println!("println1 getting running models {:?}", e);
                e
            })
            .map_err(|e| {
                println!("println2 getting running models {:?}", e);
                e
            })?;

        let body = res
            .text()
            .await
            .map_err(|e| {
                println!("println3 getting running models {:?}", e);
                e
            })
            .map_err(|e| {
                println!("println4 getting running models {:?}", e);
                e
            })?;

        let models =
            serde_json::from_str::<RunningModelResponse>(&body).map_err(OllamaError::from)?;

        Ok(models.models)
    }

    async fn chat(&self, request: &ChatRequest) -> Result<ChatResponse, OllamaError> {
        let url = format!("{}/api/generate", self.url);
        let json = json!(request);

        let res = self.client.post(url).body(json.to_string()).send().await?;

        if !res.status().is_success() {
            let err = res.bytes().await?;
            let msg = format!("reqwest returned an error. response body:  {:?}", err);
            return Err(OllamaError::AllTheOtherErrors(msg));
        }

        let body = res.text().await.map_err(OllamaError::from)?;
        let res = serde_json::from_str::<ChatResponse>(&body).map_err(OllamaError::from)?;
        Ok(res)
    }

    async fn chat_streaming(&self, request: &ChatRequest) -> Result<Response, OllamaError> {
        let url = format!("{}/api/generate", self.url);
        let start = Instant::now();
        let json = json!(request);

        let res = self.client.post(url).body(json.to_string()).send().await?;

        let duration = start.elapsed();
        println!("request took {:?}", duration);

        Ok(res)
    }

    async fn unload(&self, model: &str) -> Result<(), OllamaError> {
        let url = format!("{}/api/generate", self.url);
        let start = Instant::now();

        let req = OllamaUnloadRequest {
            model: model.to_string(),
            keep_alive: 0,
        };

        let json = json!(req);
        let res = self
            .client
            .post(url)
            .body(json.to_string())
            .send()
            .await
            .map_err(OllamaError::from)?;

        if !res.status().is_success() {
            let err = res.bytes().await?;
            println!(
                "some sort of println occurred while executing the request. model '{}'\nresponse body \n{:?}\n",
                req.model, err
            );
            let msg = format!("reqwest returned an println. response body:  {:?}", err);
            return Err(OllamaError::AllTheOtherErrors(msg));
        }

        let _ = res.text().await.map_err(OllamaError::from)?;
        let duration = start.elapsed().as_millis();
        println!("unload model {} took {}ms", req.model, duration);
        Ok(())
    }
}

fn get_client(keep_alive: u64) -> reqwest::Result<Client> {
    Client::builder()
        .tcp_keepalive(Some(std::time::Duration::from_secs(keep_alive)))
        .build()
}
