use sqlx::sqlite::{Sqlite, SqlitePool};

use crate::ApplicationResult;

// static MIGRATOR: Migrator = sqlx::migrate!("migrations/db");

pub type DbPool = SqlitePool;
pub type DbType = Sqlite;

pub async fn connect_pool(database_url: &str) -> ApplicationResult<DbPool> {
    let pool = DbPool::connect_lazy(database_url.into())?;

    let mut migrator = sqlx::migrate!("migrations/db");
    migrator.set_ignore_missing(true);
    migrator.run(&pool).await?;

    Ok(pool)
}

pub fn env_url() -> String {
    std::env::var("OF_DATABASE_URL").unwrap_or(String::from("sqlite:db.sqlite3"))
}
