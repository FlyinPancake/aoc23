use std::path::PathBuf;

use color_eyre::Result;

use common::get_file;

fn main() -> Result<()> {
    color_eyre::install()?;
    let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file = get_file(cargo_manifest_dir.join("inputs/full.txt"))?;
    let sol = solution::solve_task_one(file)?;
    println!("{sol}");
    Ok(())
}
