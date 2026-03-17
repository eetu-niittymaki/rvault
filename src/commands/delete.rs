use sqlx::sqlite::{SqliteConnectOptions, SqliteQueryResult};
use sqlx::{ConnectOptions, Connection, SqliteConnection, query};
use std::str::FromStr;

use crate::cli::DeleteCommand;
use crate::config::DB_PATH;

async fn delete_password(
    conn: &mut SqliteConnection,
    name: String,
) -> anyhow::Result<SqliteQueryResult> {
    conn.transaction(|tx| {
        Box::pin(async move {
            query(
                r#"
                    DELETE FROM Passwords 
                    WHERE name = ?;
                    "#,
            )
            .bind(name)
            .execute(&mut **tx)
            .await
        })
    })
    .await
    .map_err(|e| e.into())
}

pub async fn delete (cmd: DeleteCommand) {
    let mut conn = SqliteConnectOptions::from_str(&DB_PATH)
        .unwrap()
        .pragma("key", cmd.password)
        .connect()
        .await
        .unwrap();

    let delete = delete_password(&mut conn, cmd.name.clone()).await;

    match delete {
        Ok(_) => {
            println!("{} deleted successfully!", cmd.name.clone())
        },
        Err(e) => {
            println!("Error in deleting password: {}", e)
        }
    }
}