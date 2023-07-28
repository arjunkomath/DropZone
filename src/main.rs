use std::{fs, path::Path};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use config::Config;
use directories::ProjectDirs;

#[derive(Debug, Parser)]
#[command(name = "dropzone")]
#[command(bin_name = "dropzone")]
#[command(about = "Simple cloud clipboard", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command()]
    Config {
        #[clap(required = true, help = "Key")]
        key: String,
        #[clap(required = true, help = "Value")]
        value: String,
    },
}

fn main() -> Result<()> {
    let proj_dirs =
        ProjectDirs::from("com", "Techulus", "DropZone").context("Failed to get project dirs")?;

    if fs::metadata(proj_dirs.config_dir()).is_err() {
        fs::create_dir(proj_dirs.config_dir())?;
    }

    let settings_file = proj_dirs.config_dir().join("Settings.toml");

    if !Path::new(&settings_file).exists() {
        fs::write(&settings_file, "")?;
    }

    let settings_file_path = settings_file
        .canonicalize()
        .with_context(|| "Failed to canonicalize config path")
        .and_then(|x| {
            x.to_str()
                .context("Failed to parse patht to string")
                .map(|s| s.to_string())
        })?;

    let settings = Config::builder()
        .set_default("Hello", "World")?
        .add_source(config::File::new(
            settings_file_path.as_str(),
            config::FileFormat::Toml,
        ))
        .build()?;

    println!("debug: {:?}", settings);

    let args = Cli::parse();

    match args.command {
        Commands::Config { key, value } => {
            println!("key: {}, value: {}", key, value);
        }
    }

    Ok(())
}
