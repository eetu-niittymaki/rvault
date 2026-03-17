use clap::Parser;
use cli::{Cli, Commands};
use crate::config::{DB_PATH, db_exists};

mod cli;
mod commands;
mod config;
mod db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Create(_)) => {
            let _pool = commands::create::create().await?;
            println!("Database ready.");
        }
        Some(Commands::New(cmd)) => {
            ensure_db_exists();
            let pool = db::connect(DB_PATH).await?;
            commands::new::new(cmd.clone(), &pool).await;
        }
        Some(Commands::Update(cmd)) => {
            ensure_db_exists();
            let pool = db::connect(DB_PATH).await?;
            commands::update::update(cmd.clone(), &pool).await;
        }
        Some(Commands::Delete(cmd)) => {
            ensure_db_exists();
            let pool = db::connect(DB_PATH).await?;
            commands::delete::delete(cmd.clone(), &pool).await;
        }
        Some(Commands::All(cmd)) => {
            ensure_db_exists();
            let pool = db::connect(DB_PATH).await?;
            commands::all::all(cmd.clone(), &pool).await;
        }
        Some(Commands::Get(cmd)) => {
            ensure_db_exists();
            let pool = db::connect(DB_PATH).await?;
            commands::get::get(cmd.clone(), &pool).await;
        }
        None => {}
    }

    Ok(())
}

fn ensure_db_exists() {
    if !db_exists() {
        println!("Database not found. Run `create` first.");
        std::process::exit(1);
    }
}