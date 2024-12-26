use axum::response::IntoResponse;
use deadpool_diesel::InteractError;
use diesel::result::Error as DieselErr;
use reqwest::{Error as ReqwestError, StatusCode};
use std::fmt;
use tracing::log::error;

#[warn(dead_code)]
#[derive(Debug)]
pub enum OllamaChatError {
    ReqwestErr(ReqwestError),
    DieselInteractError(InteractError),
    DieselError(DieselErr),
    DataError(String),
    DeadpoolErr(deadpool_diesel::Error),
    DeadpoolPoolError(deadpool_diesel::PoolError),
}

impl fmt::Display for OllamaChatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OllamaChatError::ReqwestErr(err) => write!(f, "Reqwest error: {}", err),
            OllamaChatError::DieselInteractError(err) => {
                write!(f, "Diesel interact error. err: {} ", err)
            }
            OllamaChatError::DieselError(err) => {
                write!(f, "Diesel error. err: {} ", err)
            }
            OllamaChatError::DataError(err) => {
                write!(f, "data error. err {}", err)
            }
            OllamaChatError::DeadpoolErr(err) => {
                write!(f, "deadpool error. err {}", err)
            }
            OllamaChatError::DeadpoolPoolError(err) => {
                write!(f, "deadpool pool error. err {}", err)
            }
        }
    }
}

impl From<ReqwestError> for OllamaChatError {
    fn from(error: ReqwestError) -> Self {
        OllamaChatError::ReqwestErr(error)
    }
}

impl From<InteractError> for OllamaChatError {
    fn from(error: InteractError) -> Self {
        OllamaChatError::DieselInteractError(error)
    }
}

impl From<DieselErr> for OllamaChatError {
    fn from(error: DieselErr) -> Self {
        OllamaChatError::DieselError(error)
    }
}

impl From<deadpool_diesel::PoolError> for OllamaChatError {
    fn from(error: deadpool_diesel::PoolError) -> Self {
        OllamaChatError::DeadpoolPoolError(error)
    }
}

impl From<deadpool_diesel::Error> for OllamaChatError {
    fn from(error: deadpool_diesel::Error) -> Self {
        OllamaChatError::DeadpoolErr(error)
    }
}

impl IntoResponse for OllamaChatError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            OllamaChatError::ReqwestErr(e) => {
                error!("reqwest error. err {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Reqwest error")
            }
            OllamaChatError::DieselInteractError(e) => {
                error!("diesel interact error. err {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "diesel interact error")
            }
            OllamaChatError::DieselError(e) => {
                error!("diesel error. err {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "diesel error")
            }
            OllamaChatError::DataError(msg) => {
                error!("data error to INTERNAL_ERROR. err: {:?}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "internal data error")
            }
            OllamaChatError::DeadpoolErr(e) => {
                error!("deadpool error to INTERNAL_ERROR. err: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "deadpool error")
            }
            OllamaChatError::DeadpoolPoolError(e) => {
                error!("deadpool pool error to INTERNAL_ERROR. err: {:?}", e);
                error!("deadpool pool error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "deadpool pool error")
            }
        };
        (status, message).into_response()
    }
}
