use crate::mqtt::handle_incoming;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;

pub mod mqtt;
pub mod utils;

pub mod meshtastic {
	include!(concat!(env!("OUT_DIR"), "/meshtastic.rs"));
}

#[tokio::main]
async fn main() {
	println!("Hello, world!");

	let mut mqttoptions = MqttOptions::new("rumqtt-async", "192.46.222.65", 1883);
	mqttoptions.set_keep_alive(Duration::from_secs(5));

	let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
	client.subscribe("#", QoS::AtMostOnce).await.unwrap();

	loop {
		let notification = eventloop.poll().await.unwrap();

		match notification {
			rumqttc::Event::Incoming(i) => handle_incoming(i),
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
