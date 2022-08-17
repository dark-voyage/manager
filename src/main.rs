mod assets;
mod download;
mod execute;
mod init;
mod launch;

use clap::{Parser, Subcommand};

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

    /// Manage the systemd service
    Service,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Upgrade { version } => {
            println!("Upgrading minecraft server to {} version", version);
        }
        Commands::Start => {
            println!("Starting the server");
            launch::start().await;
        }
        Commands::Init => {
            println!("Initializing the server");
            init::bootstrap().await;
        }
        Commands::Service => {}
    }
}
