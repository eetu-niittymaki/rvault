use sqlx::sqlite::{SqliteConnectOptions, SqliteQueryResult};
use sqlx::{ConnectOptions, Connection, SqliteConnection, query};
use std::str::FromStr;

use crate::cli::AddCommand;
use crate::config::DB_PATH;

async fn add_password(
    conn: &mut SqliteConnection,
    name: String,
    pass: String,
) -> anyhow::Result<SqliteQueryResult> {
    conn.transaction(|tx| {
        Box::pin(async move {
            query(
                r#"
                    INSERT INTO Passwords (name, password)
                    VALUES (?, ?)
                    "#,
            )
            .bind(name)
            .bind(pass)
            .execute(&mut **tx)
            .await
        })
    })
    .await
    .map_err(|e| e.into())
}

pub async fn add(cmd: AddCommand) {
    let mut conn = SqliteConnectOptions::from_str(&DB_PATH)
        .unwrap()
        .pragma("key", cmd.password)
        .connect()
        .await
        .unwrap();

    let add = add_password(&mut conn, cmd.name.clone(), cmd.word).await;

    match add {
        Ok(_) => {
            println!("{} added successfully!", cmd.name.clone())
        },
        Err(e) => {
            println!("Error in adding password: {}", e)
        }
    }
}
