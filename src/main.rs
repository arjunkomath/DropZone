use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use inquire::Password;

use crate::store::Store;

mod settings;
mod store;

#[derive(Debug, Parser)]
#[command(name = "dropzone")]
#[command(bin_name = "dz")]
#[command(about = "A simple cloud clipboard", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Initialize dropzone with a redis connection string")]
    Init {},

    #[command(about = "Set a configuration value")]
    Config {
        #[clap(required = true, help = "Key")]
        key: String,
        #[clap(required = true, help = "Value")]
        value: String,
    },

    #[command(about = "Set a value, overwriting if it already exists")]
    Set {
        #[clap(required = true, help = "Key")]
        key: String,
        #[clap(required = true, help = "Value")]
        value: String,
    },

    #[command(about = "Get a value")]
    Get {
        #[clap(required = true, help = "Key")]
        key: String,
    },

    #[command(about = "Yank clipboard contents")]
    Yank {
        #[clap(required = true, help = "Key")]
        key: String,
    },

    #[command(about = "Reset and clear settings")]
    Reset {},
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Init {} => {
            let connection_string = Password::new("Redis connection string:")
                .without_confirmation()
                .prompt();

            match connection_string {
                Ok(connection_string) => {
                    if connection_string.is_empty() {
                        println!("You need to provide a connection string.");
                    } else {
                        settings::upsert("redis_connection_string", connection_string)?;
                    }
                }
                Err(_) => println!("An error happened when asking for your key, try again later."),
            }
        }

        Commands::Config { key, value } => {
            settings::upsert(&key, value)?;
        }

        Commands::Set { key, value } => {
            let store = Store::new()?;
            store.set(&key, value)?;
        }

        Commands::Get { key } => {
            let store = Store::new()?;
            let value = store.get(&key).context("Failed to get value")?;

            let mut ctx: ClipboardContext =
                ClipboardProvider::new().expect("Failed to access clipboard");

            if settings::fetch()?.get_bool("get_to_clipboard")? {
                ctx.set_contents(value.clone())
                    .expect("Failed to set clipboard contents");
                println!("value copied to clipboard");
            } else {
                println!("{}", value);
            }
        }

        Commands::Yank { key } => {
            let store = Store::new()?;

            let mut ctx: ClipboardContext =
                ClipboardProvider::new().expect("Failed to access clipboard");
            let value = ctx
                .get_contents()
                .expect("Failed to get clipboard contents");

            store.set(&key, value)?;
        }

        Commands::Reset {} => {
            settings::reset()?;
        }
    }

    Ok(())
}
