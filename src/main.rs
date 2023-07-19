use chrono::{DateTime, NaiveDateTime, Utc};
use meshtastic::mesh_packet::PayloadVariant;
use meshtastic::{Data, MeshPacket, PortNum, ServiceEnvelope};
use prost::Message;
use rumqttc::Packet::{
	self, ConnAck, Connect, Disconnect, PingReq, PingResp, PubAck, PubComp, PubRec, PubRel, Publish, SubAck, Subscribe,
	UnsubAck, Unsubscribe,
};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;

use crate::meshtastic::mesh_packet;

pub mod meshtastic {
	include!(concat!(env!("OUT_DIR"), "/meshtastic.rs"));
}

const COORDINATE_MULTIPLIER: f64 = 0.0000001;

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
                    rumqttc::Outgoing::PingReq => {}, // Don't do anything, they're so loud
                    _ => { println!("Received = {:?}", o)}
                    // rumqttc::Outgoing::Publish(_) => todo!(),
                    // rumqttc::Outgoing::Subscribe(_) => todo!(),
                    // rumqttc::Outgoing::Unsubscribe(_) => todo!(),
                    // rumqttc::Outgoing::PubAck(_) => todo!(),
                    // rumqttc::Outgoing::PubRec(_) => todo!(),
                    // rumqttc::Outgoing::PubRel(_) => todo!(),
                    // rumqttc::Outgoing::PubComp(_) => todo!(),
                    // rumqttc::Outgoing::PingResp => todo!(),
                    // rumqttc::Outgoing::Disconnect => todo!(),
                    // rumqttc::Outgoing::AwaitAck(_) => todo!(),
                }
			}
		}
	}
}

fn handle_incoming(i: Packet) {
	match i {
      Publish(p) => handle_incoming_publish(p),
    PingResp => {}, // Don't do anything, they're so loud
    _ => { println!("Received = {:?}", i)}
    // Connect(_) => todo!(),
    // ConnAck(_) => todo!(),
    // PubAck(_) => todo!(),
    // PubRec(_) => todo!(),
    // PubRel(_) => todo!(),
    // PubComp(_) => todo!(),
    // Subscribe(_) => todo!(),
    // SubAck(_) => todo!(),
    // Unsubscribe(_) => todo!(),
    // UnsubAck(_) => todo!(),
    // PingReq => todo!(),
    // PingResp => todo!(),
    // Disconnect => todo!(),
  }
}

fn handle_portnum(p: PortNum, d: Data) {
	match p {
		// PortNum::UnknownApp => todo!(),
		PortNum::TextMessageApp => {
			println!("  Decoded = {:?}", d);
			println!("    Payload = {:?}", d.payload);
			match String::from_utf8(d.payload) {
				Ok(t) => {
					println!("    TextMessage = {:?}", t)
				}
				Err(e) => println!("  Error = {:?}", e),
			}
		}
		// PortNum::RemoteHardwareApp => todo!(),
		PortNum::PositionApp => {
			println!("  Decoded = {:?}", d);
			match meshtastic::Position::decode(d.payload.as_ref()) {
				Ok(p) => {
					let timestamp = NaiveDateTime::from_timestamp_opt(p.time.into(), 0);

					println!("    Position = {:?}", p);
					println!("      Longitude = {:.5?}", p.longitude_i as f64 * COORDINATE_MULTIPLIER);
					println!("      Latitude = {:.5?}", p.latitude_i as f64 * COORDINATE_MULTIPLIER);
					match timestamp {
						Some(t) => {
							let datetime = DateTime::<Utc>::from_utc(t, Utc);
							let est_tz = chrono_tz::Tz::America__New_York;
							let est_datetime = datetime.with_timezone(&est_tz);
							println!("      Datetime = {:?}", est_datetime)
						}
						None => println!("      Can't parse timestamp {:?}", p.time),
					}
				}
				Err(e) => println!("  Error = {:?}", e),
			}
		}
		// PortNum::NodeinfoApp => todo!(),
		// PortNum::RoutingApp => todo!(),
		// PortNum::AdminApp => todo!(),
		// PortNum::TextMessageCompressedApp => todo!(),
		// PortNum::WaypointApp => todo!(),
		// PortNum::AudioApp => todo!(),
		// PortNum::MqttClientProxyApp => todo!(),
		// PortNum::ReplyApp => todo!(),
		// PortNum::IpTunnelApp => todo!(),
		// PortNum::SerialApp => todo!(),
		// PortNum::StoreForwardApp => todo!(),
		// PortNum::RangeTestApp => todo!(),
		PortNum::TelemetryApp => {
			println!("  Decoded = {:?}", d);
			println!("    Payload = {:?}", d.payload);
			match meshtastic::Telemetry::decode(d.payload.as_ref()) {
				Ok(t) => {
					println!("    Telemetry = {:?}", t)
				}
				Err(e) => println!("  Error = {:?}", e),
			}
		}
		// PortNum::ZpsApp => todo!(),
		// PortNum::SimulatorApp => todo!(),
		// PortNum::TracerouteApp => todo!(),
		// PortNum::NeighborinfoApp => todo!(),
		// PortNum::PrivateApp => todo!(),
		// PortNum::AtakForwarder => todo!(),
		// PortNum::Max => todo!(),
		_ => {
			println!("  Decoded = {:?}", d);
			println!("    Payload = {:?}", d.payload);
		}
	}
}

