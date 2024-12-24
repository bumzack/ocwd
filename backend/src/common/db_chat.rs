use crate::ollama::ollama_rest_api_models::{OllamaRequest, OllamaResponse};
use crate::schema::ollama_chat;
use crate::server::ollamachat_error::OllamaChatError;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Insertable, Queryable, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ollama_chat)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(DbOllamaModel, foreign_key = model_id))]
#[diesel(belongs_to(DbOllamaPrompt, foreign_key = prompt_id))]
// #[diesel(belongs_to(DbOllamaChat, foreign_key = parent_id))]
pub struct DbOllamaChat {
    pub id: i32,
    pub model_id: i32,
    pub prompt_id: i32,
    pub parent_id: Option<i32>,
    pub response: String,
    pub ollama_response_json: serde_json::Value,
    pub ollama_request_json: serde_json::Value,
    pub seed: Option<i64>,
    pub num_ctx: Option<i64>,
    pub temperature: Option<f64>,
    pub top_k: Option<f64>,
    pub top_p: Option<f64>,
    pub duration_ms: i64,
    pub created: NaiveDateTime,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ollama_chat)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(DbOllamaModel, foreign_key = model_id))]
#[diesel(belongs_to(DbOllamaPrompt, foreign_key = prompt_id))]
// #[diesel(belongs_to(DbOllamaChat, foreign_key = parent_id))]
pub struct DbNewOllamaChat {
    pub model_id: i32,
    pub parent_id: Option<i32>,
    pub prompt_id: i32,
    pub response: String,
    pub ollama_response_json: serde_json::Value,
    pub ollama_request_json: serde_json::Value,
    pub seed: Option<i64>,
    pub num_ctx: Option<i64>,
    pub temperature: Option<f64>,
    pub top_k: Option<f64>,
    pub top_p: Option<f64>,
    pub duration_ms: i64,
}

pub async fn ollama_chat_insert(
    pool: &deadpool_diesel::postgres::Pool,
    mmodel_id: i32,
    pprompt_id: i32,
    ollama_response: OllamaResponse,
    ollama_request: OllamaRequest,
    duration_ms: i64,
) -> Result<DbOllamaChat, OllamaChatError> {
    let conn = pool.get().await?;

    let new_chat = DbNewOllamaChat {
        model_id: mmodel_id,
        parent_id: None,
        prompt_id: pprompt_id,
        response: ollama_response.response.clone(),
        ollama_response_json: json!(ollama_response),
        ollama_request_json: json!(ollama_request),
        temperature: ollama_request.options.temperature,
        seed: ollama_request.options.seed,
        num_ctx: ollama_request.options.num_ctx,
        top_k: ollama_request.options.top_k,
        top_p: ollama_request.options.top_p,
        duration_ms,
    };

    conn.interact(move |conn| {
        diesel::insert_into(ollama_chat::table)
            .values(new_chat)
            .returning(DbOllamaChat::as_returning())
            .get_result(conn)
    })
    .await
    .map_err(OllamaChatError::from)?
    .map_err(OllamaChatError::from)
}

pub async fn ollama_chat_load_by_prompt_id(
    pool: &deadpool_diesel::postgres::Pool,
    pprompt_id: i32,
) -> Result<Vec<DbOllamaChat>, OllamaChatError> {
    let conn = pool.get().await?;

    conn.interact(move |conn| {
        ollama_chat::table
            //  .order(ollama_chat::name.asc())
            .filter(ollama_chat::prompt_id.eq(pprompt_id))
            .select(DbOllamaChat::as_select())
            .load(conn)
    })
    .await
    .map_err(OllamaChatError::from)?
    .map_err(OllamaChatError::from)
}
