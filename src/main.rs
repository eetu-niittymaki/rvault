use clap::Parser;
use cli::{Cli, Commands};
use crate::config::get_db_path;

mod cli;
mod commands;
mod config;
mod db;
mod crypto;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let db_path = get_db_path();

    match &cli.command {
        Some(Commands::Create(_)) => {
            let _pool = commands::create::create().await?;
        }
        Some(Commands::New(cmd)) => {
            check_db_exists();
            let pool = db::connect(&db_path).await?;
            commands::new::new(cmd.clone(), &pool).await;
        }
        Some(Commands::Update(cmd)) => {
            check_db_exists();
            let pool = db::connect(&db_path).await?;
            commands::update::update(cmd.clone(), &pool).await;
        }
        Some(Commands::Delete(cmd)) => {
            check_db_exists();
            let pool = db::connect(&db_path).await?;
            commands::delete::delete(cmd.clone(), &pool).await;
        }
        Some(Commands::All(cmd)) => {
            check_db_exists();
            let pool = db::connect(&db_path).await?;
            commands::all::all(cmd.clone(), &pool).await;
        }
        Some(Commands::Get(cmd)) => {
            check_db_exists();
            let pool = db::connect(&db_path).await?;
            commands::get::get(cmd.clone(), &pool).await;
        }
        Some(Commands::Version) => {
            commands::version::version().await;
        }
        None => {}
    }

    Ok(())
}

fn check_db_exists() {
    let db_path = get_db_path();
    if !db_path.exists() {
        println!("Database not found. Run `create` first.");
        std::process::exit(0);
    }
}