use anyhow::Result;
use log::info;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, migrate};

/// Database connection manager for PostgreSQL
#[derive(Clone)]
pub struct Postgres {
  pool: PgPool,
}

impl Postgres {
  /// Creates a new PostgreSQL connection pool and runs migrations
  pub async fn try_new(database_url: &str) -> Result<Self> {
    let pool = PgPoolOptions::new().connect(database_url).await?;
    info!("Database connection established successfully");

    migrate!().run(&pool).await?;
    info!("Database migrated successfully");

    Ok(Self { pool })
  }

  /// Returns a reference to the connection pool
  pub fn pool(&self) -> &PgPool {
    &self.pool
  }
}
