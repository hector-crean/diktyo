// use std::fmt;

/// ReciteError enumerates all possible errors returned by this library.
#[derive(thiserror::Error, Debug)]
pub enum BibeDrinkDispenserError {
    #[error(transparent)]
    AxumError(axum::Error),
    #[error(transparent)]
    ReqError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error(transparent)]
    EnvVariableError(#[from] std::env::VarError),
    #[error(transparent)]
    GpioError(#[from] rppal::gpio::Error),
}

// impl fmt::Display for RaspberryBibeError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self)
//     }
// }

pub type Result<T> = color_eyre::eyre::Result<T, BibeDrinkDispenserError>;
