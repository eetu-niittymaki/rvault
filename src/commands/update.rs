use sqlx::{SqliteConnection, SqlitePool, query};

use crate::cli::UpdateCommand;

async fn update_password(
    conn: &mut SqliteConnection,
    old_name: String,
    new_name: String,
) -> anyhow::Result<()> {
    let result = query(
        r#"
        UPDATE Passwords
        SET name = ?
        WHERE name = ?;
        "#,
    )
    .bind(&new_name)
    .bind(&old_name)
    .execute(conn)
    .await?;

    if result.rows_affected() > 0 {
        println!("{} updated to: {}", old_name, new_name);
    } else {
        println!("Password doesn't exist");
    }

    Ok(())
}

pub async fn update(cmd: UpdateCommand, pool: &SqlitePool) {
    let mut conn = match pool.acquire().await {
        Ok(conn) => conn,
        Err(e) => {
            println!("Failed to acquire DB connection: {}", e);
            return;
        }
    };

    let _ = update_password(&mut conn, cmd.old_name.clone(), cmd.new_name.clone()).await;
}
