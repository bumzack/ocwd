use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    pub last_order_created: DateTime<Utc>,
    pub page_size: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub order_id: String,
    pub buyer_id: String,
    pub buyer_name: String,
    pub erp_order_number: Option<String>,
    pub state: Option<String>,
    pub additional_info_1: Option<String>,
    pub additional_info_2: Option<String>,
    pub number_items: i32,
    pub blacklisted: bool,
    pub order_created: DateTime<Utc>,
    pub items: Vec<OrderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderItem {
    pub item_id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub state: Option<String>,
    pub additional_info_1: Option<String>,
    pub additional_info_2: Option<String>,
    pub item_created: DateTime<Utc>,
}
