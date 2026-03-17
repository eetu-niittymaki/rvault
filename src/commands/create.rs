use sqlx::{query, SqliteConnection, SqlitePool};
use sqlx::sqlite::{SqliteConnectOptions, SqliteQueryResult,};
use std::path::Path;
use std::str::FromStr;
use std::{fs, io, io::Write};

use rpassword::read_password; 

use crate::config::DB_PATH;

pub async fn create() -> anyhow::Result<SqlitePool> {
    // Check if DB exists
    if Path::new(DB_PATH).exists() {
        println!("Database already exists. Delete and create new? [y/n]");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
            println!("Database creation cancelled.");
            std::process::exit(0);
        }

        fs::remove_file(DB_PATH)?;
    }

    let password = prompt_password();

    let opts = SqliteConnectOptions::from_str(DB_PATH)?
        .pragma("key", password)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(opts).await?;

    let mut conn = pool.acquire().await?;
    create_table(&mut conn).await?;

    println!("Database created successfully!");
    Ok(pool)
}

fn prompt_password() -> String {
    print!("Enter database password: ");
    io::stdout().flush().unwrap();
    let password = read_password().unwrap();

    print!("Confirm password: ");
    io::stdout().flush().unwrap();
    let password_confirm = read_password().unwrap();

    if password != password_confirm {
        eprintln!("Passwords do not match!");
        std::process::exit(1);
    }

    password
}

async fn create_table(conn: &mut SqliteConnection) -> anyhow::Result<SqliteQueryResult> {
    query(
        r#"
        CREATE TABLE IF NOT EXISTS Passwords(
            id       INTEGER PRIMARY KEY NOT NULL,
            name     TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL UNIQUE
        );
        "#
    )
    .execute(conn)
    .await
    .map_err(Into::into)
}