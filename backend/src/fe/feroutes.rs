use crate::common::db_chat::{
    ollama_cat_update_result, ollama_chat_load_all, ollama_chat_load_by_prompt_id,
};
use crate::common::db_model::{ollama_model_insert, ollama_model_load_by_id, ollama_models_load};
use crate::common::db_prompt::{
    ollama_prompt_insert, ollama_prompt_load, ollama_prompt_load_by_id,
};
use crate::common::db_queue::{ollama_queue_all, ollama_queue_insert};
use crate::fe::femodels::{
    FeDbOllamaModel, FeLiveChat, FeOllamaChat, FeOllamaChatQueue, FeOllamaChatQueueResponse,
    FeOllamaInformation, FeOllamaModel, FeOllamaPrompt, FeRunModelRequest, FeStreamingRequest,
    FeUpdateOllamaChatResult, InsertModelsResponse,
};
use crate::server::ollamachat_error::OllamaChatError;
use crate::CONFIG;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use ollama::api::OllamaImpl;
use ollama::error::OllamaError;
use ollama::models::ContentEnum::{AContent, AString};
use ollama::models::{
    ChatRequest, ChatResponse, ContentEnum, CreateModelRequest, Function, Message, Ollama,
    Parameter, Property, Tool,
};
use ollama::tools_v2::get_tools_v2;
use std::collections::HashMap;
use tracing::info;

