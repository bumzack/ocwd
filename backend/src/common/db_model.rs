use crate::fe::femodels::InsertModelsResponse;
use crate::schema::ollama_model;
use crate::server::ollamachat_error::OllamaChatError;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use ollama::models::ListModel;
use tracing::error;

#[derive(Queryable, Selectable, Debug, PartialEq, Clone)]
#[diesel(table_name = crate::schema::ollama_model)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbOllamaModel {
    pub id: i32,
    pub name: String,
    pub model: String,
    pub size: i64,
    pub detail_format: String,
    pub detail_family: String,
    pub detail_parameter_size: String,
    pub detail_quantization_level: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Insertable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::ollama_model)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbNewOllamaModel {
    pub name: String,
    pub model: String,
    pub size: i64,
    pub detail_format: String,
    pub detail_family: String,
    pub detail_parameter_size: String,
    pub detail_quantization_level: String,
}

pub async fn ollama_model_insert(
    pool: &deadpool_diesel::postgres::Pool,
    models: &[ListModel],
) -> Result<Vec<InsertModelsResponse>, OllamaChatError> {
    let conn = pool.get().await?;

    let mut res: Vec<InsertModelsResponse> = vec![];

    for model in models {
        let u = DbNewOllamaModel {
            name: model.name.clone(),
            model: model.model.clone(),
            size: model.size,
            detail_format: model.details.format.clone(),
            detail_family: model.details.family.clone(),
            detail_parameter_size: model.details.parameter_size.clone(),
            detail_quantization_level: model.details.quantization_level.clone(),
        };

        let db_ollama_model = conn
            .interact(|conn| {
                diesel::insert_into(ollama_model::table)
                    .values(u)
                    .returning(DbOllamaModel::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(OllamaChatError::from)?;

        let model_response = match db_ollama_model {
            Ok(db_model) => {
                tracing::info!(
                    "SUCCESS: result of inserting a ollama_model is ok:  {:?}",
                    db_model.id
                );
                InsertModelsResponse {
                    model: model.model.clone(),
                    name: model.name.clone(),
                    model_id: Some(db_model.id),
                    result: "success inserting model".to_string(),
                }
            }
            Err(err) => {
                error!("error inserting a model into the DB {}", err);
                InsertModelsResponse {
                    model: model.model.clone(),
                    name: model.name.clone(),
                    model_id: None,
                    result: "error inserting model".to_string(),
                }
            }
        };
        res.push(model_response);
    }

    Ok(res)
}

pub async fn ollama_models_load(
    pool: &deadpool_diesel::postgres::Pool,
) -> Result<Vec<DbOllamaModel>, OllamaChatError> {
    let conn = pool.get().await?;

    conn.interact(move |conn| {
        ollama_model::table
            .order(ollama_model::name.asc())
            .select(DbOllamaModel::as_select())
            .load(conn)
    })
    .await
    .map_err(OllamaChatError::from)?
    .map_err(OllamaChatError::from)
}

pub async fn ollama_model_load_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    model_id: i32,
) -> Result<Option<DbOllamaModel>, OllamaChatError> {
    let conn = pool.get().await?;

    conn.interact(move |conn| {
        ollama_model::table
            .filter(ollama_model::id.eq(model_id))
            .select(DbOllamaModel::as_select())
            .first(conn)
            .optional()
    })
    .await
    .map_err(OllamaChatError::from)?
    .map_err(OllamaChatError::from)
}
