//! Crate Prelude

pub use crate::error::Error;

#[allow(dead_code)]
pub type Result<T> = core::result::Result<T, Error>;

// preference items

#[allow(unused_imports)]
pub use std::format as f;
