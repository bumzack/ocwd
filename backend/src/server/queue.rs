use crate::common::db_chat::ollama_chat_insert;
use crate::common::db_model::{ollama_model_load_by_id, ollama_models_load};
use crate::common::db_prompt::ollama_prompt_load_by_id;
use crate::common::db_queue::{ollama_queue_next, ollama_queue_update_state};
use crate::ollama::ollama_rest_api::{execute_ollama_chat, execute_ollama_unload};
use crate::ollama::ollama_rest_api_models::{OllamaOptions, OllamaRequest};
use crate::server::models::QueueState;
use crate::server::ollamachat_error::OllamaChatError;
use crate::QUEUE_RUNNING;
use chrono::Utc;
use deadpool_diesel::postgres::Pool;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{error, info};

pub async fn run_queue(pool: deadpool_diesel::postgres::Pool) -> Result<(), OllamaChatError> {
    tokio::spawn(async move {
        let p = pool.clone();

        loop {
            let now = Utc::now();

            let running = QUEUE_RUNNING.lock().await;

            info!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
            info!("processing queue @ {:?}. running {:?}", now, *running);
            info!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

            if !*running {
                let sleep_duration = Duration::from_secs(30);

                info!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
                info!(
                    "processing queue NOT running @ {:?}. waiting for {:?}",
                    now, sleep_duration
                );
                info!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

                sleep(sleep_duration).await;
                continue;
            } else {
                info!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
                info!("processing queue  running @ {:?}", now);
                info!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
            }

            let next = ollama_queue_next(&p).await.expect("queue next failed");

            match next {
                Some(db_queue) => {
                    info!("start unloading models @ {}", Utc::now());
                    ollama_unload_all_models(&pool).await;
                    info!("finished unloading models @ {}", Utc::now());

                    let db_prompt = ollama_prompt_load_by_id(&pool, db_queue.prompt_id)
                        .await
                        .expect("queue element not found")
                        .expect("queue element not found");
                    let db_model = ollama_model_load_by_id(&pool, db_queue.model_id)
                        .await
                        .expect("model element not found")
                        .expect("model element not found");

                    let opt = OllamaOptions {
                        temperature: db_queue.temperature,
                        num_ctx: db_queue.num_ctx,
                        seed: db_queue.seed,
                        top_k: db_queue.top_k,
                        top_p: db_queue.top_p,
                    };

                    let request = OllamaRequest {
                        model: db_model.model,
                        prompt: db_prompt.prompt.clone(),
                        stream: false,
                        options: opt,
                    };
                    info!("queue_element={:?}", db_queue);

                    let start = Instant::now();

                    let db_ollama_queue_update =
                        ollama_queue_update_state(&pool, db_queue.id, QueueState::Processing).await;

                    match db_ollama_queue_update {
                        Ok(res) => {
                            info!("successfully updated ollama queue status. id {}", res.id);
                        }
                        Err(e) => {
                            error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
                            error!(
                                "error updating queue in DB, queue.id {} {:?}",
                                db_queue.id, e
                            );
                            error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
                        }
                    }

                    info!("------------------------------------------------------------------------------------------");
                    info!("starting request to Ollama.");
                    let ollama_response = execute_ollama_chat(request.clone()).await;
                    let duration = start.elapsed().as_millis();
                    info!("finished request to Ollama.");
                    info!("------------------------------------------------------------------------------------------");

                    match ollama_response {
                        Ok(response) => {
                            let db_ollama_queue_update =
                                ollama_queue_update_state(&pool, db_queue.id, QueueState::Finished)
                                    .await;

                            match db_ollama_queue_update {
                                Ok(res) => {
                                    info!(
                                        "successfully updated ollama queue status. id {}",
                                        res.id
                                    );
                                }
                                Err(e) => {
                                    error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
                                    error!(
                                        "error updating queue in DB, queue.id {} {:?}",
                                        db_queue.id, e
                                    );
                                    error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
                                }
                            }

                            info!("finished request to Ollama after {:?}ms.", duration);

                            let db_ollama_chat = ollama_chat_insert(
                                &pool,
                                db_model.id,
                                db_prompt.id,
                                response,
                                request,
                                duration as i64,
                            )
                            .await;

                            match db_ollama_chat {
                                Ok(res) => {
                                    info!("successfully inserted new ollama chat. id {}", res.id);
                                }
                                Err(e) => {
                                    error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
                                    error!("error inserting into DB {:?}", e);
                                    error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
                                }
                            }
                        }
                        Err(e) => {
                            error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
                            error!("error executing request by ollama server {:?}", e);
                            error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
                            let db_ollama_queue_update =
                                ollama_queue_update_state(&pool, db_queue.id, QueueState::Error)
                                    .await;

                            match db_ollama_queue_update {
                                Ok(res) => {
                                    info!("successfully updated queue status. id {}", res.id);
                                }
                                Err(e) => {
                                    error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
                                    error!(
                                        "error updating queue status in DB. queue.id {}, error: {:?}",
                                        db_queue.id, e
                                    );
                                    error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
                                }
                            }
                        }
                    }
                }
                None => {
                    info!("no 'next' element, waiting ... ");
                    let sleep_duration = Duration::from_secs(30);
                    sleep(sleep_duration).await;
                }
            }
        }
    });
    Ok(())
}

async fn ollama_unload_all_models(pool: &Pool) {
    let db_models = ollama_models_load(pool).await.unwrap();

    for db_model in db_models {
        let start = Utc::now();
        info!("unloading model {}", db_model.model);
        let res_unload = execute_ollama_unload(db_model.model.clone()).await;
        let finished = Utc::now();
        let duration = finished.signed_duration_since(start);
        info!(
            "done unloading model {}. started @ {}, finished @ {}, duration {}",
            db_model.model, start, finished, duration
        );

        if res_unload.is_err() {
            error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
            error!(
                "error unloading model {}, ignoring the error and continuing. err.msg {:?}",
                db_model.model,
                res_unload.unwrap_err()
            );
            error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
        }

        let sleep_duration = Duration::from_secs(1);
        sleep(sleep_duration).await;

        let finished2 = Utc::now();
        let duration2 = finished2.signed_duration_since(start);
        info!(
            "waiting after  unloading model {}. started @ {}, finished @ {}, duration {}, after wait {}, duration with wait {}",
            db_model.model, start, finished, duration, finished2, duration2
        );
    }
}

pub async fn start_queue() -> Result<(), OllamaChatError> {
    let mut r = QUEUE_RUNNING.lock().await;
    *r = true;
    Ok(())
}

pub async fn stop_queue() -> Result<(), OllamaChatError> {
    let mut r = QUEUE_RUNNING.lock().await;
    *r = false;
    Ok(())
}
