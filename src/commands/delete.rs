use sqlx::{SqliteConnection, query, SqlitePool};

use crate::cli::DeleteCommand;

async fn delete_password(
    conn: &mut SqliteConnection,
    name: String,
) -> anyhow::Result<()> {
    let result = query(
        r#"
        DELETE FROM Passwords
        WHERE name = ?
        "#
    )
    .bind(&name)
    .execute(conn)
    .await?;

    if result.rows_affected() > 0 {
        println!("{} deleted successfully!", name);
    } else {
        println!("Password doesn't exist");
    }

    Ok(()) 
}

pub async fn delete(cmd: DeleteCommand, pool: &SqlitePool) {
    let mut conn = match pool.acquire().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to acquire DB connection: {}", e);
            return;
        }
    };

    let _ = delete_password(&mut conn, cmd.name.clone()).await;
}
