use color_eyre::eyre::Result;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{env, time::Duration};

pub async fn db_connect() -> Result<DatabaseConnection> {
    let db_url = format!(
        "sqlite://{}/.config/wd/stardict.db",
        env::var("HOME").expect("HOME env not set")
    );

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    let db = Database::connect(opt).await?;
    Ok(db)
}

#[tokio::test]
async fn test_db_connect() {
    let db = db_connect().await.unwrap();
    assert!(db.ping().await.is_ok());
}
