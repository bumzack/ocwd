use crate::models::{Order, OrderItem, OrderRequest};
use crate::webshoperror::WebshopError;
use axum::extract::MatchedPath;
use axum::http::Request;
use axum::response::Response;
use axum::routing::post;
use axum::{Json, Router};
use chrono::Utc;
use rand::Rng;
use std::net::SocketAddr;
use std::time::Duration;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::Span;

mod models;
mod webshoperror;

pub fn cors_layer() -> CorsLayer {
    CorsLayer::permissive()
}

#[axum::debug_handler]
pub async fn orders(Json(request): Json<OrderRequest>) -> Result<Json<Vec<Order>>, WebshopError> {
    println!("orders request {:?}", request);
    Ok(Json(vec![]))
}

#[axum::debug_handler]
pub async fn items(
    Json(request): Json<OrderRequest>,
) -> Result<Json<Vec<OrderItem>>, WebshopError> {
    println!("items request {:?}", request);
    Ok(Json(vec![]))
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut orders = vec![];
    let mut items = vec![];

    let mut rng = rand::rng();
    let y: f64 = rng.random(); // generates a float between 0 and 1

    let cnt_buyer_ids = rng.random_range(0..30);
    let buyer_ids = (0..cnt_buyer_ids)
        .into_iter()
        .map(|id| uuid::Uuid::new_v4())
        .collect::<Vec<_>>();

    let article_code_cnt = rng.random_range(10..50);
    let article_codes = (0..article_code_cnt)
        .into_iter()
        .map(|id| format!("article_{0:10}", id))
        .collect::<Vec<_>>();

    let cnt_orders = rng.random_range(100..200);

    for i in 0..cnt_orders {
        let order_number = format!("order_nr_{}", i);

        let cnt_items = rng.random_range(2..50);

        let items = (0..cnt_items)
            .into_iter()
            .map(|i| {
                let price: f64 = rng.random_range(0.95..2323.23); // generates a float between 0 and 1
                let state = "finished".to_string();
                let now = Utc::now();
                let code = format!("code_{}", i);

                let item = OrderItem {
                    order_number: order_number.clone(),
                    code: "".to_string(),
                    name: None,
                    description: None,
                    price,
                    state: Some(state),
                    additional_info_1: None,
                    additional_info_2: None,
                    item_created: now,
                };
                item
            })
            .collect::<Vec<_>>();

        let mut chars = vec![];

        for c in 0..20 {
            let c = rng.random_range('a'..'z');
            chars.push(c);
        }

        let name = chars.concat();
        let buyer_id = rng.random_range(0..cnt_buyer_ids);

        let buyer_id = buyer_ids.get(buyer_id).expect("should be there").clone();
        let buyer_name = format!("name_{}", buyer_id);
        let erp_order_number = format!("{}", rng.random_range(100_000..1_000_000));

        let r: f64 = rng.random(); // generates a float between 0 and 1

        let state = if r > 0.9 { "processing" } else { "finished" };

        let state = state.to_string();
        let now = Utc::now();

        let order = Order {
            order_number: order_number,
            buyer_id: buyer_id.to_string(),
            buyer_name: Some(buyer_name),
            erp_order_number: Some(erp_order_number),
            state: Some(state),
            additional_info_1: None,
            additional_info_2: None,
            number_items: 0,
            blacklisted: false,
            order_created: now,
        };
    }

    // build our application with some routes
    let app = Router::new()
        .route("/api/orders", post(orders))
        .route("/api/items", post(items))
        .layer(cors_layer())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    tracing::info!("match path {:?}", matched_path);

                    let request_id = uuid::Uuid::new_v4();
                    tracing::span!(
                        tracing::Level::INFO,
                        "http_request",
                        method = display(request.method()),
                        uri = display(request.uri()),
                        version = debug(request.version()),
                        request_id = display(request_id)
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    tracing::info!("tracing::on_request {:?}", _request);
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    tracing::info!("response {:?}", _response);
                })
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        tracing::error!("tracing::on_failure");
                        tracing::error!("error {:?}", &_error);
                    },
                ),
        );

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 2002));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .expect("should listen on port 3023");

    Ok(())
}
