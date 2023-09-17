pub mod authentication;

/// ReciteError enumerates all possible errors returned by this library.
#[derive(thiserror::Error, Debug)]
pub enum RibasomeServerError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    MigrateError(#[from] sqlx::migrate::MigrateError),
    #[error(transparent)]
    AxumError(#[from] axum::Error),
    #[error(transparent)]
    EnvVariableError(#[from] std::env::VarError),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    CsvError(#[from] csv::Error),
    #[error(transparent)]
    JsonReadError(#[from] crate::csv_ops::JsonReadError),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

pub type Result<T> = color_eyre::eyre::Result<T, RibasomeServerError>;
