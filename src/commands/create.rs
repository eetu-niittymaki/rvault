use sqlx::sqlite::{SqliteConnectOptions, SqliteQueryResult};
use sqlx::{ConnectOptions, SqliteConnection, query};
use std::path::Path;
use std::str::FromStr;
use std::{fs, io};

use crate::cli::CreateCommand;
use crate::config::DB_PATH;

async fn create_encrypted_database(database_path: &str, password: &str) -> anyhow::Result<()> {
    let _ = SqliteConnectOptions::from_str(&database_path)?
        .pragma("key", password.to_owned())
        .create_if_missing(true)
        .connect()
        .await?;

    Ok(())
}

async fn create_table(conn: &mut SqliteConnection) -> anyhow::Result<SqliteQueryResult> {
    query(
        "CREATE TABLE IF NOT EXISTS Passwords(
            id       INTEGER PRIMARY KEY NOT NULL,
            name     TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL UNIQUE
        );",
    )
    .execute(conn)
    .await
    .map_err(Into::into)
}

pub async fn create(cmd: CreateCommand) {
    let db_path = Path::new(DB_PATH);

    if db_path.exists() {
        let mut input = String::new();
        println!("Database already exists, delete and create new [y/n]?");
        println!("Deleting will destroy all saved data!");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                fs::remove_file(db_path).unwrap();
            }
            _ => return,
        }
    }

    let _ = create_encrypted_database(&DB_PATH, &cmd.password).await;
    let mut conn = SqliteConnectOptions::from_str(&DB_PATH)
        .unwrap()
        .pragma("key", cmd.password)
        .create_if_missing(true)
        .connect()
        .await
        .unwrap();

    let _ = create_table(&mut conn).await;
    println!("Database created!")
}
