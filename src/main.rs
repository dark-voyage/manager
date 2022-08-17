extern crate core;

mod assets;
mod backup;
mod download;
mod execute;
mod init;
mod launch;
mod service;

use clap::{Parser, Subcommand};

const WIP: &str = "Oops, I forgot that this feature is still WIP";

/// UwUssimo's Minecraft Server Manager
#[derive(Debug, Parser)]
#[clap(name = "uwucrafter")]
#[clap(bin_name = "uwucrafter")]
#[clap(about = "UwUssimo's Minecraft Server Manager", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Upgrade the minecraft server
    #[clap(arg_required_else_help = true)]
    Upgrade {
        /// Version to upgrade to
        version: String,
    },
    /// Start the minecraft server
    Start,

    /// Initialize server assets
    Init,

    /// Backup the server to GitHub
    Backup,

    /// Manage the systemd service
    Service,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Upgrade { version } => {
            println!("Upgrading minecraft server to {} version", version);
            println!("{}", WIP);
        }
        Commands::Start => {
            println!("Starting the server");
            launch::start().await;
        }
        Commands::Init => {
            println!("Initializing the server");
            init::bootstrap().await;
        }
        Commands::Service => {
            println!("{}", WIP);
        }
        Commands::Backup => {
            println!("Backing up the assets");
            backup::upload(
                match std::env::var("UWU_BOT_TOKEN") {
                    Ok(v) => v,
                    Err(_) => "".to_string(),
                }
                .as_str(),
                match std::env::var("UWU_CHAT_ID") {
                    Ok(v) => v,
                    Err(_) => "-1001576728891".to_string(),
                }
                .parse()
                .unwrap(),
            )
            .await;
        }
    }
}
