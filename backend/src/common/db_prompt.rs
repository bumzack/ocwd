use crate::schema::ollama_prompt;
use crate::server::ollamachat_error::OllamaChatError;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, PartialEq, Clone)]
#[diesel(table_name = crate::schema::ollama_prompt)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbOllamaPrompt {
    pub id: i32,
    pub prompt: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::ollama_prompt)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbNewOllamaPrompt {
    pub prompt: String,
}

pub async fn ollama_prompt_insert(
    pool: &deadpool_diesel::postgres::Pool,
    prompt: String,
) -> Result<DbOllamaPrompt, OllamaChatError> {
    let conn = pool.get().await?;

    let new_prompt = DbNewOllamaPrompt { prompt };

    conn.interact(|conn| {
        diesel::insert_into(ollama_prompt::table)
            .values(new_prompt)
            .returning(DbOllamaPrompt::as_returning())
            .get_result(conn)
    })
    .await
    .map_err(OllamaChatError::from)?
    .map_err(OllamaChatError::from)
}

pub async fn ollama_prompt_load_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    pprompt_id: i32,
) -> Result<Option<DbOllamaPrompt>, OllamaChatError> {
    let conn = pool.get().await?;

    conn.interact(move |conn| {
        ollama_prompt::table
            .filter(ollama_prompt::id.eq(pprompt_id))
            .select(DbOllamaPrompt::as_select())
            .first(conn)
            .optional()
    })
    .await
    .map_err(OllamaChatError::from)?
    .map_err(OllamaChatError::from)
}

pub async fn ollama_prompt_load(
    pool: &deadpool_diesel::postgres::Pool,
) -> Result<Vec<DbOllamaPrompt>, OllamaChatError> {
    let conn = pool.get().await?;

    conn.interact(move |conn| {
        ollama_prompt::table
            .order(ollama_prompt::created.desc())
            .select(DbOllamaPrompt::as_select())
            .load(conn)
    })
    .await
    .map_err(OllamaChatError::from)?
    .map_err(OllamaChatError::from)
}
