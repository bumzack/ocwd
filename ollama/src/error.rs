use reqwest::Error as ReqwestError;
use std::fmt;

#[derive(Debug)]
pub enum OllamaError {
    ReqwestErr(ReqwestError),
    AllTheOtherErrors(String),
    SerdeJsonErr(serde_json::error::Error),
}

impl fmt::Display for OllamaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OllamaError::ReqwestErr(err) => write!(f, "Reqwest error: {}", err),
            OllamaError::AllTheOtherErrors(err) => {
                write!(f, "data error. err {}", err)
            }
            OllamaError::SerdeJsonErr(err) => write!(f, "serde json error. err: {}", err),
        }
    }
}

impl From<ReqwestError> for OllamaError {
    fn from(error: ReqwestError) -> Self {
        OllamaError::ReqwestErr(error)
    }
}

impl From<serde_json::Error> for OllamaError {
    fn from(error: serde_json::Error) -> Self {
        OllamaError::SerdeJsonErr(error)
    }
}
//
// impl IntoResponse for OllamaError {
//     fn into_response(self) -> axum::response::Response {
//         let (status, message) = match self {
//             OllamaError::ReqwestErr(e) => {
//                 error!("reqwest error. err {:?}", e);
//                 (StatusCode::INTERNAL_SERVER_ERROR, "Reqwest error")
//             }
//
//             OllamaError::AllTheOtherErrors(msg) => {
//                 error!("data error to INTERNAL_ERROR. err: {:?}", msg);
//                 (StatusCode::INTERNAL_SERVER_ERROR, "internal data error")
//             }
//             OllamaError::SerdeJsonErr(e) => {
//                 error!("serde json to INTERNAL_ERROR. err: {:?}", e);
//                 error!("serde json: {}", e);
//                 (StatusCode::INTERNAL_SERVER_ERROR, "serde json error")
//             }
//         };
//         (status, message).into_response()
//     }
// }
