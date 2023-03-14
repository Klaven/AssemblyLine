use std::{path::Path, env};

use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};

pub struct ConnectionManager {
    pub pool: SqlitePool,
}

impl ConnectionManager {
    pub async fn new(url: String) -> anyhow::Result<Self> {
        // will create the db if needed
        let url = SqliteConnectOptions::new()
            .filename(url)
            .create_if_missing(true);
        let pool = SqlitePool::connect_with(url).await?;

        // Run migrations
        let migrations = if env::var("RUST_ENV") == Ok("production".to_string()) {
            // Productions migrations dir
            std::env::current_exe()?.join("./migrations")
        } else {
            // Development migrations dir
            let crate_dir = std::env::var("CARGO_MANIFEST_DIR")?;
            Path::new(&crate_dir)
                .join("./migrations")
        };
    
        sqlx::migrate::Migrator::new(migrations)
            .await?
            .run(&pool)
            .await?;

        // Return the connection manager
        Ok(Self { pool })
    }
}
