use sqlx::{query, SqlitePool};
use sqlx::sqlite::SqliteQueryResult;

use crate::cli::NewCommand;

async fn new_password(
    pool: &SqlitePool,
    name: String,
    pass: String,
) -> anyhow::Result<SqliteQueryResult> {
    let result = query(
        r#"
        INSERT INTO Passwords (name, password)
        VALUES (?, ?)
        "#,
    )
    .bind(name)
    .bind(pass)
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