use sqlx::sqlite::{SqliteRow};
use sqlx::{Row, SqliteConnection, query, SqlitePool};
use std::env;

use crate::cli::GetCommand;
use crate::crypto::secret_crypto::decrypt;
use crate::utils::copy_to_clipboard::copy_to_clipboard;

include!(concat!(env!("OUT_DIR"), "/built_env.rs"));

async fn get_password(
    conn: &mut SqliteConnection,
    name: String,
) -> anyhow::Result<Option<String>> {
    let row: Option<SqliteRow> = query(
        "SELECT password FROM Passwords WHERE name = ?"
    )
    .bind(name)
    .fetch_optional(conn) 
    .await?;

    if let Some(row) = row {
        let password: String = row.try_get("password")?;
        Ok(Some(password))
    } else {
        Ok(None)
    }
}

pub async fn get(cmd: GetCommand, pool: &SqlitePool) {
    let master_pass = MASTER_PASSWORD;

    let mut conn = match pool.acquire().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to acquire DB connection: {}", e);
            return;
        }
    };

    let get = get_password(& mut conn, cmd.name.clone()).await;

    match get {
        Ok(Some(password)) => {
            let decrypt_pass = decrypt(&password, master_pass.to_string());
            let copy_to_clipboard = copy_to_clipboard(decrypt_pass.unwrap());
            if copy_to_clipboard {
                println!("{} copied to clipboard succesfully", cmd.name.clone())
            } else {
                println!("Error in copying to clipboard")
            }
        }
        Ok(None) => {
            println!("No password found for service '{}'", cmd.name);
        }
        Err(e) => {
            println!("Error getting password: {}", e);
        }
    }
}
