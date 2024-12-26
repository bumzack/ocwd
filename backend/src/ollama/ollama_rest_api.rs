use crate::ollama::ollama_rest_api_models::{
    OllamaModel, OllamaModelResponse, OllamaRequest, OllamaResponse, OllamaUnloadRequest,
};
use crate::server::ollamachat_error::OllamaChatError;
use crate::CONFIG;
use reqwest::Client;
use serde_json::json;
use std::time::Instant;
use tracing::{error, info};

pub async fn get_all_local_models() -> Result<Vec<OllamaModel>, OllamaChatError> {
    let url = CONFIG.ollama_url.clone();
    let url = format!("{}/api/tags", url);

    let res = get_client()?
        .get(url)
        .send()
        .await
        .map_err(OllamaChatError::from)
        .map_err(|e| OllamaChatError::from(e))?;

    let body = res.text().await.map_err(|e| OllamaChatError::from(e))?;

    let models =
        serde_json::from_str::<OllamaModelResponse>(&body).map_err(|e| OllamaChatError::from(e))?;

    Ok(models.models)
}

pub async fn get_loaded_models() -> Result<Vec<OllamaModel>, OllamaChatError> {
    let url = CONFIG.ollama_url.clone();
    let url = format!("{}/api/ps", url);

    let res = get_client()?
        .get(url)
        .send()
        .await
        .map_err(|e| {
            error!("error1 getting running models {:?}", e);
            OllamaChatError::from(e)
        })
        .map_err(|e| {
            error!("error2 getting running models {:?}", e);
            OllamaChatError::from(e)
        })?;

    let body = res
        .text()
        .await
        .map_err(|e| {
            error!("error3 getting running models {:?}", e);
            OllamaChatError::from(e)
        })
        .map_err(|e| {
            error!("error4 getting running models {:?}", e);
            OllamaChatError::from(e)
        })?;

    let models =
        serde_json::from_str::<OllamaModelResponse>(&body).map_err(|e| OllamaChatError::from(e))?;

    Ok(models.models)
}

pub async fn execute_ollama_chat(req: OllamaRequest) -> Result<OllamaResponse, OllamaChatError> {
    let ollama_url = CONFIG.ollama_url.clone();
    let url = format!("{}/api/generate", ollama_url);
    let start = Instant::now();
    let json = json!(req);

    let res = get_client()?
        .post(url)
        .body(json.to_string())
        .send()
        .await
        .map_err(|e| OllamaChatError::from(e))?;

    if res.status().is_success() {
        let body = res.text().await.map_err(|e| OllamaChatError::from(e))?;

        let duration = start.elapsed().as_millis();
        info!("model {} took {}ms -> \n{}\n", req.model, duration, body);
        let res =
            serde_json::from_str::<OllamaResponse>(&body).map_err(|e| OllamaChatError::from(e))?;
        info!("response {}", res.response);
        return Ok(res);
    }

    let err = res.bytes().await?;
    error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
    error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
    error!(
        "some sort of error occurred while executing the request. model '{}', prompt: '{}', \nresponse body \n{:?}\n",
        req.model, req.prompt, err
    );
    error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
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
    let res = get_client()?
        .post(url)
        .body(json.to_string())
        .send()
        .await
        .map_err(OllamaChatError::from)?;

    if res.status().is_success() {
        let _ = res.text().await.map_err(|e| OllamaChatError::from(e))?;
        let duration = start.elapsed().as_millis();
        info!("unload model {} took {}ms", req.model, duration);
        // let res = serde_json::from_str::<OllamaUnloadResponse>(&body).unwrap();
        // let res_pretty = serde_json::to_string_pretty(&res).unwrap();
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

fn get_client() -> reqwest::Result<Client> {
    Client::builder()
        .tcp_keepalive(Some(std::time::Duration::from_secs(3000)))
        .build()
}
