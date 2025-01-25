use crate::error::webshoperror::WebshopError;
use crate::schema::order_items;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::order_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbOrderItem {
    pub id: i32,
    pub order_id: String,
    pub item_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: f64,
    pub state: Option<String>,
    pub additional_info_1: Option<String>,
    pub additional_info_2: Option<String>,
    pub item_created: NaiveDateTime,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::order_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(DbOrders, foreign_key = order_number))]
pub struct DbNewOrderItem {
    pub order_id: String,
    pub item_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: f64,
    pub state: Option<String>,
    pub additional_info_1: Option<String>,
    pub additional_info_2: Option<String>,
    pub item_created: NaiveDateTime,
}

pub async fn order_items_insert(
    pool: &deadpool_diesel::postgres::Pool,
    order_items: Vec<DbNewOrderItem>,
) -> Result<DbOrderItem, WebshopError> {
    let conn = pool.get().await?;

    conn.interact(move |conn| {
        diesel::insert_into(order_items::table)
            .values(order_items)
            .returning(DbOrderItem::as_returning())
            .get_result(conn)
    })
    .await
    .map_err(WebshopError::from)?
    .map_err(WebshopError::from)
}

#[allow(dead_code)]
pub async fn order_items_last_item_created(
    pool: &deadpool_diesel::postgres::Pool,
) -> Result<Option<DbOrderItem>, WebshopError> {
    let conn = pool.get().await?;

    conn.interact(move |conn| {
        order_items::table
            .order(order_items::item_created.desc())
            .select(DbOrderItem::as_select())
            .first(conn)
            .optional()
    })
    .await
    .map_err(WebshopError::from)?
    .map_err(WebshopError::from)
}
