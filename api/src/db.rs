use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, SqlitePool};
use std::env;

pub async fn connect() -> Result<SqlitePool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./sql.db".to_string());
    let options = SqliteConnectOptions::from_str(&db_url)?.create_if_missing(true);
    SqlitePool::connect_with(options).await
}

pub async fn migrate(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations").run(pool).await
}
