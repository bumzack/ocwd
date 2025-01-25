use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::fmt;

#[warn(dead_code)]
#[derive(Debug)]
pub enum WebshopError {
    SerdeJsonErr(serde_json::error::Error),
}

impl fmt::Display for WebshopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebshopError::SerdeJsonErr(err) => write!(f, "serde json error. err: {}", err),
        }
    }
}

impl From<serde_json::Error> for WebshopError {
    fn from(error: serde_json::Error) -> Self {
        WebshopError::SerdeJsonErr(error)
    }
}

impl IntoResponse for WebshopError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            WebshopError::SerdeJsonErr(e) => {
                println!("serde json: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "serde json error")
            }
        };
        (status, message).into_response()
    }
}
