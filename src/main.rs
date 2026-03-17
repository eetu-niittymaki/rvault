pub mod crypto;
mod cli;
mod commands;
mod config;

use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Create(cmd)) => {
            commands::create::create(cmd).await;
        }
        Some(Commands::Add(cmd)) => {
            commands::add::add(cmd).await;
        }
        Some(Commands::Update(cmd)) => {
            commands::update::update(cmd).await;
        }
        Some(Commands::Delete(cmd)) => {
            commands::delete::delete(cmd).await;
        }
        Some(Commands::All(cmd)) => {
            commands::all::all(cmd).await;
        }
        None => {}
    }
}
