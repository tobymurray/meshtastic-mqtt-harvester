use crate::mqtt::handle_incoming;
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;
use thiserror::Error;

mod protobufs {
	pub mod meshtastic;
}

mod packet_handlers {
	pub mod publish;
}

pub mod errors;
pub mod mqtt;
pub mod postgres;
pub mod utils;

#[derive(Error, Debug)]
enum HarvesterError {
	#[error("Client error {0}")]
	ClientEror(#[from] rumqttc::ClientError),
	#[error("Connection error {0}")]
	ConnectionError(#[from] rumqttc::ConnectionError),
}

static MQTT_CONFIG: Lazy<MqttOptions> = Lazy::new(|| {
	let mqtt_host = std::env::var("MQTT_HOST").unwrap();
	let mqtt_port = std::env::var("MQTT_PORT").unwrap().parse::<u16>().unwrap();
	let mqtt_user = std::env::var("MQTT_USER").unwrap();

	let mut mqttoptions = MqttOptions::new(mqtt_user, mqtt_host, mqtt_port);
	mqttoptions.set_keep_alive(Duration::from_secs(5));
	mqttoptions
});

#[tokio::main]
async fn main() {
	dotenv().ok();

	let mqtt_topic = std::env::var("MQTT_TOPIC").unwrap();
	let mut reconnect_delay = Duration::from_secs(5);

	loop {
		match run_mqtt_client(mqtt_topic.clone()).await {
			Ok(_) => {
				eprintln!("MQTT client disconnected. Reconnecting...");
				// Reset the delay to the initial value on successful connection
				reconnect_delay = Duration::from_secs(5);
			}
			Err(e) => {
				eprintln!("Error during MQTT client execution: {:#?}", e);
				// Incrementally increase the delay before attempting to reconnect
				println!("Sleeping #{reconnect_delay:?} before reconnecting");
				tokio::time::sleep(reconnect_delay).await;
				reconnect_delay = reconnect_delay.mul_f32(1.5).min(Duration::from_secs(300));
			}
		}
	}
}

async fn run_mqtt_client(mqtt_topic: String) -> Result<(), HarvesterError> {
	let (client, mut eventloop) = AsyncClient::new(MQTT_CONFIG.clone(), 10);
	client.subscribe(mqtt_topic.clone(), QoS::AtMostOnce).await?;

	loop {
		match eventloop.poll().await? {
			rumqttc::Event::Incoming(i) => {
				if let Err(e) = handle_incoming(i).await {
					eprintln!("Error = {:?}", e);
				}
			}
			rumqttc::Event::Outgoing(o) => {
				match o {
					rumqttc::Outgoing::PingReq => {} // Don't do anything, they're so loud
					_ => {
						println!("Outgoing = {:?}", o)
					}
				}
			}
		}
	}
}
