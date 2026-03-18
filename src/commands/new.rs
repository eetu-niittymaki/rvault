use sqlx::sqlite::SqliteQueryResult;
use sqlx::{SqlitePool, query};
use std::env;

use crate::cli::NewCommand;
use crate::utils::password_gen::generate_password;
use crate::utils::copy_to_clipboard::copy_to_clipboard;
use crate::crypto::secret_crypto::encrypt;

include!(concat!(env!("OUT_DIR"), "/built_env.rs"));

async fn new_password(pool: &SqlitePool, name: String, password: String) -> anyhow::Result<SqliteQueryResult> {
    let master_pass = MASTER_PASSWORD;

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
    let password = generate_password();
    match new_password(pool, cmd.name.clone(), password.clone()).await {
        Ok(_) => {
            println!("{} added successfully", cmd.name);
            if cmd.copy { // Ih optional copy flag given
                let copy_to_clipboard = copy_to_clipboard(password);
                if copy_to_clipboard {
                    println!("{} copied to clipboard", cmd.name.clone())
                } else {
                    println!("Error in copying to clipboard!")
                }
            }
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
