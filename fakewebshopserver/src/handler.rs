use crate::models::{AppState, Order, OrderRequest};
use crate::webshoperror::WebshopError;
use axum::extract::State;
use axum::Json;
use tower_http::cors::CorsLayer;

pub fn cors_layer() -> CorsLayer {
    CorsLayer::permissive()
}

#[axum::debug_handler]
pub async fn server_orders(
    State(state): State<AppState>,
    Json(request): Json<OrderRequest>,
) -> Result<Json<Vec<Order>>, WebshopError> {
    println!("orders request {:?}", request);

    let first_idx = state
        .orders
        .iter()
        .position(|o| o.order_created > request.last_order_created);

    if first_idx.is_none() {
        return Ok(Json(vec![]));
    }

    let first_idx = first_idx.unwrap();
    let orders: Vec<Order> = state
        .orders
        .iter()
        .skip(first_idx)
        .take(request.page_size as usize)
        .cloned()
        .collect();

    let order_ids = orders
        .iter()
        .map(|o| o.order_id.clone())
        .collect::<Vec<_>>();

    println!(
        "total orders found {}, request.last_order_created {}, orders found {:?}",
        state.orders.len(),
        request.last_order_created,
        orders.len()
    );

    println!("returning order_ids  {:?}, ", order_ids.join(" // "));

    Ok(Json(orders))
}
