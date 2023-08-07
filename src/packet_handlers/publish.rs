use crate::protobufs::meshtastic::{
	int_to_portnum, mesh_packet::PayloadVariant, Data, PortNum, Position, ServiceEnvelope, Telemetry,
};
use crate::utils::get_user_id;
use chrono::{DateTime, NaiveDateTime, Utc};
use prost::Message;

const COORDINATE_MULTIPLIER: f64 = 0.0000001;

pub async fn handle(publish_packet: rumqttc::Publish) -> Result<(), prost::DecodeError> {
	let message = ServiceEnvelope::decode(publish_packet.payload)?;

	if let Some(packet) = message.packet {
		match packet.payload_variant {
			Some(PayloadVariant::Decoded(d)) => {
				let portnum = int_to_portnum(d.portnum);

				match portnum {
					Some(p) => handle_portnum(&publish_packet.topic, p, d).await,
					None => todo!(),
				}
			}
			Some(PayloadVariant::Encrypted(_)) => (), // Nothing to do with encrypted packets
			None => println!("  MeshPacket = {:?}", packet),
		}
	}
	Ok(())
}

async fn handle_portnum(topic: &str, p: PortNum, d: Data) {
	match p {
		PortNum::TextMessageApp => {
			println!("  Decoded = {:?}", d);
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
			match Telemetry::decode(d.payload.as_ref()) {
				Ok(t) => {
					println!("    {:?}", t)
				}
				Err(e) => println!("  Error = {:?}", e),
			}
		}
		_ => {
			println!("  Decoded = {:?}", d);
		}
	}
	println!();
}

async fn handle_position(topic: &str, d: Data) {
	println!("  Decoded = {:?}", d);
	match Position::decode(d.payload.as_ref()) {
		Ok(p) => {
			let timestamp = NaiveDateTime::from_timestamp_opt(p.time.into(), 0);

			let user_id = get_user_id(topic).unwrap_or("UNKNOWN CLIENT");

			let latitude = p.latitude_i as f64 * COORDINATE_MULTIPLIER;
			let longitude = p.longitude_i as f64 * COORDINATE_MULTIPLIER;

			println!("    Position = {user_id:?}: ({longitude:.5}, {latitude:.5})");
			match timestamp {
				Some(t) => {
					let datetime = DateTime::<Utc>::from_utc(t, Utc);
					let est_tz = chrono_tz::Tz::America__New_York;
					let est_datetime = datetime.with_timezone(&est_tz);
					println!("      Datetime = {:?}", est_datetime);
					crate::postgres::insert_location(user_id, latitude, longitude, datetime).await;
				}
				None => println!("      Can't parse timestamp {:?}", p.time),
			}
		}
		Err(e) => println!("  Error = {:?}", e),
	}
}
