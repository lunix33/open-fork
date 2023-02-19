use diesel::{
    r2d2::{ConnectionManager, Pool},
    sqlite::Sqlite,
    SqliteConnection,
};
use diesel_migrations::{
    embed_migrations, EmbeddedMigrations, HarnessWithOutput, MigrationHarness,
};

const MIGRATION: EmbeddedMigrations = embed_migrations!("./migrations/db");

pub type DbConnection = SqliteConnection;
pub type DbConnectionManager = ConnectionManager<DbConnection>;
pub type DbPool = Pool<DbConnectionManager>;
pub type DbBackend = Sqlite;

pub fn connect_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<DbConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .unwrap_or_else(|e| panic!("Unable to open database '{database_url}': {e}"));

    let mut connection = pool.get().unwrap();
    run_migrations(&mut connection).expect("Migrations failed.");

    pool
}

pub fn env_url() -> String {
    std::env::var("OF_DATABASE_URL").unwrap_or(String::from("./db.sqlite3"))
}

fn run_migrations(
    connection: &mut impl MigrationHarness<DbBackend>,
) -> diesel::migration::Result<()> {
    HarnessWithOutput::write_to_stdout(connection).run_pending_migrations(MIGRATION)?;

    Ok(())
}
