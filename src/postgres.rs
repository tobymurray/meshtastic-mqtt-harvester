use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use tokio_postgres::NoTls;

static CONFIG: Lazy<String> = Lazy::new(|| {
	let postgres_database = std::env::var("POSTGRES_DATABASE").unwrap();
	let postgres_host = std::env::var("POSTGRES_HOST").unwrap();
	let postgres_password = std::env::var("POSTGRES_PASSWORD").unwrap();
	let postgres_port = std::env::var("POSTGRES_PORT").unwrap();
	let postgres_user = std::env::var("POSTGRES_USER").unwrap();

	format!("host={postgres_host} user={postgres_user} port={postgres_port} dbname={postgres_database} password={postgres_password}")
});

static INSERT_STATEMENT: Lazy<String> = Lazy::new(|| {
	let table = std::env::var("POSTGRES_TABLE").unwrap();

	format!(
		"INSERT INTO {table} (user_id, location, timestamp) VALUES ($1, ST_SetSRID(ST_MakePoint($2, $3), 4326), $4)"
	)
});

pub async fn setup() -> tokio_postgres::Client {
	let (client, connection) = tokio_postgres::connect(&CONFIG, NoTls).await.unwrap();

	tokio::spawn(async move {
		if let Err(e) = connection.await {
			eprintln!("connection error: {}", e);
		}
	});

	client
}

pub async fn insert_location(
	client: tokio_postgres::Client,
	user_id: &str,
	latitude: f64,
	longitude: f64,
	timestamp: DateTime<Utc>,
) -> u64 {
	let stmt = client.prepare(&INSERT_STATEMENT).await.unwrap();

	client
		.execute(&stmt, &[&user_id, &longitude, &latitude, &timestamp])
		.await
		.unwrap()
}
