use crate::mqtt::handle_incoming;
use dotenvy::dotenv;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;

pub mod mqtt;
pub mod postgres;
pub mod utils;

pub mod meshtastic {
	include!(concat!(env!("OUT_DIR"), "/meshtastic.rs"));
}

#[tokio::main]
async fn main() {
	dotenv().ok();
	let mqtt_host = std::env::var("MQTT_HOST").unwrap();
	let mqtt_port = std::env::var("MQTT_PORT").unwrap().parse::<u16>().unwrap();

	let mut mqttoptions = MqttOptions::new("rumqtt-async", mqtt_host, mqtt_port);
	mqttoptions.set_keep_alive(Duration::from_secs(5));

	let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
	client.subscribe("msh/2/c/LongFast/#", QoS::AtMostOnce).await.unwrap();

	loop {
		let notification = match eventloop.poll().await {
			Ok(e) => e,
			Err(e) => {
				eprintln!("{:#?}", e);
				continue;
			}
		};

		match notification {
			rumqttc::Event::Incoming(i) => handle_incoming(i).await,
			rumqttc::Event::Outgoing(o) => {
				match o {
					rumqttc::Outgoing::PingReq => {} // Don't do anything, they're so loud
					_ => {
						println!("Received = {:?}", o)
					}
				}
			}
		}
	}
}
