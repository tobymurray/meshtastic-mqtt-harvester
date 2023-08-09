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

static INSERT_LOCATION_STATEMENT: Lazy<String> = Lazy::new(|| {
	let table: String = std::env::var("POSTGRES_POSITION_TABLE").unwrap();

	format!(
		"
		INSERT INTO {table} (user_id, location, timestamp)
		VALUES ($1, ST_SetSRID(ST_MakePoint($2, $3), 4326), $4)
		ON CONFLICT ON CONSTRAINT unique_user_timestamp_positions
		DO UPDATE SET
				location = ST_SetSRID(ST_MakePoint($2, $3), 4326),
				created_at = EXCLUDED.created_at;
		"
	)
});

pub async fn insert_location(
	user_id: &str,
	latitude: f64,
	longitude: f64,
	timestamp: DateTime<Utc>,
) -> Result<u64, sqlx::Error> {
	Ok(sqlx::query(&INSERT_LOCATION_STATEMENT)
		.bind(user_id)
		.bind(longitude)
		.bind(latitude)
		.bind(timestamp)
		.execute(&*DB_POOL)
		.await?
		.rows_affected())
}

static INSERT_TELEMETRY_STATEMENT: Lazy<String> = Lazy::new(|| {
	let table: String = std::env::var("POSTGRES_TELEMETRY_TABLE").unwrap();

	format!(
		"
		INSERT INTO {table} (user_id, battery_level, voltage, timestamp)
		VALUES ($1, $2, $3, $4)
		ON CONFLICT ON CONSTRAINT unique_user_timestamp_telemetry
		DO UPDATE SET
				battery_level = EXCLUDED.battery_level,
				voltage = EXCLUDED.voltage,
				created_at = EXCLUDED.created_at;
		"
	)
});

pub async fn insert_telemetry(
	user_id: &str,
	battery_level: u32,
	voltage: f32,
	timestamp: DateTime<Utc>,
) -> Result<u64, sqlx::Error> {
	Ok(sqlx::query(&INSERT_TELEMETRY_STATEMENT)
		.bind(user_id)
		.bind(battery_level as i32)
		.bind(voltage)
		.bind(timestamp)
		.execute(&*DB_POOL)
		.await?
		.rows_affected())
}
