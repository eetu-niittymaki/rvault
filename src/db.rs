use rpassword::read_password;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::io::{self, Write};
use std::str::FromStr;

pub async fn connect(db_path: &str) -> Result<SqlitePool, sqlx::Error> {
    let password = std::env::var("DB_PASSWORD").unwrap_or_else(|_| {
        print!("Enter database password: ");
        io::stdout().flush().unwrap();
        read_password().unwrap()
    });

    let options = SqliteConnectOptions::from_str(&format!("sqlite://{}", db_path))?
        .pragma("key", password.clone());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    sqlx::query("SELECT count(*) FROM sqlite_master;")
        .fetch_one(&pool)
        .await?;

    Ok(pool)
}