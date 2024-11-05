//! Main Crate Error

use color_eyre::eyre::ErrReport;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[allow(dead_code)]
    #[error("Generic {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    ColorEyre(#[from] ErrReport),
}
