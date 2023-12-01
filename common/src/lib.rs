use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct CommonCli {
    #[arg(short, long)]
    pub file: PathBuf,
}
