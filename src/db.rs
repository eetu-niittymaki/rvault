use std::path::Path;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{SqlitePool, Error};
use std::str::FromStr;
use std::io::{self, Write};
use rpassword::read_password;

pub async fn connect(db_path: &Path) -> Result<SqlitePool, Error> {
    let password = std::env::var("DB_PASSWORD").unwrap_or_else(|_| {
        print!("Enter database password: ");
        io::stdout().flush().unwrap();
        read_password().unwrap()
    });

    let options = SqliteConnectOptions::from_str(db_path.to_str().unwrap())?
        .pragma("key", password.clone());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    match sqlx::query("SELECT count(*) FROM sqlite_master;")
        .fetch_one(&pool)
        .await
    {
        Ok(_) => Ok(pool),
        Err(e) => {
            if let Error::Database(db_err) = &e {
                if db_err.message().contains("file is not a database") {
                    println!("Incorrect password");
                    std::process::exit(0);
                }
            }
            Err(e)
        }
    }
}