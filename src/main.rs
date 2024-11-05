
mod args;
mod error;
mod prelude;

use crate::prelude::*;

use env_logger;
use log::{info, trace, warn};

use std::path::PathBuf;

fn main() -> Result<()> {
    env_logger::init();
    let path: PathBuf = crate::args::Args::get_args();
    println!("path recieved as argument: {}", path.display());
    Ok(())
}
