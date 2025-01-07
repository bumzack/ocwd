use crate::client::config::Config;
use crate::client::stuff::{find_and_insert_items, find_and_insert_orders};
use crate::error::error::WebshopError;
use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env;


mod client;
mod db;
mod error;
mod schema;

lazy_static! {
    static ref CONF: Config = {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let basic_auth_key = env::var("BASIC_AUTH_KEY").expect("BASIC_AUTH_KEY must be set");
        let api_url = env::var("API_URL").expect("API_URL must be set");

        Config {
            api_url,
            basic_auth_key,
            database_url,
        }
    };
}

#[tokio::main]
async fn main() -> Result<(), WebshopError> {

    let c= CONF.clone();
    println!("config {:?}", c);

    let database_url = &c.database_url;

    let manager =
        deadpool_diesel::postgres::Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);

    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .expect("should work");

    let res_orders = find_and_insert_orders(&pool).await?;
    println!("inserted {} orders", res_orders);

    let res_items = find_and_insert_items(&pool).await?;
    println!("inserted {} items", res_items);

    Ok(())
}
