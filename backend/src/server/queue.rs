use crate::common::db_chat::ollama_chat_insert;
use crate::common::db_model::ollama_model_load_by_id;
use crate::common::db_prompt::ollama_prompt_load_by_id;
use crate::common::db_queue::{ollama_queue_next, ollama_queue_update_state};
use crate::server::models::QueueState;
use crate::server::ollamachat_error::OllamaChatError;
use crate::{CONFIG, QUEUE_RUNNING};
use chrono::Utc;
use deadpool_diesel::postgres::Pool;
use ollama::api::OllamaImpl;
use ollama::models::{ChatRequest, ChatRequestOptions, Ollama};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{error, info};

pub async fn run_queue(pool: Pool) -> Result<(), OllamaChatError> {
    tokio::spawn(async move {
        let p = pool.clone();
        let o = Ollama::new(CONFIG.ollama_url.clone()).expect("should create an ollama thingi");

        loop {
            let now = Utc::now();

            let running = QUEUE_RUNNING.lock().await;

            info!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
            info!("processing enqueue @ {:?}. running {:?}", now, *running);
            info!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

            if !*running {
                let sleep_duration = Duration::from_secs(60);

                info!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
                info!(
                    "processing enqueue NOT running @ {:?}. waiting for {:?}",
                    now, sleep_duration
                );
                info!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

                sleep(sleep_duration).await;
                continue;
            } else {
                info!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
                info!("processing enqueue  running @ {:?}", now);
                info!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
            }

            let next = ollama_queue_next(&p).await.expect("enqueue next failed");

            match next {
                Some(db_queue) => {
                    let db_model = ollama_model_load_by_id(&pool, db_queue.model_id)
                        .await
                        .expect("model element not found")
                        .expect("model element not found");

                    info!("start unloading models we don't need  @ {}", Utc::now());
                    ollama_unload_all_models_except(&db_model.model).await;
                    info!("finished unloading models we don't need @ {}", Utc::now());

                    let db_prompt = ollama_prompt_load_by_id(&pool, db_queue.prompt_id)
                        .await
                        .expect("enqueue element not found")
                        .expect("enqueue element not found");

                    let opt = ChatRequestOptions {
                        temperature: Some(db_queue.temperature as f32),
                        num_ctx: Some(db_queue.num_ctx as u32),
                        seed: Some(db_queue.seed as u32),
                        top_k: Some(db_queue.top_k as f32),
                        top_p: Some(db_queue.top_p as f32),
                        ..Default::default()
                    };

                    let request = ChatRequest {
                        model: db_model.model,
                        prompt: db_prompt.prompt.clone(),
                        stream: false,
                        options: Some(opt),
                        messages: None,
                        format: None,
                        tools: None,
                    };
                    info!("queue_element={:?}", db_queue);

                    let start = Instant::now();

                    let db_ollama_queue_update =
                        ollama_queue_update_state(&pool, db_queue.id, QueueState::Processing).await;

                    match db_ollama_queue_update {
                        Ok(res) => {
                            info!(
                                "successfully updated ollama enqueue status. id {}",
                                res.id
                            );
                        }
                        Err(e) => {
                            error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
                            error!(
                                "error updating enqueue in DB, enqueue.id {} {:?}",
                                db_queue.id, e
                            );
                            error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
                        }
                    }

                    info!("------------------------------------------------------------------------------------------");
                    info!("starting request to Ollama.");
                    let ollama_response = o.chat(&request).await;
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
                                        "successfully updated ollamaold enqueue status. id {}",
                                        res.id
                                    );
                                }
                                Err(e) => {
                                    error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
                                    error!(
                                        "error updating enqueue in DB, enqueue.id {} {:?}",
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
                                    info!(
                                        "successfully inserted new ollamaold chat. id {}",
                                        res.id
                                    );
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
                            error!("error executing request by ollamaold server {:?}", e);
                            error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
                            let db_ollama_queue_update =
                                ollama_queue_update_state(&pool, db_queue.id, QueueState::Error)
                                    .await;

                            match db_ollama_queue_update {
                                Ok(res) => {
                                    info!("successfully updated enqueue status. id {}", res.id);
                                }
                                Err(e) => {
                                    error!("EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEvEEEEEEEEEEEEEEEEEEEEEEEE");
                                    error!(
                    "error updating enqueue status in DB. enqueue.id {}, error: {:?}",
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
                    let sleep_duration = Duration::from_secs(5);
                    sleep(sleep_duration).await;
                }
            }
        }
    });
    Ok(())
}

async fn ollama_unload_all_models_except(model: &str) {
    let o = Ollama::new(CONFIG.ollama_url.clone()).expect("should create an ollama thingi");
    let db_models = o.loaded_models().await;

    let db_models = match db_models {
        Ok(db_models) => db_models,
        Err(e) => {
            error!("error downloading loaded models. err {}", e);
            return;
        }
    };

    // unload all models except the one
    for db_model in db_models.iter().filter(|m| m.model != model) {
        let start = Utc::now();
        info!("unloading model {}", db_model.model);
        let res_unload = o.unload(&db_model.model).await;
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

// untested ¯\_(ツ)_/¯
pub async fn start_queue() -> Result<(), OllamaChatError> {
    let mut r = QUEUE_RUNNING.lock().await;
    *r = true;
    Ok(())
}

// untested ¯\_(ツ)_/¯
pub async fn stop_queue() -> Result<(), OllamaChatError> {
    let mut r = QUEUE_RUNNING.lock().await;
    *r = false;
    Ok(())
}
