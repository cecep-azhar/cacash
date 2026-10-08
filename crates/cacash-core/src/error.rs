use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CashError {
    #[error("CASH-DB-001: Database error: {0}")]
    Db(String),

    #[error("CASH-AUTH-001: Authentication error: {0}")]
    Auth(String),

    #[error("CASH-NOTFOUND-001: Resource not found: {0}")]
    NotFound(String),

    #[error("CASH-VAL-001: Validation error: {0}")]
    Validation(String),

    #[error("CASH-AI-001: AI error: {0}")]
    Ai(String),

    #[error("CASH-IO-001: IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<rusqlite::Error> for CashError {
    fn from(err: rusqlite::Error) -> Self {
        CashError::Db(err.to_string())
    }
}

impl Serialize for CashError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
