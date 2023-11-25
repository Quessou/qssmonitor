use sqlx::migrate::MigrateError;
use sqlx::{migrate, Pool};
use sqlx::{sqlite, Sqlite, SqlitePool};

use crate::filesystem::paths;

pub async fn apply_migrations(connection: &Pool<Sqlite>) -> Result<(), MigrateError> {
    migrate!("db/migrations").run(connection).await
}

pub async fn connect_to_database() -> SqlitePool {
    let connect_options = sqlite::SqliteConnectOptions::new()
        .filename(paths::get_db_file_path())
        .create_if_missing(true);
    SqlitePool::connect_with(connect_options).await.unwrap()
}
