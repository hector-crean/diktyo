/// ReciteError enumerates all possible errors returned by this library.
#[derive(thiserror::Error, Debug)]
pub enum BibeDesktopClientError {
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

pub type Result<T> = color_eyre::eyre::Result<T, BibeDesktopClientError>;
