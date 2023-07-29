use anyhow::Result;
use clap::{Parser, Subcommand};
use inquire::Password;

use crate::store::Store;

mod settings;
mod store;

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
    Init {},

    #[command()]
    Config {
        #[clap(required = true, help = "Key")]
        key: String,
        #[clap(required = true, help = "Value")]
        value: String,
    },

    #[command()]
    Set {
        #[clap(required = true, help = "Key")]
        key: String,
        #[clap(required = true, help = "Value")]
        value: String,
    },

    #[command()]
    Get {
        #[clap(required = true, help = "Key")]
        key: String,
    },
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
            let value = store.get(&key)?;

            println!("{}", value);
        }
    }

    Ok(())
}
