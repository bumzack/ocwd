mod common;
mod fe;
mod schema;
mod server;

use crate::fe::feroutes::{
    chat_load_all, chat_load_by_prompt_id, chat_update_result, models_loaded, prompts_load,
    prompts_load_by_id, queue_load, streaming_response,
};
use crate::fe::feroutes::{list_db_models, model_create, models_details};
use crate::server::models::Config;
use crate::server::queue::{start_queue, stop_queue};
use crate::server::utils::cors_layer;
use axum::extract::MatchedPath;
use axum::http::Request;
use axum::response::Response;
use axum::routing::{get, post, put};
use axum::Router;
use dotenvy::dotenv;
use fe::feroutes::{add_to_queue, import_local_models, list_local_models};
use lazy_static::lazy_static;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::TraceLayer;
use tracing::Span;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;

lazy_static! {
    static ref CONFIG: Config = {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let ollama_url = env::var("OLLAMA_URL").expect("OLLAMA_URL must be set");

        Config {
            database_url,
            ollama_url,
        }
    };
}

lazy_static! {
    static ref QUEUE_RUNNING: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let info_file = rolling::daily("./logs", "ollama_chat").with_max_level(tracing::Level::INFO);

    tracing_subscriber::fmt()
        .with_writer(info_file)
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .init();

    let database_url = &CONFIG.database_url;

    let manager =
        deadpool_diesel::postgres::Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);

    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .expect("should work");

    // run_queue(pool.clone()).await.expect("should start queue");

    // build our application with some routes
    let app = Router::new()
        .route("/api/model/import", post(import_local_models))
        .route("/api/model", get(list_local_models))
        .route("/api/model/enqueue", post(add_to_queue))
        .route("/api/model/loaded", get(models_loaded))
        .route("/api/model/create", post(model_create))
        .route("/api/model/details/{model}", get(models_details))
        .route("/api/dbmodel", get(list_db_models))
        .route("/api/prompt", get(prompts_load))
        .route("/api/prompt/{pprompt_id}", get(prompts_load_by_id))
        .route("/api/chat", get(chat_load_all))
        .route("/api/chat/result", put(chat_update_result))
        .route("/api/chat/byid/{pprompt_id}", get(chat_load_by_prompt_id))
        .route("/api/queue/stop", post(stop_queue))
        .route("/api/queue/start", post(start_queue))
        .route("/api/queue", get(queue_load))
        .route("/ollama/api/stream", post(streaming_response))
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
        )
        .with_state(pool)
        .layer(cors_layer());

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3023));
    tracing::info!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .expect("should listen on port 3023");

    Ok(())
}
