use crate::ollama::ollama_rest_api_models::{
    OllamaModel, OllamaModelResponse, OllamaRequest, OllamaResponse, OllamaUnloadRequest,
    OllamaUnloadResponse,
};
use crate::server::ollamachat_error::OllamaChatError;
use crate::CONFIG;
use serde_json::json;
use std::time::Instant;
use tracing::{error, info};

pub async fn get_all_local_models() -> Result<Vec<OllamaModel>, OllamaChatError> {
    let client = reqwest::Client::new();

    let url = CONFIG.ollama_url.clone();
    let url = format!("{}/api/tags", url);

    let res = client
        .get(url)
        .send()
        .await
        .map_err(OllamaChatError::from)
        .expect("should be there");

    let body = res.text().await.expect("Couldn't read body");

    info!("{}", body);

    let models = serde_json::from_str::<OllamaModelResponse>(&body).unwrap();
    let s = serde_json::to_string_pretty(&models).unwrap();
    info!("{}", s);

    let models = models.models;

    Ok(models)
}

pub async fn execute_ollama_chat(req: OllamaRequest) -> Result<OllamaResponse, OllamaChatError> {
    let ollama_url = CONFIG.ollama_url.clone();
    let url = format!("{}/api/generate", ollama_url);

    let start = Instant::now();

    let json = json!(req);
    let client = reqwest::Client::builder()
        .tcp_keepalive(Some(std::time::Duration::from_secs(3000)))
        .build()?;

    let res = client
        .post(url)
        .body(json.to_string())
        .send()
        .await
        .expect("Request failed");

    if res.status().is_success() {
        let body = res.text().await.expect("Couldn't read body");
        let duration = start.elapsed().as_millis();
        info!("model {} took {}ms -> \n{}\n", req.model, duration, body);
        let res = serde_json::from_str::<OllamaResponse>(&body).unwrap();
        info!("response {}", res.response);
        return Ok(res);
    }

    let err = res.bytes().await?;
    error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
    error!(
        "some sort of error occurred while executing the request. model '{}', prompt: '{}', \nresponse body \n{:?}\n",
        req.model, req.prompt, err
    );
    error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
    let msg = format!("reqwest returned an error. response body:  {:?}", err);
    Err(OllamaChatError::DataError(msg))
}

pub async fn execute_ollama_unload(model_name: String) -> Result<(), OllamaChatError> {
    let ollama_url = CONFIG.ollama_url.clone();
    let url = format!("{}/api/generate", ollama_url);

    let start = Instant::now();

    let req = OllamaUnloadRequest {
        model: model_name,
        keep_alive: 0,
    };

    let json = json!(req);

    let client = reqwest::Client::builder()
        .tcp_keepalive(Some(std::time::Duration::from_secs(3000)))
        .build()?;

    let res = client.post(url).body(json.to_string()).send().await;

    let res = match res {
        Ok(r) => {
            info!("ollama request successful");
            r
        }
        Err(e) => {
            error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
            error!("error sending request {:?}", e);
            error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
            return Err(OllamaChatError::from(e));
        }
    };

    if res.status().is_success() {
        let body = res.text().await.expect("Couldn't read body");
        let duration = start.elapsed().as_millis();
        info!("unload model {} took {}ms", req.model, duration,);
        let res = serde_json::from_str::<OllamaUnloadResponse>(&body).unwrap();
        let res_pretty = serde_json::to_string_pretty(&res).unwrap();
        // info!("unload response {}", res_pretty);
        return Ok(());
    }

    let err = res.bytes().await?;
    error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
    error!(
        "some sort of error occurred while executing the request. model '{}'\nresponse body \n{:?}\n",
        req.model,  err
    );
    error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
    let msg = format!("reqwest returned an error. response body:  {:?}", err);
    Err(OllamaChatError::DataError(msg))
}
