use std::path::{Path, PathBuf};

use aoc_client::AocClient;
use clap::Parser;
use color_eyre::{eyre::anyhow, Result};
use markdown::{to_mdast, ParseOptions};

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    day: u32,
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
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

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    let cargo_manifest_dir = &PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = cargo_manifest_dir
        .parent()
        .ok_or(anyhow!("No parent for cargo toml dir"))?;

    let day_dir = workspace_dir.join(format!("day_{}", args.day));
    let day_template_dir = workspace_dir.join("day_template");
    copy_dir_all(&day_template_dir, &day_dir)?;
    let puzzle_file = day_dir.join("README.md");
    let inputs_dir = day_dir.join("inputs");
    let aoc = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2023)?
        .day(args.day)?
        .puzzle_filename(&puzzle_file)
        .input_filename(&inputs_dir.join("full.txt"))
        .build()?;
    if !aoc.day_unlocked() {
        fs_extra::dir::remove(day_dir)?;
        return Err(anyhow!("🎄 day not unlocked"));
    }
    aoc.save_puzzle_markdown()?;
    aoc.save_input()?;

    let workspace_manifest = workspace_dir.join("Cargo.toml");
    let mut workspace_manifest_contents =
        std::fs::read_to_string(&workspace_manifest)?.parse::<toml::Table>()?;
    let workspace_settings = workspace_manifest_contents
        .get_mut("workspace")
        .ok_or(anyhow!("No workspace configuration found"))?;
    let workspace_members = workspace_settings
        .get_mut("members")
        .ok_or(anyhow!("No workspace members found"))?;

    if let toml::Value::Array(arr) = workspace_members {
        let day_value = toml::Value::String(format!("day_{}", args.day).to_string());
        if !arr.contains(&day_value) {
            arr.push(day_value)
        }
    }
    let workspace_manifest_string = toml::to_string_pretty(&workspace_manifest_contents)?;
    std::fs::write(&workspace_manifest, workspace_manifest_string)?;

    let day_cargo_manifest_path = day_dir.join("Cargo.toml");
    let mut day_cargo_manifest_contents =
        std::fs::read_to_string(&day_cargo_manifest_path)?.parse::<toml::Table>()?;
    if let toml::Value::Table(table) = day_cargo_manifest_contents
        .get_mut("package")
        .ok_or(anyhow!("No pakcage section"))?
    {
        table.insert(
            "name".to_string(),
            toml::Value::String(format!("day_{}", args.day)),
        );
    }
    std::fs::write(
        &day_cargo_manifest_path,
        toml::to_string_pretty(&day_cargo_manifest_contents)?,
    )?;

    let puzzle_file_contents = std::fs::read_to_string(puzzle_file)?;
    let puzzle_desc = to_mdast(&puzzle_file_contents, &ParseOptions::default())
        .map_err(|_| anyhow!("Markdown ast is not ok"))?;
    if let markdown::mdast::Node::Root(markdown::mdast::Root { children, .. }) = puzzle_desc {
        children
            .into_iter()
            .filter_map(|node| {
                if let markdown::mdast::Node::Code(code) = node {
                    Some(code)
                } else {
                    None
                }
            })
            .enumerate()
            .try_for_each(|(idx, code)| -> Result<()> {
                let filename = &inputs_dir.join(format!("example_{}.txt", idx + 1));
                std::fs::write(filename, code.value)?;
                Ok(())
            })?;

        let gitkeep_file = inputs_dir.join(".gitkeep");
        if gitkeep_file.is_file() {
            std::fs::remove_file(gitkeep_file)?;
        }
    }

    Ok(())
}
