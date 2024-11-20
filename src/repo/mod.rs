use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Pool, Sqlite, SqlitePool};

pub mod model;
pub mod query;

fn get_db_url() -> String {
    format!("{}/.local/share/wd/stardict.db", env!("HOME"))
}

pub async fn get_db_connection() -> color_eyre::Result<Pool<Sqlite>> {
    let opts = SqliteConnectOptions::new()
        .filename(get_db_url());
    let pool = SqlitePool::connect_with(opts).await?;
    Ok(pool)
}


#[cfg(test)]
mod tests {
    use crate::repo::get_db_connection;

    #[tokio::test]
    async fn test_db_connection() {
        let _ = get_db_connection().await.unwrap();
    }
}