use sqlx::{query, SqlitePool};
use sqlx::sqlite::SqliteQueryResult;
use dotenv::dotenv;
use std::env;

use crate::cli::NewCommand;
use crate::crypto::secret_crypto::encrypt;

async fn new_password(
    pool: &SqlitePool,
    name: String,
    pass: String,
) -> anyhow::Result<SqliteQueryResult> {
    dotenv().ok();
    let master_pass = env::var("MASTER_PASSWORD").expect("API_KEY must be set");

    let result = query(
        r#"
        INSERT INTO Passwords (name, password)
        VALUES (?, ?)
        "#,
    )
    .bind(name)
    .bind(encrypt(&pass, master_pass))
    .execute(pool)  // execute directly on the pool
    .await?;

    Ok(result)
}

pub async fn new(cmd: NewCommand, pool: &SqlitePool) {
    match new_password(pool, cmd.name.clone(), cmd.word.clone()).await {
        Ok(_) => {
            println!("{} added successfully!", cmd.name);
        }
        Err(e) => {
            println!("Error adding password: {}", e);
        }
    }
}