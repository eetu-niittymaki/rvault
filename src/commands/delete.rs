use sqlx::sqlite::{SqliteQueryResult};
use sqlx::{SqliteConnection, query, SqlitePool};

use crate::cli::DeleteCommand;

async fn delete_password(
    conn: &mut SqliteConnection,
    name: String,
) -> anyhow::Result<SqliteQueryResult> {
    let result = query(
        r#"
        DELETE FROM  Passwords
        WHERE name = ?
        "#,
    )
    .bind(name)
    .execute(conn)  
    .await?;

    Ok(result)
}

pub async fn delete(cmd: DeleteCommand, pool: &SqlitePool) {
    let mut conn = match pool.acquire().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to acquire DB connection: {}", e);
            return;
        }
    };

    let delete = delete_password(&mut conn, cmd.name.clone()).await;

    match delete {
        Ok(_) => {
            println!("{} deleted successfully!", cmd.name.clone())
        }
        Err(e) => {
            println!("Error in deleting password: {}", e)
        }
    }
}
