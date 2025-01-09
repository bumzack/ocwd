use crate::client::config::get_client;
use crate::client::webmodels::{Order, OrderItem, OrderItemRequest, OrderRequest};
use crate::error::webshoperror::WebshopError;
use crate::CONF;
use serde_json::json;

pub async fn get_orders(order_request: OrderRequest) -> Result<Vec<Order>, WebshopError> {
    let server = CONF.api_url.clone();
    let url = format!("{}/api/orders", server);

    println!("url {}", url);
    let client = get_client(300).expect("should get a reqwest client");
    let body = json!(order_request);
    let response = client.post(url).json(&body).send().await?;

    println!("orders response {:?}", response);

    let body = response.text().await?;

    let response: Vec<Order> = serde_json::from_str(&body)?;

    println!("orders:  {:#?}", response);

    Ok(response)
}

pub async fn get_items(item_request: OrderItemRequest) -> Result<Vec<OrderItem>, WebshopError> {
    let server = CONF.api_url.clone();
    let url = format!("{}/api/items", server);
    println!("url {}", url);

    let client = get_client(300).expect("should get a reqwest client");
    let body = json!(item_request);
    let response = client.post(url).json(&body).send().await?;

    println!("items response {:?}", response);

    let body = response.text().await?;

    let response: Vec<OrderItem> = serde_json::from_str(&body)?;

    println!("items: {:#?}", response);

    Ok(response)
}
