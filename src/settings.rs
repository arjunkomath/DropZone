use std::{fs, path::Path};

use anyhow::{Context, Ok, Result};
use config::Config;
use directories::ProjectDirs;

fn get_file_path() -> Result<String> {
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

    Ok(settings_file_path)
}

pub fn fetch() -> Result<Config> {
    let settings_file_path = get_file_path()?;

    let settings = Config::builder()
        .set_default("expiry", 300)? // 5 minutes
        .set_default("get_to_clipboard", false)? // Get will set the value in clipboard
        .add_source(config::File::new(
            settings_file_path.as_str(),
            config::FileFormat::Toml,
        ))
        .build()?;

    Ok(settings)
}

pub fn upsert(key: &str, value: String) -> Result<()> {
    if key != "expiry" && key != "get_to_clipboard" {
        return Err(anyhow::anyhow!("Invalid key"));
    }

    let settings_file = fs::read_to_string(get_file_path()?)?;
    let mut settings_file: toml::Value = settings_file.parse()?;

    let updated_settings = if let toml::Value::Table(ref mut table) = settings_file {
        table.insert(key.to_string(), toml::Value::String(value));

        toml::to_string(&table)?
    } else {
        let mut new_settings = toml::map::Map::new();
        new_settings.insert(key.to_string(), toml::Value::String(value));

        toml::to_string(&toml::Value::Table(new_settings))?
    };

    fs::write(get_file_path()?, updated_settings)?;

    Ok(())
}

pub fn reset() -> Result<()> {
    let settings_file_path = get_file_path()?;
    fs::remove_file(settings_file_path)?;

    Ok(())
}
