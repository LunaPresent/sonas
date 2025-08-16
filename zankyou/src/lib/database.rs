use sqlx::{
	Pool, Sqlite,
	sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

#[derive(Debug, Clone)]
pub struct Database(Pool<Sqlite>);

impl Database {
	pub async fn connect(path: &str) -> sqlx::Result<Self> {
		let connection_options = SqliteConnectOptions::new()
			.filename(path)
			.create_if_missing(true);

		let pool = SqlitePoolOptions::new()
			.connect_with(connection_options)
			.await;

		Ok(Self(pool))
	}
}
