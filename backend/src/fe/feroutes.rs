use crate::common::db_chat::{ollama_chat_load_all, ollama_chat_load_by_prompt_id};
use crate::common::db_model::{ollama_model_insert, ollama_model_load_by_id, ollama_models_load};
use crate::common::db_prompt::{
    ollama_prompt_insert, ollama_prompt_load, ollama_prompt_load_by_id,
};
use crate::common::db_queue::{ollama_queue_all, ollama_queue_insert};
use crate::fe::femodels::{
    FeOllamaChat, FeOllamaChatQueue, FeOllamaChatQueueResponse, FeOllamaModel, FeOllamaPrompt,
    FeRunModelRequest,
};
use crate::ollama::ollama_rest_api::{get_all_local_models, get_loaded_models};
use crate::ollama::ollama_rest_api_models::InsertModelsResponse;
use crate::server::ollamachat_error::OllamaChatError;
use axum::extract::{Path, State};
use axum::Json;
use chrono::Utc;
use tracing::info;

pub async fn import_local_models(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<InsertModelsResponse>>, OllamaChatError> {
    let models = get_all_local_models().await?;
    let res = ollama_model_insert(&pool, &models).await?;
    Ok(Json(res))
}

pub async fn list_local_models(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<FeOllamaModel>>, OllamaChatError> {
    let models = ollama_models_load(&pool).await?;

    let models: Vec<FeOllamaModel> = models
        .iter()
        .map(|x| FeOllamaModel {
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

    Ok(Json(models))
}

pub async fn add_to_queue(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Json(fe_run_model_request): Json<FeRunModelRequest>,
) -> Result<Json<Vec<FeOllamaChatQueueResponse>>, OllamaChatError> {
    info!("inserting requests into queue {:?}", fe_run_model_request);

    let db_ollama_prompt = ollama_prompt_insert(&pool, fe_run_model_request.prompt.clone()).await?;

    let mut res = vec![];

    for model in fe_run_model_request.models {
        let db_queue = ollama_queue_insert(&pool, db_ollama_prompt.id, model.clone()).await?;
        info!(
            "inserted queue element id {:?}, queue id {}, model id {}",
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
    let loaded_models = get_loaded_models().await?;

    let loaded_models: Vec<FeOllamaModel> = loaded_models
        .iter()
        .map(|x| FeOllamaModel {
            id: -1,
            name: x.name.clone(),
            model: x.model.clone(),
            size: x.size,
            detail_format: x.details.format.clone(),
            detail_family: x.details.family.clone(),
            detail_parameter_size: x.details.parameter_size.clone(),
            detail_quantization_level: x.details.quantization_level.clone(),
            created: Utc::now(),
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
