use rpassword::read_password;
use sqlx::Error;
use sqlx::sqlite::SqlitePool;
use std::io::{self, Write};
use std::str::FromStr;

pub async fn connect(db_path: &str) -> Result<SqlitePool, sqlx::Error> {
    let password = std::env::var("DB_PASSWORD").unwrap_or_else(|_| {
        print!("Enter database password: ");
        io::stdout().flush().unwrap();
        read_password().unwrap()
    });

    let options = sqlx::sqlite::SqliteConnectOptions::from_str(&format!("sqlite://{}", db_path))
        .unwrap()
        .pragma("key", password.clone());

    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options.clone())
        .await?;

    match sqlx::query("SELECT count(*) FROM sqlite_master;")
        .fetch_one(&pool)
        .await
    {
        Ok(_) => Ok(pool),
        Err(e) => {
            if let Error::Database(db_err) = &e {
                if db_err.message().contains("file is not a database") {
                    println!("Incorrect password for database"); 
                }
            }
            Err(e) 
        }
    }
}
