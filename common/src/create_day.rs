use std::{fs, path::PathBuf};

use aoc_client::AocClient;
use clap::{Parser, Subcommand};
use color_eyre::{eyre::anyhow, Result};
use common::copy_dir_all;

use markdown::{to_mdast, ParseOptions};

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    day: u32,
    #[command(subcommand)]
    command: Commands,
}
#[derive(Debug, Subcommand)]
enum Commands {
    Init,
    UpdatePuzzle,
    Vscode,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    let cargo_manifest_dir = &PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = cargo_manifest_dir
        .parent()
        .ok_or(anyhow!("No parent for cargo toml dir"))?;

    let day_dir = workspace_dir.join(format!("day_{}", &args.day));
    let day_template_dir = workspace_dir.join("day_template");
    let puzzle_file = day_dir.join("README.md");
    let inputs_dir = day_dir.join("inputs");
    let personal_input_file = inputs_dir.join("full.txt");
    let workspace_manifest = workspace_dir.join("Cargo.toml");
    let day_cargo_manifest_path = day_dir.join("Cargo.toml");
    let vscode_dir = workspace_dir.join(".vscode");
    let aoc = make_aoc_client(&args, &puzzle_file, &personal_input_file)?;
    if !aoc.day_unlocked() {
        fs_extra::dir::remove(day_dir)?;
        return Err(anyhow!("🎄 day not unlocked"));
    }

    match args.command {
        Commands::Init => {
            copy_template_day(day_template_dir, &day_dir)?;
            save_aoc_files(aoc)?;
            update_workspace_manifest(workspace_manifest, &args)?;
            update_day_manifest(day_cargo_manifest_path, &args)?;
            extract_example_inputs(puzzle_file, inputs_dir)?;
            vscode_conventional_commits(&vscode_dir, &args)?;
            vscode_create_tasks(&vscode_dir, args)?;
        }

        Commands::UpdatePuzzle => {
            clear_aoc_files(&puzzle_file, &personal_input_file)?;
            save_aoc_files(aoc)?;
            extract_example_inputs(puzzle_file, inputs_dir)?;
        }

        Commands::Vscode => {
            vscode_conventional_commits(&vscode_dir, &args)?;
            vscode_create_tasks(&vscode_dir, args)?;
        }
    }

    Ok(())
}

static CHRISTMAS_EMOJIS: [&str; 25] = [
    "🎄",
    "🎅",
    "🤶",
    "🦌",
    "🛷",
    "🎁",
    "🔔",
    "🕯️",
    "🧦",
    "🎉",
    "🍪",
    "🥛",
    "🍬",
    "🎶",
    "⛄",
    "❄️",
    "🌟",
    "🌲",
    "🕰️",
    "🍾",
    "🎂",
    "🤗",
    "🎊",
    "🧑‍🎄",
    "🕊️",
];

fn vscode_create_tasks(vscode_dir: &PathBuf, args: Args) -> Result<(), color_eyre::eyre::Error> {
    let vscode_launch_path = vscode_dir.join("launch.json");
    let vscode_launch = fs::read_to_string(&vscode_launch_path)?;
    let vscode_launch = json5::from_str::<serde_json::Value>(&vscode_launch)?;
    Ok(
        if let serde_json::Value::Object(mut launch) = vscode_launch {
            let launch_configurations = launch
                .get_mut("configurations")
                .ok_or(anyhow!("No configurations"))?;
            if let serde_json::Value::Array(arr) = launch_configurations {
                // remove the existing day config if it exists
                arr.retain(|config| {
                    if let serde_json::Value::Object(config) = config {
                        if let Some(serde_json::Value::String(name)) = config.get("name") {
                            return !name.contains(&format!("day_{}", &args.day));
                        }
                    }
                    true
                });
                let day_lib_config = serde_json::json!({
                    "type": "lldb",
                    "request": "launch",
                    "name": format!("{}  Debug unit tests in day_{}",CHRISTMAS_EMOJIS[args.day as usize], &args.day),
                    "cargo": {
                        "args": [
                            "test",
                            "--no-run",
                            "--package",
                            format!("day_{}", &args.day),
                            "--lib",
                        ],
                        "filter": {
                            "name": "solution",
                            "kind": "lib"
                        }
                    },
                    "args": [],
                    "cwd": "${workspaceFolder}",

                });
                arr.push(day_lib_config);
            }
            let launch = json5::to_string(&launch)?;
            fs::write(&vscode_launch_path, launch)?;
        },
    )
}

fn vscode_conventional_commits(
    vscode_dir: &PathBuf,
    args: &Args,
) -> Result<(), color_eyre::eyre::Error> {
    let vscode_settings_path = vscode_dir.join("settings.json");
    let vscode_settings = fs::read_to_string(&vscode_settings_path)?;
    let vscode_settings = json5::from_str::<serde_json::Value>(&vscode_settings)?;
    Ok(
        if let serde_json::Value::Object(mut settings) = vscode_settings {
            let cc_scopes = settings.get_mut("conventionalCommits.scopes");
            match cc_scopes {
                Some(serde_json::Value::Array(arr)) => {
                    if !arr.contains(&serde_json::Value::String(format!("day_{}", &args.day))) {
                        arr.push(serde_json::Value::String(format!("day_{}", &args.day)));
                    }
                }
                _ => {
                    settings.insert(
                        "conventionalCommits.scopes".to_string(),
                        serde_json::Value::Array(vec![serde_json::Value::String(format!(
                            "day_{}",
                            &args.day
                        ))]),
                    );
                }
            }

            let settings = json5::to_string(&settings)?;
            fs::write(&vscode_settings_path, settings)?;
        },
    )
}

fn clear_aoc_files(
    puzzle_file: &PathBuf,
    personal_input_file: &PathBuf,
) -> Result<(), color_eyre::eyre::Error> {
    fs::remove_file(puzzle_file)?;
    fs::remove_file(&personal_input_file)?;
    Ok(())
}

fn extract_example_inputs(
    puzzle_file: PathBuf,
    inputs_dir: PathBuf,
) -> Result<(), color_eyre::eyre::Error> {
    let puzzle_file_contents = std::fs::read_to_string(puzzle_file)?;
    let puzzle_desc = to_mdast(&puzzle_file_contents, &ParseOptions::default())
        .map_err(|_| anyhow!("Markdown ast is not ok"))?;
    Ok(
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
        },
    )
}

fn update_day_manifest(
    day_cargo_manifest_path: PathBuf,
    args: &Args,
) -> Result<(), color_eyre::eyre::Error> {
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
    Ok(())
}

fn copy_template_day(
    day_template_dir: PathBuf,
    day_dir: &PathBuf,
) -> Result<(), color_eyre::eyre::Error> {
    copy_dir_all(&day_template_dir, day_dir)?;
    Ok(())
}

fn update_workspace_manifest(
    workspace_manifest: PathBuf,
    args: &Args,
) -> Result<(), color_eyre::eyre::Error> {
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
    Ok(())
}

fn save_aoc_files(aoc: AocClient) -> Result<(), color_eyre::eyre::Error> {
    aoc.save_puzzle_markdown()?;
    aoc.save_input()?;
    Ok(())
}

fn make_aoc_client(
    args: &Args,
    puzzle_file: &PathBuf,
    personal_input_file: &PathBuf,
) -> Result<AocClient, color_eyre::eyre::Error> {
    let aoc = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2023)?
        .day(args.day)?
        .puzzle_filename(puzzle_file)
        .input_filename(personal_input_file)
        .build()?;
    Ok(aoc)
}
