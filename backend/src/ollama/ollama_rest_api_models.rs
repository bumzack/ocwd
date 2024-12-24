use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OllamaDetails {
    pub parent_model: String,
    pub format: String,
    pub family: String,
    pub families: Vec<String>,
    pub parameter_size: String,
    pub quantization_level: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OllamaModel {
    pub name: String,
    pub modified_at: String,
    pub model: String,
    pub size: i64,
    pub digest: String,
    pub expires_at: Option<String>,
    pub size_vram: Option<i64>,
    pub details: OllamaDetails,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OllamaModelResponse {
    pub models: Vec<OllamaModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OllamaOptions {
    pub temperature: Option<f64>,
    pub num_ctx: Option<i64>,
    pub seed: Option<i64>,
    pub top_k: Option<f64>,
    pub top_p: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    pub options: OllamaOptions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OllamaResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
    pub done_reason: String,
    pub context: Vec<i64>,
    pub total_duration: i64,
    pub load_duration: i64,
    pub prompt_eval_count: i64,
    pub prompt_eval_duration: i64,
    pub eval_count: i64,
    pub eval_duration: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertModelsResponse {
    pub model: String,
    pub name: String,
    pub model_id: Option<i32>,
    pub result: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OllamaUnloadRequest {
    pub model: String,
    pub keep_alive: i32,
}

#[derive(Serialize, Deserialize)]
pub struct OllamaUnloadResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
    pub done_reason: String,
}
