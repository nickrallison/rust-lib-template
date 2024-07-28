use clap::Parser;

use std::path::PathBuf;

/// program to decompile an executable, dll, or object file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// file to decompile
    #[arg(short, long)]
    path: PathBuf,
}

impl Args {
    pub fn get_args() -> PathBuf {
        let args = Args::parse();
        PathBuf::from(args.path)
    }
}
