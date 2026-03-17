use sqlx::sqlite::{SqliteConnectOptions, SqliteQueryResult};
use sqlx::{ConnectOptions, Connection, SqliteConnection, query};
use std::str::FromStr;

use crate::cli::UpdateCommand;
use crate::config::DB_PATH;

async fn update_password(
    conn: &mut SqliteConnection,
    old_name: String,
    new_name: String
) -> anyhow::Result<SqliteQueryResult> {
    conn.transaction(|tx| {
        Box::pin(async move {
            query(
                r#"
                    UPDATE Passwords 
                    SET name = ?
                    WHERE name = ?;
                    "#,
            )
            .bind(new_name)
            .bind(old_name)
            .execute(&mut **tx)
            .await
        })
    })
    .await
    .map_err(|e| e.into())
}


pub async fn update (cmd: UpdateCommand) {
    let mut conn = SqliteConnectOptions::from_str(&DB_PATH)
        .unwrap()
        .pragma("key", cmd.password)
        .connect()
        .await
        .unwrap();

    let update = update_password(&mut conn, cmd.old_name.clone(), cmd.new_name.clone()).await;

    match update {
        Ok(_) => {
            println!("{} updated to: {} successfully!", cmd.old_name.clone(), cmd.new_name.clone())
        },
        Err(e) => {
            println!("Error in updated password: {}", e)
        }
    }

}