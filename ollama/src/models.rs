use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Ollama {
    pub(crate) url: String,
    pub(crate) client: Client,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    pub function: ToolCallFunction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Content {
    pub status: Option<String>,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ContentEnum {
    AString(String),
    AContent(Content),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: Option<ContentEnum>,
    pub images: Option<Vec<String>>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub tool_call_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ChatRequestOptions {
    pub num_keep: Option<u32>,
    pub seed: Option<u32>,
    pub num_predict: Option<u32>,
    pub top_k: Option<f32>,
    pub top_p: Option<f32>,
    pub min_p: Option<f32>,
    pub typical_p: Option<f32>,
    pub repeat_last_n: Option<u32>,
    pub temperature: Option<f32>,
    pub repeat_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub mirostat: Option<u32>,
    pub mirostat_tau: Option<f32>,
    pub mirostat_eta: Option<f32>,
    pub penalize_newline: Option<bool>,
    pub stop: Option<Vec<String>>,
    pub numa: Option<bool>,
    pub num_ctx: Option<u32>,
    pub num_batch: Option<u32>,
    pub num_gpu: Option<u32>,
    pub main_gpu: Option<u32>,
    pub low_vram: Option<bool>,
    pub vocab_only: Option<bool>,
    pub use_mmap: Option<bool>,
    pub use_mlock: Option<bool>,
    pub num_thread: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Property {
    #[serde(rename(serialize = "type"))]
    pub typ: String,
    pub description: String,
    #[serde(rename(serialize = "enum"))]
    pub enums: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parameter {
    #[serde(rename(serialize = "type"))]
    pub typ: String,
    pub properties: HashMap<String, Property>,
    pub required: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: Parameter,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Format {
    #[serde(rename(serialize = "type"))]
    pub typ: String,
    pub function: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tool {
    #[serde(rename(serialize = "type"))]
    pub typ: String,
    pub function: Function,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatRequest {
    pub model: String,
    pub prompt: Option<String>,
    pub stream: bool,
    pub options: Option<ChatRequestOptions>,
    pub messages: Option<Vec<Message>>,
    pub format: Option<String>,
    pub tools: Option<Vec<Tool>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatResponse {
    pub model: String,
    pub created_at: String,
    pub response: Option<String>,
    pub done: bool,
    pub context: Option<Vec<u32>>,
    pub total_duration: Option<i64>,
    pub load_duration: Option<i64>,
    pub prompt_eval_count: Option<i64>,
    pub prompt_eval_duration: Option<i64>,
    pub eval_count: Option<i64>,
    pub eval_duration: Option<i64>,
    pub done_reason: Option<String>,
    pub message: Option<Message>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateModelRequest {
    pub model: String,
    pub modelfile: String,
    pub quantize: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateModelResponse {
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelDetails {
    pub parent_model: Option<String>,
    pub format: Option<String>,
    pub family: Option<String>,
    pub families: Option<Vec<String>>,
    pub parameter_size: Option<String>,
    pub quantization_level: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListModel {
    pub name: String,
    pub model: String,
    pub modified_at: String,
    pub size: i64,
    pub digest: String,
    pub details: ModelDetails,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListModelResponse {
    pub models: Vec<ListModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunningModel {
    pub name: String,
    pub model: String,
    pub size: i64,
    pub digest: String,
    pub details: ModelDetails,
    pub expires_at: String,
    pub size_vram: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunningModelResponse {
    pub models: Vec<RunningModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OllamaUnloadRequest {
    pub model: String,
    pub keep_alive: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OllamaInformationRequest {
    pub model: String,
    pub verbose: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum StringOrNumber {
    String(String),
    Float(f64),
    Bool(bool),
    Integer(i64),
    ListVec(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OllamaInformation {
    pub modelfile: String,
    pub parameters: Option<String>,
    pub template: String,
    pub details: ModelDetails,
    pub model_info: Option<HashMap<String, Option<StringOrNumber>>>,
    pub license: Option<String>,
    pub modified_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OllamaErrorResponse {
    pub error: Option<String>,
}
