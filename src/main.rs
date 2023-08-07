use crate::mqtt::handle_incoming;
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;

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

	let mqtt_host = std::env::var("MQTT_TOPIC").unwrap();

	let (client, mut eventloop) = AsyncClient::new(MQTT_CONFIG.clone(), 10);
	client.subscribe(mqtt_host, QoS::AtMostOnce).await.unwrap();

	loop {
		let notification = match eventloop.poll().await {
			Ok(e) => e,
			Err(e) => {
				eprintln!("{:#?}", e);
				continue;
			}
		};

		match notification {
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
