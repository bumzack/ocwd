use crate::fe::femodels::FeRunModel;
use crate::schema::ollama_chat_queue;
use crate::server::models::QueueState;
use crate::server::ollamachat_error::OllamaChatError;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::ollama_chat_queue)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(DbOllamaModel, foreign_key = model_id))]
#[diesel(belongs_to(DbOllamaPrompt, foreign_key = prompt_id))]
pub struct DbOllamaChatQueue {
    pub id: i32,
    pub model_id: i32,
    pub prompt_id: i32,
    pub state: String,
    pub temperature: f64,
    pub seed: i64,
    pub num_ctx: i64,
    pub top_k: f64,
    pub top_p: f64,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ollama_chat_queue)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(DbOllamaModel, foreign_key = model_id))]
#[diesel(belongs_to(DbOllamaPrompt, foreign_key = prompt_id))]
pub struct DbNewOllamaChatQueue {
    pub model_id: i32,
    pub prompt_id: i32,
    pub state: String,
    pub temperature: f64,
    pub seed: i64,
    pub num_ctx: i64,
    pub top_k: f64,
    pub top_p: f64,
}

pub async fn ollama_queue_insert(
    pool: &deadpool_diesel::postgres::Pool,
    pprompt_id: i32,
    run_model: FeRunModel,
) -> Result<DbOllamaChatQueue, OllamaChatError> {
    let conn = pool.get().await?;

    let state = QueueState::Enqueued;

    let new_chat = DbNewOllamaChatQueue {
        prompt_id: pprompt_id,
        state: state.to_string(),
        model_id: run_model.model_id,
        temperature: run_model.temperature,
        seed: run_model.seed,
        num_ctx: run_model.num_ctx,
        top_k: run_model.top_k,
        top_p: run_model.top_p,
    };

    conn.interact(move |conn| {
        diesel::insert_into(ollama_chat_queue::table)
            .values(new_chat)
            .returning(DbOllamaChatQueue::as_returning())
            .get_result(conn)
    })
    .await
    .map_err(OllamaChatError::from)?
    .map_err(OllamaChatError::from)
}

pub async fn ollama_queue_next(
    pool: &deadpool_diesel::postgres::Pool,
) -> Result<Option<DbOllamaChatQueue>, OllamaChatError> {
    let conn = pool.get().await?;

    conn.interact(move |conn| {
        ollama_chat_queue::table
            .order(ollama_chat_queue::created.asc())
            // sort by model_id to avoid too many model unload/load ops on the "ollama server"
            .order(ollama_chat_queue::model_id.asc())
            .filter(ollama_chat_queue::state.eq(QueueState::Enqueued.to_string()))
            .select(DbOllamaChatQueue::as_select())
            .first(conn)
            .optional()
    })
    .await
    .map_err(OllamaChatError::from)?
    .map_err(OllamaChatError::from)
}

pub async fn ollama_queue_update_state(
    pool: &deadpool_diesel::postgres::Pool,
    queue_id: i32,
    new_state: QueueState,
) -> Result<DbOllamaChatQueue, OllamaChatError> {
    let conn = pool.get().await?;

    let now = Utc::now();
    conn.interact(move |conn| {
        diesel::update(ollama_chat_queue::table.filter(ollama_chat_queue::id.eq(queue_id)))
            .set((
                ollama_chat_queue::state.eq(new_state.to_string()),
                ollama_chat_queue::updated.eq(now),
            ))
            .returning(DbOllamaChatQueue::as_returning())
            .get_result(conn)
            .unwrap()
    })
    .await
    .map_err(OllamaChatError::from)
}

pub async fn ollama_queue_all(
    pool: &deadpool_diesel::postgres::Pool,
) -> Result<Vec<DbOllamaChatQueue>, OllamaChatError> {
    let conn = pool.get().await?;

    conn.interact(move |conn| {
        ollama_chat_queue::table
            // sort by model_id to avoid too many model unload/load ops on the "ollama server"
            .order((
                ollama_chat_queue::updated.desc(),
                ollama_chat_queue::model_id.asc(),
            ))
            .select(DbOllamaChatQueue::as_select())
            .load(conn)
    })
    .await?
    .map_err(OllamaChatError::from)
    .map_err(OllamaChatError::from)
}
