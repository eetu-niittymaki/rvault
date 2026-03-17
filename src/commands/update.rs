use sqlx::sqlite::SqliteQueryResult;
use sqlx::{SqliteConnection, query, SqlitePool};

use crate::cli::UpdateCommand;

async fn update_password(
    conn: &mut SqliteConnection,
    old_name: String,
    new_name: String,
) -> anyhow::Result<SqliteQueryResult> {
    let result = query(
        r#"
        UPDATE Passwords
        SET name = ?
        WHERE name = ?;
        "#,
    )
    .bind(new_name)
    .bind(old_name)
    .execute(conn)  
    .await?;

    Ok(result)
}

pub async fn update(cmd: UpdateCommand, pool: &SqlitePool) {
    let mut conn = match pool.acquire().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to acquire DB connection: {}", e);
            return;
        }
    };

    let result = update_password(
        &mut conn, 
        cmd.old_name.clone(), 
        cmd.new_name.clone(),
    ).await;

    match result {
        Ok(_) => {
            println!(
                "{} updated to: {} successfully!",
                cmd.old_name, cmd.new_name
            );
        }
        Err(e) => {
            println!("Error updating password: {}", e);
        }
    }
}
