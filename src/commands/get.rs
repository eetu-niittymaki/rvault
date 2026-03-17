use sqlx::sqlite::{SqliteRow};
use sqlx::{Row, SqliteConnection, query, SqlitePool};

use crate::cli::GetCommand;

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
            println!("Password: {}", password);
        }
        Ok(None) => {
            println!("No password found for service '{}'", cmd.name);
        }
        Err(e) => {
            println!("Error getting password: {}", e);
        }
    }
}
