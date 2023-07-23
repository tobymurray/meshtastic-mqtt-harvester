use crate::meshtastic::{mesh_packet::PayloadVariant, Data, MeshPacket, PortNum, Position, ServiceEnvelope, Telemetry};
use crate::utils::{get_user_id, int_to_portnum};
use chrono::{DateTime, NaiveDateTime, Utc};
use prost::Message;
use rumqttc::Packet::{self, PingResp, Publish};

const COORDINATE_MULTIPLIER: f64 = 0.0000001;

pub async fn handle_incoming(i: Packet) {
	match i {
		Publish(p) => handle_incoming_publish(p).await,
		PingResp => {} // Don't do anything, they're so loud
		_ => println!("Received = {:?}", i),
	}
}

async fn handle_position(topic: &str, d: Data) {
	println!("  Decoded = {:?}", d);
	match Position::decode(d.payload.as_ref()) {
		Ok(p) => {
			let timestamp = NaiveDateTime::from_timestamp_opt(p.time.into(), 0);

			let user_id = get_user_id(topic).unwrap_or("UNKNOWN CLIENT");

			let latitude = p.latitude_i as f64 * COORDINATE_MULTIPLIER;
			let longitude = p.longitude_i as f64 * COORDINATE_MULTIPLIER;
			println!("    Topic = {:?}", topic);
			println!("      Client = {:?}", user_id);
			println!("      Longitude = {:.5?}", longitude);
			println!("      Latitude = {:.5?}", latitude);
			match timestamp {
				Some(t) => {
					let datetime = DateTime::<Utc>::from_utc(t, Utc);
					let est_tz = chrono_tz::Tz::America__New_York;
					let est_datetime = datetime.with_timezone(&est_tz);
					println!("      Datetime = {:?}", est_datetime);
					let db_client = crate::postgres::setup().await;
					crate::postgres::insert_location(db_client, user_id, latitude, longitude, datetime).await;
				}
				None => println!("      Can't parse timestamp {:?}", p.time),
			}
		}
		Err(e) => println!("  Error = {:?}", e),
	}
}

async fn handle_portnum(topic: &str, p: PortNum, d: Data) {
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
		PortNum::PositionApp => handle_position(topic, d).await,
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

async fn handle_decoded(topic: &str, d: Data) {
	let portnum = int_to_portnum(d.portnum);

	match portnum {
		Some(p) => handle_portnum(topic, p, d).await,
		None => todo!(),
	}
}

async fn handle_payload_variant(topic: &str, v: PayloadVariant) {
	match v {
		PayloadVariant::Decoded(d) => handle_decoded(topic, d).await,
		PayloadVariant::Encrypted(e) => {
			println!("  Encrypted = {:?}", e)
		}
	}
}

async fn handle_mesh_packet(topic: &str, p: MeshPacket) {
	match p.payload_variant {
		Some(v) => handle_payload_variant(topic, v).await,
		None => println!("  Decoded = {:?}", p),
	}
}

async fn handle_service_envelope(topic: &str, m: ServiceEnvelope) {
	match m.packet {
		Some(p) => handle_mesh_packet(topic, p).await,
		None => println!("  Decoded = {:?}", m),
	}
}

async fn handle_mesh_2_c(p: rumqttc::Publish) {
	println!("Received = {:?}, {:?}", p, p.payload);
	let topic = p.topic;
	let message: Result<ServiceEnvelope, prost::DecodeError> = ServiceEnvelope::decode(p.payload.clone());
	match message {
		Ok(m) => handle_service_envelope(&topic, m).await,
		Err(e) => println!("  Error = {:?}", e),
	}
}

async fn handle_incoming_publish(p: rumqttc::Publish) {
	if p.topic.starts_with("msh/2/c/") {
		handle_mesh_2_c(p).await;
	} else {
		println!("Received = {:?}, {:?}", p, p.payload);
	}
	println!()
}
