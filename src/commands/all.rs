use sqlx::{query, Row, SqlitePool};
use sqlx::sqlite::SqliteRow;

use crate::cli::AllCommand;

async fn get_all(pool: &SqlitePool) -> anyhow::Result<Vec<SqliteRow>> {
    let results = query("SELECT * FROM Passwords;")
        .fetch_all(pool)
        .await?;

    Ok(results)
}

pub async fn all(_cmd: AllCommand, pool: &SqlitePool) {
    match get_all(pool).await {
        Ok(rows) => {
            if !rows.is_empty() {
                println!("Saved Passwords");
                println!("---------------");
                for row in rows {
                    let name: String = row.try_get("name").unwrap_or_else(|_| "Unknown".to_string());

                    println!("{}", name);
                }
            } else {
                println!("No passwords saved");
            }
        }
        Err(e) => eprintln!("Error fetching data: {}", e),
    }
}