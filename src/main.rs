#![deny(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

mod args;
mod error;
mod prelude;

#[allow(unused_imports)]
use crate::prelude::*;

use env_logger;
#[allow(unused_imports)]
use log::{info, trace, warn};

use std::path::PathBuf;

fn main() -> Result<()> {
    env_logger::init();
    let path: PathBuf = crate::args::Args::get_args();
    println!("path recieved as argument: {}", path.display());
    Ok(())
}
