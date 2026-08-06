use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use craft::{manifest_contents, valid_name, DEFAULT_MAIN, VERSION};

#[derive(Parser)]
#[command(
    name = "craft",
    version = VERSION,
    about = "An experimental package manager for single-file tools."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new craft package.
    New {
        /// Name of the package to create.
        name: String,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Command::New { name } => match new(&name) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("craft: {error}");
                ExitCode::FAILURE
            }
        },
    }
}

fn new(name: &str) -> Result<(), String> {
    if !valid_name(name) {
        return Err(
            "invalid package name; use lowercase letters, digits, dashes, or underscores"
                .to_string(),
        );
    }
    let dir = PathBuf::from(name);
    if dir.exists() {
        return Err(format!("{name} already exists"));
    }
    fs::create_dir_all(dir.join("src"))
        .map_err(|error| format!("cannot create directory: {error}"))?;
    fs::write(dir.join("Craft.toml"), manifest_contents(name))
        .map_err(|error| format!("cannot write manifest: {error}"))?;
    fs::write(dir.join("src/main.rs"), DEFAULT_MAIN)
        .map_err(|error| format!("cannot write source: {error}"))?;
    println!("created package {name}");
    Ok(())
}
