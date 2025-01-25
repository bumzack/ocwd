use deadpool_diesel::InteractError;
use diesel::result::Error as DieselErr;
use reqwest::Error as ReqwestError;
use std::fmt;

#[warn(dead_code)]
#[derive(Debug)]
pub enum WebshopError {
    ReqwestErr(ReqwestError),
    DieselInteractError(InteractError),
    DieselError(DieselErr),
    DeadpoolErr(deadpool_diesel::Error),
    DeadpoolPoolError(deadpool_diesel::PoolError),
    SerdeJsonErr(serde_json::error::Error),
}

impl fmt::Display for WebshopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebshopError::ReqwestErr(err) => write!(f, "Reqwest error: {}", err),
            WebshopError::DieselInteractError(err) => {
                write!(f, "Diesel interact error. err: {} ", err)
            }
            WebshopError::DieselError(err) => {
                write!(f, "Diesel error. err: {} ", err)
            }
            WebshopError::DeadpoolErr(err) => {
                write!(f, "deadpool error. err {}", err)
            }
            WebshopError::DeadpoolPoolError(err) => {
                write!(f, "deadpool pool error. err {}", err)
            }
            WebshopError::SerdeJsonErr(err) => write!(f, "serde json error. err: {}", err),
        }
    }
}

impl From<ReqwestError> for WebshopError {
    fn from(error: ReqwestError) -> Self {
        WebshopError::ReqwestErr(error)
    }
}

impl From<InteractError> for WebshopError {
    fn from(error: InteractError) -> Self {
        WebshopError::DieselInteractError(error)
    }
}

impl From<DieselErr> for WebshopError {
    fn from(error: DieselErr) -> Self {
        WebshopError::DieselError(error)
    }
}

impl From<deadpool_diesel::PoolError> for WebshopError {
    fn from(error: deadpool_diesel::PoolError) -> Self {
        WebshopError::DeadpoolPoolError(error)
    }
}

impl From<deadpool_diesel::Error> for WebshopError {
    fn from(error: deadpool_diesel::Error) -> Self {
        WebshopError::DeadpoolErr(error)
    }
}

impl From<serde_json::Error> for WebshopError {
    fn from(error: serde_json::Error) -> Self {
        WebshopError::SerdeJsonErr(error)
    }
}