fn handle_decoded(d: Data) {
	let portnum = int_to_portnum(d.portnum);

	match portnum {
		Some(p) => handle_portnum(p, d),
		None => todo!(),
	}
}

fn handle_payload_variant(v: PayloadVariant) {
	match v {
		mesh_packet::PayloadVariant::Decoded(d) => handle_decoded(d),
		mesh_packet::PayloadVariant::Encrypted(e) => {
			println!("  Encrypted = {:?}", e)
		}
	}
}

fn handle_mesh_packet(p: MeshPacket) {
	match p.payload_variant {
		Some(v) => handle_payload_variant(v),
		None => println!("  Decoded = {:?}", p),
	}
}

fn handle_service_envelope(m: ServiceEnvelope) {
	match m.packet {
		Some(p) => handle_mesh_packet(p),
		None => println!("  Decoded = {:?}", m),
	}
}

fn handle_mesh_2_c(p: rumqttc::Publish) {
	println!("Received = {:?}, {:?}", p, p.payload);

	let message: Result<crate::meshtastic::ServiceEnvelope, prost::DecodeError> =
		crate::meshtastic::ServiceEnvelope::decode(p.payload.clone());
	match message {
		Ok(m) => handle_service_envelope(m),
		Err(e) => println!("  Error = {:?}", e),
	}
}

fn handle_incoming_publish(p: rumqttc::Publish) {
	if p.topic.starts_with("msh/2/c/") {
		handle_mesh_2_c(p);
	} else {
		println!("Received = {:?}, {:?}", p, p.payload);
	}
	println!("")
}

fn int_to_portnum(portnum: i32) -> Option<PortNum> {
	match portnum {
		0 => Some(PortNum::UnknownApp),
		1 => Some(PortNum::TextMessageApp),
		2 => Some(PortNum::RemoteHardwareApp),
		3 => Some(PortNum::PositionApp),
		4 => Some(PortNum::NodeinfoApp),
		5 => Some(PortNum::RoutingApp),
		6 => Some(PortNum::AdminApp),
		7 => Some(PortNum::TextMessageCompressedApp),
		8 => Some(PortNum::WaypointApp),
		9 => Some(PortNum::AudioApp),
		10 => Some(PortNum::MqttClientProxyApp),
		32 => Some(PortNum::ReplyApp),
		33 => Some(PortNum::IpTunnelApp),
		64 => Some(PortNum::SerialApp),
		65 => Some(PortNum::StoreForwardApp),
		66 => Some(PortNum::RangeTestApp),
		67 => Some(PortNum::TelemetryApp),
		68 => Some(PortNum::ZpsApp),
		69 => Some(PortNum::SimulatorApp),
		70 => Some(PortNum::TracerouteApp),
		71 => Some(PortNum::NeighborinfoApp),
		256 => Some(PortNum::PrivateApp),
		257 => Some(PortNum::AtakForwarder),
		511 => Some(PortNum::Max),
		_ => None,
	}
}
