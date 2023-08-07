use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use sqlx::{
	postgres::{PgConnectOptions, PgPoolOptions},
	Pool, Postgres,
};

static DB_POOL: Lazy<Pool<Postgres>> = Lazy::new(|| {
	let postgres_database = std::env::var("POSTGRES_DATABASE").unwrap();
	let postgres_host = std::env::var("POSTGRES_HOST").unwrap();
	let postgres_password = std::env::var("POSTGRES_PASSWORD").unwrap();
	let postgres_port = std::env::var("POSTGRES_PORT").unwrap();
	let postgres_user = std::env::var("POSTGRES_USER").unwrap();

	let options = PgConnectOptions::new()
		.username(&postgres_user)
		.password(&postgres_password)
		.host(&postgres_host)
		.port(postgres_port.parse::<u16>().unwrap())
		.database(&postgres_database);

	PgPoolOptions::new().max_connections(5).connect_lazy_with(options)
});

static INSERT_STATEMENT: Lazy<String> = Lazy::new(|| {
	let table: String = std::env::var("POSTGRES_TABLE").unwrap();

	format!(
		"INSERT INTO {table} (user_id, location, timestamp) VALUES ($1, ST_SetSRID(ST_MakePoint($2, $3), 4326), $4)"
	)
});

pub async fn insert_location(user_id: &str, latitude: f64, longitude: f64, timestamp: DateTime<Utc>) -> u64 {
	let result = sqlx::query(&INSERT_STATEMENT)
		.bind(user_id)
		.bind(latitude)
		.bind(longitude)
		.bind(timestamp)
		.execute(&*DB_POOL)
		.await
		.unwrap();

	result.rows_affected()
}
