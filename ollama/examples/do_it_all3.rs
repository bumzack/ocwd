use ollama::api::OllamaImpl;
use ollama::error::OllamaError;
use ollama::models::{
    ChatRequest, ChatRequestOptions, ChatResponse, ContentEnum, ListModel, Message, Ollama,
};
use ollama::tools_v1::get_tools_v1;
use serde::Serialize;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<(), OllamaError> {
    let o = Ollama::new("http://10.0.0.48:11435".to_string()).expect("Couldn't open old sdk SDK");
    let take = 200;

    let mut local_models = o.local_models().await?;
    local_models.sort_by(|a, b| a.model.cmp(&b.model));

    println!("cnt models {}", local_models.len());

    let mut res = vec![];

    let prompts = vec![
        "Get me a list of all products in the shop.'".to_string(),
        "How many orders have been made in the Q1 and Q3 of 2022 compared to 2023 compared to 2024.'".to_string(),
        "What are the top 3 most sold articles.'".to_string(),
        "Which shop databases do you have access to and can me show some basic statistics about?'"
            .to_string(),
        "Create an image using the prompt 'A astronaut riding a dolphin on the moon.'".to_string(),
        "How is the weather in Vienna?".to_string(),
    ];

    let model_blacklist = vec![
        "codebooga:latest",
        "codellama:70b",
        "deepseek-v2.5:236b",
        "dolphin-mixtral:8x22b",
        "exaone3.5:32b",
        "falcon:180b",
        "gemma2:27b",
        "hengwen/Sky-T1-32B-Preview:q4_k_m",
        "hengwen/Sky-T1-32B-Preview:q8_0",
        "hengwen/watt-tool-70B:latest",
        "nemotron-mini:4b", // returns <tool_call> in message.content
        "olmo2:13b",
        "phi4:14b",
        "nous-hermes2:34b",
        "nous-hermes2-mixtral:8x7b",
        "notux:8x7b",
        "zephyr:141b",
        "tulu3:70b",
        "marco-o1:7b-fp16",
        "reflection:70b",
    ];

    let mut res_only = vec![];

    for local_model in local_models
        .iter()
        .filter(|local_model| !model_blacklist.contains(&local_model.model.as_str()))
        .take(take)
    {
        for prompt in prompts.iter().take(take) {
            println!(
                "START: model.name {}, prompt '{}'",
                local_model.name, prompt
            );
            let r = doit(&local_model, prompt, &o).await;
            if r.is_ok() {
                let test_result = r?;
                let duration = test_result.duration;
                res.push(test_result.clone());
                println!(
                    "END model.name {}, prompt '{}', duration {}",
                    local_model.name, prompt, duration
                );

                let x = test_result
                    .response
                    .as_ref()
                    .map(|cr| cr.message.clone())
                    .map(|m| {
                        m.map_or("no_msg".to_string(), |msg| match msg.content {
                            ContentEnum::AString(a) => a,
                            ContentEnum::AContent(content) => {
                                content.message.unwrap_or("no content messsage".to_string())
                            }
                        })
                    })
                    .unwrap_or("n/a".to_string());

                let message = test_result
                    .response
                    .as_ref()
                    .and_then(|cr| cr.message.as_ref().map(|msg| msg.clone()));

                let mini = TestResultMinimal {
                    prompt: test_result.prompt.clone(),
                    response_txt: x,
                    duration: test_result.duration,
                    error: test_result.error.clone(),
                    success: test_result.success,
                    message,
                };

                let pretty =
                    serde_json::to_string_pretty(&mini).expect("could not serialize result");
                println!("mini {}", pretty);

                res_only.push(mini);
            }
        }
    }

    let all = AllTestResult {
        all_test_result: res,
    };

    let all_res_only = AllTestResultMin {
        all_test_result_min: res_only,
    };

    // let pretty = serde_json::to_string_pretty(&all).expect("could not serialize result");
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let filename = format!("results_tools_v1_{}.json", since_the_epoch.as_millis());
    let results = serde_json::to_string(&all).expect("couldn't serialize results");
    fs::write(filename, &results).expect("Unable to write file");

    let filename = format!(
        "results_tools_v1_res_only_{}.json",
        since_the_epoch.as_millis()
    );
    let results = serde_json::to_string(&all_res_only).expect("couldn't serialize results");
    fs::write(filename, &results).expect("Unable to write file");

    Ok(())
}

#[derive(Serialize, Clone, Debug)]
struct TestResult {
    pub request: Option<ChatRequest>,
    pub response: Option<ChatResponse>,
    pub prompt: String,
    pub model: String,
    pub duration: u64,
    pub error: Option<String>,
    pub success: bool,
}

#[derive(Serialize, Clone, Debug)]
struct TestResultMinimal {
    pub prompt: String,
    pub response_txt: String,
    pub duration: u64,
    pub error: Option<String>,
    pub success: bool,
    pub message: Option<Message>,
}

#[derive(Serialize, Clone, Debug)]
struct AllTestResult {
    pub all_test_result: Vec<TestResult>,
}

#[derive(Serialize, Clone, Debug)]
struct AllTestResultMin {
    pub all_test_result_min: Vec<TestResultMinimal>,
}

async fn doit(
    local_model: &ListModel,
    prompt: &str,
    ollama: &Ollama,
) -> Result<TestResult, OllamaError> {
    let tools = get_tools_v1();

    let msg = Message {
        role: "user".to_string(),
        content: ContentEnum::AString(prompt.to_string()),
        images: None,
        tool_calls: None,
        tool_call_id: None,
    };

    let options = ChatRequestOptions {
        num_keep: None,
        seed: Some(23),
        num_predict: None,
        top_k: None,
        top_p: None,
        min_p: None,
        typical_p: None,
        repeat_last_n: None,
        temperature: Some(0.1),
        repeat_penalty: None,
        presence_penalty: None,
        frequency_penalty: None,
        mirostat: None,
        mirostat_tau: None,
        mirostat_eta: None,
        penalize_newline: None,
        stop: None,
        numa: None,
        num_ctx: None,
        num_batch: None,
        num_gpu: None,
        main_gpu: None,
        low_vram: None,
        vocab_only: None,
        use_mmap: None,
        use_mlock: None,
        num_thread: None,
    };

    let request = ChatRequest {
        model: local_model.name.to_string(),
        prompt: None,
        stream: false,
        options: Some(options),
        messages: Some(vec![msg]),
        format: None,
        tools: Some(tools.clone()),
    };

    let res = ollama.chat_dump(&request).await;

    let x = match res {
        Ok((response, duration)) => TestResult {
            request: Some(request),
            response: Some(response),
            prompt: prompt.to_string(),
            duration,
            model: local_model.model.clone(),
            error: None,
            success: true,
        },
        Err(e) => TestResult {
            request: None,
            response: None,
            prompt: prompt.to_string(),
            model: "".to_string(),
            duration: 0,
            error: Some(e.to_string()),
            success: false,
        },
    };
    Ok(x)
}
