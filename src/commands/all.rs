use sqlx::sqlite::{SqliteConnectOptions, SqliteRow};
use sqlx::{Connection, Row, SqliteConnection};
use std::str::FromStr;

use crate::cli::AllCommand;
use crate::config::DB_PATH;

async fn get_all(conn: &mut SqliteConnection) -> anyhow::Result<Vec<SqliteRow>> {
    let results = conn
        .transaction(|tx| {
            Box::pin(async move {
                // Use fetch_all to actually get data
                sqlx::query("SELECT * FROM Passwords;")
                    .fetch_all(&mut **tx)
                    .await
            })
        })
        .await?;

    Ok(results)
}


pub async fn all (cmd: AllCommand) {
    let opts = SqliteConnectOptions::from_str(&DB_PATH)
        .unwrap()
        .pragma("key", cmd.password)
        .to_owned();

    let mut conn = SqliteConnection::connect_with(&opts).await.unwrap();
    
    match get_all(&mut conn).await {
        Ok(rows) => {
            println!("Name: Password");
            println!("--------------");
            for row in rows {
                let name: String = row.try_get("name").unwrap_or_else(|_| "Unknown".to_string());
                let pass: String = row.try_get("password").unwrap_or_default();

                println!("{}: {}", name, pass);
            }
        }
        Err(e) => eprintln!("Error fetching data: {}", e),
    }
}