use crate::meshtastic::{mesh_packet::PayloadVariant, Data, MeshPacket, PortNum, Position, ServiceEnvelope, Telemetry};
use crate::utils::{get_user_id, int_to_portnum};
use chrono::{DateTime, NaiveDateTime, Utc};
use prost::Message;
use rumqttc::Packet::{self, PingResp, Publish};

use crate::meshtastic::mesh_packet;

const COORDINATE_MULTIPLIER: f64 = 0.0000001;

pub fn handle_incoming(i: Packet) {
	match i {
		Publish(p) => handle_incoming_publish(p),
		PingResp => {} // Don't do anything, they're so loud
		_ => println!("Received = {:?}", i),
	}
}

fn handle_portnum(topic: &str, p: PortNum, d: Data) {
	match p {
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
		PortNum::PositionApp => {
			println!("  Decoded = {:?}", d);
			match Position::decode(d.payload.as_ref()) {
				Ok(p) => {
					let timestamp = NaiveDateTime::from_timestamp_opt(p.time.into(), 0);

					let user_id = get_user_id(topic).unwrap_or("UNKNOWN CLIENT");

					println!("    Topic = {:?}", topic);
					println!("      Client = {:?}", user_id);
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
		PortNum::TelemetryApp => {
			println!("  Decoded = {:?}", d);
			println!("    Payload = {:?}", d.payload);
			match Telemetry::decode(d.payload.as_ref()) {
				Ok(t) => {
					println!("    Telemetry = {:?}", t)
				}
				Err(e) => println!("  Error = {:?}", e),
			}
		}
		_ => {
			println!("  Decoded = {:?}", d);
			println!("    Payload = {:?}", d.payload);
		}
	}
}

fn handle_decoded(topic: &str, d: Data) {
	let portnum = int_to_portnum(d.portnum);

	match portnum {
		Some(p) => handle_portnum(topic, p, d),
		None => todo!(),
	}
}

fn handle_payload_variant(topic: &str, v: PayloadVariant) {
	match v {
		mesh_packet::PayloadVariant::Decoded(d) => handle_decoded(topic, d),
		mesh_packet::PayloadVariant::Encrypted(e) => {
			println!("  Encrypted = {:?}", e)
		}
	}
}

fn handle_mesh_packet(topic: &str, p: MeshPacket) {
	match p.payload_variant {
		Some(v) => handle_payload_variant(topic, v),
		None => println!("  Decoded = {:?}", p),
	}
}

fn handle_service_envelope(topic: &str, m: ServiceEnvelope) {
	match m.packet {
		Some(p) => handle_mesh_packet(topic, p),
		None => println!("  Decoded = {:?}", m),
	}
}

fn handle_mesh_2_c(p: rumqttc::Publish) {
	println!("Received = {:?}, {:?}", p, p.payload);
	let topic = p.topic;
	let message: Result<ServiceEnvelope, prost::DecodeError> = ServiceEnvelope::decode(p.payload.clone());
	match message {
		Ok(m) => handle_service_envelope(&topic, m),
		Err(e) => println!("  Error = {:?}", e),
	}
}

fn handle_incoming_publish(p: rumqttc::Publish) {
	if p.topic.starts_with("msh/2/c/") {
		handle_mesh_2_c(p);
	} else {
		println!("Received = {:?}, {:?}", p, p.payload);
	}
	println!()
}