pub async fn import_local_models(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<InsertModelsResponse>>, OllamaChatError> {
    let o = Ollama::new(CONFIG.ollama_url.clone())?;
    let models = o.local_models().await?;
    let res = ollama_model_insert(&pool, &models).await?;
    Ok(Json(res))
}

pub async fn list_local_models() -> Result<Json<Vec<FeOllamaModel>>, OllamaChatError> {
    let o = Ollama::new(CONFIG.ollama_url.clone())?;
    let models = o.local_models().await?;

    let models: Vec<FeOllamaModel> = models
        .iter()
        .map(|x| FeOllamaModel {
            name: x.name.clone(),
            model: x.model.clone(),
            size: x.size,
            detail_format: x.details.format.clone().unwrap_or_default(),
            detail_family: x.details.family.clone().unwrap_or_default(),
            detail_parameter_size: x.details.parameter_size.clone().unwrap_or_default(),
            detail_quantization_level: x.details.quantization_level.clone().unwrap_or_default(),
        })
        .collect();

    Ok(Json(models))
}

pub async fn add_to_queue(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(fe_run_model_request): Json<FeRunModelRequest>,
) -> Result<Json<Vec<FeOllamaChatQueueResponse>>, OllamaChatError> {
    info!("inserting requests into enqueue {:?}", fe_run_model_request);

    let db_ollama_prompt = ollama_prompt_insert(&pool, fe_run_model_request.prompt.clone()).await?;

    let mut res = vec![];

    for model in fe_run_model_request.models {
        let db_queue = ollama_queue_insert(&pool, db_ollama_prompt.id, model.clone()).await?;
        info!(
            "inserted enqueue element id {:?}, enqueue id {}, model id {}",
            db_queue.id, db_queue.model_id, db_queue.prompt_id
        );

        let r = FeOllamaChatQueueResponse {
            id: db_queue.id,
            model_id: db_queue.model_id,
            prompt_id: db_queue.prompt_id,
            state: db_queue.state.clone(),
            created: db_queue.created.and_utc(),
        };
        res.push(r);
    }

    Ok(Json(res))
}

pub async fn prompts_load(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<FeOllamaPrompt>>, OllamaChatError> {
    let db_ollama_prompts = ollama_prompt_load(&pool).await?;

    let prompts: Vec<FeOllamaPrompt> = db_ollama_prompts
        .iter()
        .map(|pprompt| FeOllamaPrompt {
            id: pprompt.id,
            prompt: pprompt.prompt.clone(),
            created: pprompt.created.and_utc(),
        })
        .collect();

    Ok(Json(prompts))
}

pub async fn prompts_load_by_id(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Path(pprompt_id): Path<i32>,
) -> Result<Json<FeOllamaPrompt>, OllamaChatError> {
    let db_ollama_prompts = ollama_prompt_load_by_id(&pool, pprompt_id).await?;

    let res = match db_ollama_prompts {
        Some(pprompt) => FeOllamaPrompt {
            id: pprompt.id,
            prompt: pprompt.prompt.clone(),
            created: pprompt.created.and_utc(),
        },
        None => return Err(OllamaChatError::DataError("prompt not found".to_string())),
    };

    Ok(Json(res))
}

pub async fn chat_load_by_prompt_id(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Path(pprompt_id): Path<i32>,
) -> Result<Json<Vec<FeOllamaChat>>, OllamaChatError> {
    let db_ollama_chats = ollama_chat_load_by_prompt_id(&pool, pprompt_id).await?;

    let mut res = vec![];

    for chat in db_ollama_chats {
        let db_model = ollama_model_load_by_id(&pool, chat.model_id)
            .await?
            .expect("expect the model to be present");

        let db_prompt = ollama_prompt_load_by_id(&pool, chat.prompt_id)
            .await?
            .expect("expect the prompt to be present");

        let p = FeOllamaChat {
            id: chat.id,
            model_id: chat.model_id,
            prompt_id: chat.prompt_id,
            model_name: db_model.name,
            prompt: db_prompt.prompt,
            result: chat.result,
            model_size: db_model.detail_parameter_size,
            response: chat.response.clone(),
            seed: chat.seed,
            num_ctx: chat.num_ctx,
            temperature: chat.temperature,
            top_k: chat.top_k,
            top_p: chat.top_p,
            duration_ms: chat.duration_ms,
            created: chat.created.and_utc(),
        };
        res.push(p);
    }

    Ok(Json(res))
}

pub async fn chat_load_all(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<FeOllamaChat>>, OllamaChatError> {
    let db_ollama_chats = ollama_chat_load_all(&pool).await?;

    let mut res = vec![];

    for chat in db_ollama_chats {
        let db_model = ollama_model_load_by_id(&pool, chat.model_id)
            .await?
            .expect("expect the model to be present");

        let db_prompt = ollama_prompt_load_by_id(&pool, chat.prompt_id)
            .await?
            .expect("expect the prompt to be present");

        let p = FeOllamaChat {
            id: chat.id,
            model_id: chat.model_id,
            prompt_id: chat.prompt_id,
            prompt: db_prompt.prompt,
            model_name: db_model.name,
            model_size: db_model.detail_parameter_size,
            response: chat.response.clone(),
            result: chat.result.clone(),
            seed: chat.seed,
            num_ctx: chat.num_ctx,
            temperature: chat.temperature,
            top_k: chat.top_k,
            top_p: chat.top_p,
            duration_ms: chat.duration_ms,
            created: chat.created.and_utc(),
        };
        res.push(p);
    }

    Ok(Json(res))
}

pub async fn models_loaded() -> Result<Json<Vec<FeOllamaModel>>, OllamaChatError> {
    let o = Ollama::new(CONFIG.ollama_url.clone())?;
    println!("o {:?}", o);

    let models = o.loaded_models().await?;
    println!("got some models {:?}", models);

    let loaded_models: Vec<FeOllamaModel> = models
        .iter()
        .map(|x| FeOllamaModel {
            name: x.name.clone(),
            model: x.model.clone(),
            size: x.size,
            detail_format: x.details.format.clone().unwrap_or_default(),
            detail_family: x.details.family.clone().unwrap_or_default(),
            detail_parameter_size: x.details.parameter_size.clone().unwrap_or_default(),
            detail_quantization_level: x.details.quantization_level.clone().unwrap_or_default(),
        })
        .collect();

    Ok(Json(loaded_models))
}

pub async fn queue_load(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<FeOllamaChatQueue>>, OllamaChatError> {
    let ollama_queues = ollama_queue_all(&pool).await?;

    let ollama_queues: Vec<FeOllamaChatQueue> = ollama_queues
        .iter()
        .map(|x| FeOllamaChatQueue {
            id: x.id,
            model_id: x.model_id,
            prompt_id: x.prompt_id,
            state: x.state.clone(),
            temperature: x.temperature,
            seed: x.seed,
            num_ctx: x.num_ctx,
            top_k: x.top_k,
            top_p: x.top_p,
            created: x.created.and_utc(),
            updated: x.updated.and_utc(),
        })
        .collect();

    Ok(Json(ollama_queues))
}

// https://github.com/tokio-rs/axum/blob/main/examples/reqwest-response/src/main.rs
// you have to make sure, that the request has set "streaming" to true, otherwise all bets are off
pub async fn streaming_response(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(request): Json<FeStreamingRequest>,
) -> impl IntoResponse {
    println!("got a request for streaming chat {:?}", request);
    let db_model = ollama_model_load_by_id(&pool, request.model_id)
        .await
        .expect("expect the model to be present")
        // hahahahaaaha
        .expect("expect the model to be present");

    let o = Ollama::new(CONFIG.ollama_url.clone()).expect("should be able to create Ollama");
    println!("o {:?}", o);

    let property_location = Property {
        typ: "string".to_string(),
        description: "The location to get the weather for, e.g. San Francisco, CA".to_string(),
        enums: None,
    };

    let property_format = Property {
        typ: "string".to_string(),
        description: "The format to return the weather in, e.g. 'celsius' or 'fahrenheit'"
            .to_string(),
        enums: Some(vec!["celsius".to_string(), "fahrenheit".to_string()]),
    };

    let mut properties = HashMap::new();
    properties.insert("location".to_string(), property_location);
    properties.insert("format".to_string(), property_format);

    let parameters = Parameter {
        typ: "object".to_string(),
        properties,
        required: Some(vec!["location".to_string(), "format".to_string()]),
    };

    let function = Function {
        name: "get_current_weather".to_string(),
        description: "Get the current weather for a location".to_string(),
        parameters,
    };

    let tool = Tool {
        typ: "function".to_string(),
        function,
    };

    let request = ChatRequest {
        model: db_model.name.clone(),
        prompt: Some(request.prompt.clone()),
        stream: true,
        options: None,
        messages: None,
        format: None,
        tools: Some(vec![tool]),
    };

    let res = o
        .chat_streaming(&request)
        .await
        .expect("couldn't send chat plain response");

    let mut response_builder = Response::builder().status(res.status());
    *response_builder.headers_mut().unwrap() = res.headers().clone();
    println!("done?");
    response_builder
        .body(Body::from_stream(res.bytes_stream()))
        // This unwrap is fine because the body is empty here
        .unwrap()
}

pub async fn chat_update_result(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(request): Json<FeUpdateOllamaChatResult>,
) -> Result<Json<FeOllamaChat>, OllamaChatError> {
    let chat = ollama_cat_update_result(&pool, request.chat_id, request.result).await?;

    let db_model = ollama_model_load_by_id(&pool, chat.model_id)
        .await?
        .expect("expect the model to be present");

    let db_prompt = ollama_prompt_load_by_id(&pool, chat.prompt_id)
        .await?
        .expect("expect the prompt to be present");

    let p = FeOllamaChat {
        id: chat.id,
        model_id: chat.model_id,
        prompt_id: chat.prompt_id,
        prompt: db_prompt.prompt,
        model_name: db_model.name,
        model_size: db_model.detail_parameter_size,
        response: chat.response.clone(),
        result: chat.result.clone(),
        seed: chat.seed,
        num_ctx: chat.num_ctx,
        temperature: chat.temperature,
        top_k: chat.top_k,
        top_p: chat.top_p,
        duration_ms: chat.duration_ms,
        created: chat.created.and_utc(),
    };

    Ok(Json(p))
}

pub async fn list_db_models(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<FeDbOllamaModel>>, OllamaChatError> {
    let models = ollama_models_load(&pool).await?;
    let loaded_models: Vec<FeDbOllamaModel> = models
        .iter()
        .map(|x| FeDbOllamaModel {
            id: x.id,
            name: x.name.clone(),
            model: x.model.clone(),
            size: x.size,
            detail_format: x.detail_format.clone(),
            detail_family: x.detail_family.clone(),
            detail_parameter_size: x.detail_parameter_size.clone(),
            detail_quantization_level: x.detail_quantization_level.clone(),
            created: x.created.and_utc(),
        })
        .collect();

    Ok(Json(loaded_models))
}

pub async fn models_details(
    Path(model): Path<String>,
) -> Result<Json<FeOllamaInformation>, OllamaChatError> {
    let o = Ollama::new(CONFIG.ollama_url.clone())?;
    let details = o.details(&model).await?;
    let details = FeOllamaInformation {
        modelfile: details.modelfile.to_string(),
        parameters: details.parameters,
        template: details.template.to_string(),
        details: details.details,
        model_info: details.model_info,
        license: details.license.unwrap_or_default(),
        modified_at: details.modified_at.clone().unwrap_or_default(),
    };
    Ok(Json(details))
}

pub async fn model_create(Json(request): Json<CreateModelRequest>) -> impl IntoResponse {
    let o = Ollama::new(CONFIG.ollama_url.clone()).expect("should create Ollama");
    let res = o.create(request).await;
    match res {
        Ok(r) => {
            let mut response_builder = Response::builder().status(r.status());
            *response_builder.headers_mut().unwrap() = r.headers().clone();
            println!("done?");
            response_builder
                .body(Body::from_stream(r.bytes_stream()))
                // This unwrap is fine because the body is empty here
                .unwrap()
        }
        Err(e) => {
            println!("error: {:?}", e);
            let response_builder = Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR);
            response_builder
                .status(500)
                .body(format!("error: {}", e).into())
                .unwrap()
        }
    }
}

// https://github.com/tokio-rs/axum/blob/main/examples/reqwest-response/src/main.rs
// you have to make sure, that the request has set "streaming" to true, otherwise all bets are off
pub async fn chat(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(request): Json<FeLiveChat>,
) -> Result<Json<FeLiveChat>, OllamaChatError> {
    println!("got a request for streaming chat \n{:?}\n", request);
    // let db_model = ollama_model_load_by_id(&pool, request.model_id)
    //     .await
    //     .expect("expect the model to be present")
    //     // hahahahaaaha
    //     .expect("expect the model to be present");
    //
    // let o = Ollama::new(CONFIG.ollama_url.clone()).expect("should be able to create Ollama");
    //
    // let tools = match request.enable_tools {
    //     true => Some(get_tools_v2()),
    //     false => None,
    // };
    //
    // let new_message = Message {
    //     role: "user".to_string(),
    //     content: Some(AString(fe_request.prompt.clone())),
    //     images: None,
    //     tool_calls: None,
    //     tool_call_id: None,
    // };
    //
    // let mut messages = fe_request.messages;
    //
    // messages.push(new_message);
    //
    // println!("messages {:?}", messages);
    //
    // let request = ChatRequest {
    //     model: db_model.name.clone(),
    //     prompt: None,
    //     stream: false,
    //     options: None,
    //     messages: Some(messages.clone()),
    //     format: None,
    //     tools,
    // };
    //
    // let (response, duration) = o
    //     .chat(&request)
    //     .await
    //     .expect("couldn't send chat plain response");
    //
    // println!(
    //     "request for model {} took {}ms. prompt {}",
    //     db_model.model, duration, fe_request.prompt
    // );
    //
    // // TODO loop and satisfy tool_calls until there are no tool calls any more ..
    // let markdown = match response.message {
    //     Some(ref msg) => {
    //         messages.push(msg.clone());
    //         match &msg.content {
    //             Some(content) => match content {
    //                 ContentEnum::AString(s) => s.to_string(),
    //                 ContentEnum::AContent(con) => {
    //                     format!("{:?}", con)
    //                 }
    //             },
    //             None => {
    //                 println!("no content in chat response found in  response message from Ollama?");
    //                 "no content in chat response found in  response message from Ollama?"
    //                     .to_string()
    //             }
    //         }
    //     }
    //     None => {
    //         println!("this does not seem right, there is no response message from Ollama?");
    //         "no response from ollama :-(".to_string()
    //     }
    // };
    //
    // let message_ping_pong = MessagePingPong {
    //     request,
    //     responses: vec![response],
    //     markdown,
    // };
    //
    // let r = FeLiveChatResponse {
    //     req_resp: vec![message_ping_pong],
    // };

    Ok(Json(request))
}
