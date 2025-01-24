use std::env;
use crate::fake_data::{buyer_names, get_description, get_names};
use crate::handler::cors_layer;
use crate::handler::server_orders;
use crate::models::{AppState, Order, OrderItem};
use axum::routing::post;
use axum::Router;
use chrono::{TimeDelta, Utc};
use rand::Rng;
use std::net::SocketAddr;
use std::ops::Sub;
use dotenvy::dotenv;

mod fake_data;
mod handler;
mod models;
mod webshoperror;

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv().ok();

    let min_orders = env::var("MIN_ORDERS").expect("MIN_ORDERS must be set").parse::<i32>().expect("should be a number");
    let max_orders = env::var("MAX_ORDERS").expect("MAX_ORDERS must be set").parse::<i32>().expect("should be a number");

    let min_items = env::var("MIN_ITEMS").expect("MIN_ITEMS must be set").parse::<i32>().expect("should be a number");
    let max_items = env::var("MAX_ITEMS").expect("MAX_ITEMS must be set").parse::<i32>().expect("should be a number");

    let max_order_age_days = env::var("MAX_ORDER_AGE_DAYS").expect("MAX_ORDER_AGE_DAYS must be set").parse::<i64>().expect("should be a number");

    let mut orders = vec![];

    let mut rng = rand::rng();

    let article_names: Vec<String> = get_names().iter().map(|s| s.trim().to_string()).collect();
    let article_descriptions: Vec<String> = get_description()
        .iter()
        .map(|s| s.trim().to_string())
        .collect();

    let buyer_names: Vec<String> = buyer_names().iter().map(|s| s.trim().to_string()).collect();

    let mut articles = vec![];

    for idx in 0..article_names.len() {
        let article_code = format!("{:04}", idx);
        let article_name = article_names[idx].to_string();
        articles.push((article_code, article_name));
    }

    let cnt_orders = rng.random_range(min_orders..max_orders);

    for i in 0..cnt_orders {
        let delta_days: i64 = rng.random_range(3..max_order_age_days);
        let created = Utc::now().sub(TimeDelta::days(delta_days));

        let ts = created.timestamp_millis();
        let order_number = format!("order_nr_{:05}_{}", i, ts);
        let cnt_items = rng.random_range(min_items..max_items);

        let items = (0..cnt_items)
            .into_iter()
            .map(|_| {
                let price: f64 = rng.random_range(0.95..2323.23);
                let state = "finished".to_string();

                let article_idx: usize = rng.random_range(0..articles.len()); // generates a float between 0 and 1
                let (item_id, name) = articles[article_idx].clone();
                let description = article_descriptions[article_idx].clone();

                let item = OrderItem {
                    item_id,
                    name,
                    description,
                    price,
                    state: Some(state),
                    additional_info_1: None,
                    additional_info_2: None,
                    item_created: created,
                };
                item
            })
            .collect::<Vec<_>>();

        let buyer_name_idx: usize = rng.random_range(0..buyer_names.len()); // generates a float between 0 and 1
        let buyer_name = buyer_names[buyer_name_idx].clone();
        let buyer_id = format!("id_{}", buyer_name.to_ascii_lowercase());
        let erp_order_number = format!("erp_{:0>10}", rng.random_range(100_000..1_000_000));

        let r: f64 = rng.random(); // generates a float between 0 and 1

        let state = if r > 0.95 { "processing" } else { "finished" };
        let state = state.to_string();

        let order = Order {
            order_id: order_number,
            buyer_id,
            buyer_name,
            erp_order_number: Some(erp_order_number),
            state: Some(state),
            additional_info_1: None,
            additional_info_2: None,
            number_items: items.len() as i32,
            blacklisted: false,
            order_created: created,
            items,
        };
        orders.push(order);
    }

    orders.sort_by(|a, b| a.order_created.cmp(&b.order_created));

    let state = AppState { orders };
    // build our application with some routes
    let app = Router::new()
        .route("/api/orders", post(server_orders))
        .layer(cors_layer())
        .with_state(state);

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 2002));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .expect("should listen on port 3023");

    Ok(())
}
