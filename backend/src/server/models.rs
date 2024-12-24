use std::fmt::{Display, Formatter};

pub struct Config {
    pub database_url: String,
    pub ollama_url: String,
}

#[derive(Debug, PartialEq)]
pub enum QueueState {
    Enqueued,
    Processing,
    Finished,
    Error,
}

impl Display for QueueState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            QueueState::Finished => write!(f, "finished"),
            QueueState::Enqueued => write!(f, "enqueued"),
            QueueState::Processing => write!(f, "processing"),
            QueueState::Error => write!(f, "error"),
        }
    }
}
