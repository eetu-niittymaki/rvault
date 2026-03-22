use sqlx::{query, SqliteConnection, SqlitePool};
use sqlx::sqlite::{SqliteConnectOptions, SqliteQueryResult,};
use std::str::FromStr;
use std::{fs, io, io::Write};
use rpassword::read_password; 

use crate::db::connect; 
use crate::config::get_db_path;

pub async fn create() -> anyhow::Result<SqlitePool> {
    let db_path = get_db_path();
    
    // Check if DB exists, ask to delete if does
    if db_path.exists() {
        // Try to connect to existing database
        let _ = connect(&db_path).await;

        let mut delete = String::new();
        let mut confirm_delete = String::new();

        println!("Database already exists. Delete and create new database? [y/n]");
        io::stdin().read_line(&mut delete)?;
        if !matches!(delete.trim().to_lowercase().as_str(), "y" | "yes") {
            println!("Database creation cancelled");
            std::process::exit(0);
        } 

        println!("Confirm database deletion: [y/n]");
        io::stdin().read_line(&mut confirm_delete)?;
        if !matches!(confirm_delete.trim().to_lowercase().as_str(), "y" | "yes") {
            println!("Database creation cancelled");
            std::process::exit(0);
        } 

        fs::remove_file(&db_path)?;
    }

    let password = prompt_password();

    let opts = SqliteConnectOptions::from_str(db_path.to_str().unwrap())?
        .pragma("key", password)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(opts).await?;

    let mut conn = pool.acquire().await?;
    create_table(&mut conn).await?;

    println!("Database created successfully!");
    Ok(pool)
}

fn prompt_password() -> String {
    print!("Enter password to use with database: ");
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
            password TEXT NOT NULL
        );
        "#
    )
    .execute(conn)
    .await
    .map_err(Into::into)
}