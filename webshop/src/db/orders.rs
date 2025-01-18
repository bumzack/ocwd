use crate::error::webshoperror::WebshopError;
use crate::schema::orders;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbOrder {
    pub id: i32,
    pub order_id: String,
    pub buyer_id: String,
    pub buyer_name: Option<String>,
    pub erp_order_number: Option<String>,
    pub state: Option<String>,
    pub additional_info_1: Option<String>,
    pub additional_info_2: Option<String>,
    pub number_items: i32,
    pub blacklisted: bool,
    pub order_created: NaiveDateTime,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbNewOrder {
    pub order_id: String,
    pub buyer_id: String,
    pub buyer_name: Option<String>,
    pub erp_order_number: Option<String>,
    pub state: Option<String>,
    pub additional_info_1: Option<String>,
    pub additional_info_2: Option<String>,
    pub number_items: i32,
    pub order_created: NaiveDateTime,
}

pub async fn order_insert(
    pool: &deadpool_diesel::postgres::Pool,
    orders: Vec<DbNewOrder>,
) -> Result<DbOrder, WebshopError> {
    let conn = pool.get().await?;

    conn.interact(move |conn| {
        diesel::insert_into(orders::table)
            .values(orders)
            .returning(DbOrder::as_returning())
            .get_result(conn)
    })
    .await
    .map_err(WebshopError::from)?
    .map_err(WebshopError::from)
}

pub async fn order_last_order_created(
    pool: &deadpool_diesel::postgres::Pool,
) -> Result<Option<DbOrder>, WebshopError> {
    let conn = pool.get().await?;

    conn.interact(move |conn| {
        orders::table
            .order(orders::order_created.desc())
            .select(DbOrder::as_select())
            .first(conn)
            .optional()
    })
    .await
    .map_err(WebshopError::from)?
    .map_err(WebshopError::from)
}
