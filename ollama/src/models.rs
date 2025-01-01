use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Ollama {
    pub(crate) url: String,
    pub(crate) client: Client,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    pub function: ToolCallFunction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub images: Option<Vec<String>>,
    pub tool_calls: Vec<ToolCall>,
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
    pub prompt: String,
    pub stream: bool,
    pub options: Option<ChatRequestOptions>,
    pub messages: Option<Vec<Message>>,
    pub format: Option<Format>,
    pub tools: Option<Vec<Tool>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
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
    pub parent_model: String,
    pub format: String,
    pub family: String,
    pub families: Option<Vec<String>>,
    pub parameter_size: String,
    pub quantization_level: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListModel {
    pub name: String,
    pub model: String,
    pub modified_at: String,
    pub size: i32,
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
    pub size: i32,
    pub digest: String,
    pub details: ModelDetails,
    pub expires_at: String,
    pub size_vram: String,
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
   // Integer(i64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OllamaInformation {
    pub modelfile: String,
    pub parameters: String,
    pub template: String,
    pub details: ModelDetails,
    pub model_info: Option<HashMap<String, Option<StringOrNumber>>>,
    pub license: String,
    pub modified_at: String,
}


#[cfg(test)]
mod test {
    use crate::error::OllamaError;
    use crate::models::OllamaInformation;

    #[test]
    fn test() {
        let json = r#"{
  "model_info": {
    "general.architecture": "qwen2",
    "general.file_type": 2,
    "general.parameter_count": 619570176,
    "general.quantization_version": 2,
    "qwen2.attention.head_count": 16,
    "qwen2.attention.head_count_kv": 16,
    "qwen2.attention.layer_norm_rms_epsilon": 0.000001,
    "qwen2.block_count": 24,
    "qwen2.context_length": 32768,
    "qwen2.embedding_length": 1024,
    "qwen2.feed_forward_length": 2816,
    "qwen2.use_parallel_residual": true,
    "tokenizer.ggml.bos_token_id": 151643,
    "tokenizer.ggml.eos_token_id": 151643,
    "tokenizer.ggml.merges": null,
    "tokenizer.ggml.model": "gpt2",
    "tokenizer.ggml.padding_token_id": 151643,
    "tokenizer.ggml.token_type": null,
    "tokenizer.ggml.tokens": null
  },
  "modified_at": "2024-12-30T00:22:03.600174684+01:00"
}"#;

        let res = serde_json::from_str::<OllamaInformation>(&json).map_err(OllamaError::from).expect("fff");

        println!("{:?}", res);
    }
}