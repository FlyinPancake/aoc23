use color_eyre::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use clap::Parser;

#[derive(Parser)]
pub struct CommonCli {
    #[arg(short, long)]
    pub file: PathBuf,
}

pub fn get_file(filename: PathBuf) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: std::result::Result<Vec<String>, _> = reader.lines().collect();
    Ok(lines?)
}

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    if dst.as_ref().exists() {
        let dst_metadata = std::fs::metadata(&dst)?;
        if dst_metadata.is_dir() {
            std::fs::remove_dir_all(&dst)?;
        }
    }
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let entry_type = entry.file_type()?;
        if entry_type.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}
