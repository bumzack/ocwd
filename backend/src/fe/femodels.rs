use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeOllamaModel {
    pub name: String,
    pub model: String,
    pub size: i64,
    pub detail_format: String,
    pub detail_family: String,
    pub detail_parameter_size: String,
    pub detail_quantization_level: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeDbOllamaModel {
    pub id: i32,
    pub name: String,
    pub model: String,
    pub size: i64,
    pub detail_format: String,
    pub detail_family: String,
    pub detail_parameter_size: String,
    pub detail_quantization_level: String,
    pub created: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeOllamaRunningModel {
    pub name: String,
    pub model: String,
    pub size: i64,
    pub format: String,
    pub family: String,
    pub parameter_size: String,
    pub quantization_level: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeRunModel {
    pub model_id: i32,
    pub temperature: f64,
    pub num_ctx: i64,
    pub seed: i64,
    pub top_k: f64,
    pub top_p: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeRunModelRequest {
    pub prompt: String,
    pub models: Vec<FeRunModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeRunModelResponse {
    pub response_id: i32,
    pub response: String,
    pub model_name: String,
    pub duration: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeOllamaChatQueueResponse {
    pub id: i32,
    pub model_id: i32,
    pub prompt_id: i32,
    pub state: String,
    pub created: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeOllamaPrompt {
    pub id: i32,
    pub prompt: String,
    pub created: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeOllamaChat {
    pub id: i32,
    pub model_id: i32,
    pub prompt_id: i32,
    pub model_name: String,
    pub prompt: String,
    pub model_size: String,
    pub response: String,
    pub result: String,
    pub seed: i64,
    pub num_ctx: i64,
    pub temperature: f64,
    pub top_k: f64,
    pub top_p: f64,
    pub duration_ms: i64,
    pub created: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeOllamaChatQueue {
    pub id: i32,
    pub model_id: i32,
    pub prompt_id: i32,
    pub state: String,
    pub temperature: f64,
    pub seed: i64,
    pub num_ctx: i64,
    pub top_k: f64,
    pub top_p: f64,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeUpdateOllamaChatResult {
    pub chat_id: i32,
    pub result: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertModelsResponse {
    pub model: String,
    pub name: String,
    pub model_id: Option<i32>,
    pub result: String,
}

