use chrono::{DateTime, Utc};
use tokio_postgres::NoTls;

pub async fn setup() -> tokio_postgres::Client {
	let postgres_database = std::env::var("POSTGRES_DATABASE").unwrap();
	let postgres_host = std::env::var("POSTGRES_HOST").unwrap();
	let postgres_password = std::env::var("POSTGRES_PASSWORD").unwrap();
	let postgres_port = std::env::var("POSTGRES_PORT").unwrap();
	let postgres_user = std::env::var("POSTGRES_USER").unwrap();

	let config = format!("host={postgres_host} user={postgres_user} port={postgres_port} dbname={postgres_database} password={postgres_password}");

	let (client, connection) = tokio_postgres::connect(&config, NoTls).await.unwrap();

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
	let table = std::env::var("POSTGRES_TABLE").unwrap();

	let stmt = format!(
		"INSERT INTO {table} (user_id, location, timestamp) VALUES ($1, ST_SetSRID(ST_MakePoint($2, $3), 4326), $4)"
	);

	// Define the SQL statement with placeholders for the parameters
	let stmt = client.prepare(&stmt).await.unwrap();

	// Execute the prepared statement with the provided arguments
	client
		.execute(&stmt, &[&user_id, &longitude, &latitude, &timestamp])
		.await
		.unwrap()
}
