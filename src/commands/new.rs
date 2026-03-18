use sqlx::sqlite::SqliteQueryResult;
use sqlx::{SqlitePool, query};
use std::env;

use crate::cli::NewCommand;
use crate::utils::password_gen::generate_password;
use crate::crypto::secret_crypto::encrypt;

include!(concat!(env!("OUT_DIR"), "/built_env.rs"));

async fn new_password(pool: &SqlitePool, name: String) -> anyhow::Result<SqliteQueryResult> {
    let master_pass = MASTER_PASSWORD;
    let password = generate_password();

    let result = query(
        r#"
        INSERT INTO Passwords (name, password)
        VALUES (?, ?)
        "#,
    )
    .bind(name)
    .bind(encrypt(&password, master_pass.to_string()))
    .execute(pool)
    .await?;

    Ok(result)
}

pub async fn new(cmd: NewCommand, pool: &SqlitePool) {
    match new_password(pool, cmd.name.clone()).await {
        Ok(_) => {
            println!("{} added successfully!", cmd.name);
        }
        Err(e) => {
            if let Some(sqlx::Error::Database(db_err)) = e.downcast_ref::<sqlx::Error>() {
                if db_err.code().as_deref() == Some("2067") {
                    println!("A password with that name already exists.");
                    return;
                }
            }

            println!("Error adding password: {}", e);
        }
    }
}
