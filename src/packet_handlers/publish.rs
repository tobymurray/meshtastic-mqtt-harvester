use std::error::Error;

use crate::errors::HarvesterError::BadTimestamp;
use crate::protobufs::meshtastic::telemetry;
use crate::protobufs::meshtastic::{
	int_to_portnum, mesh_packet::PayloadVariant, Data, PortNum, Position, ServiceEnvelope, Telemetry,
};
use crate::utils::get_user_id;
use chrono::{DateTime, NaiveDateTime, Utc};
use prost::Message;

const COORDINATE_MULTIPLIER: f64 = 0.0000001;

pub async fn handle(publish_packet: rumqttc::Publish) -> Result<(), Box<dyn Error>> {
	let message = ServiceEnvelope::decode(publish_packet.payload)?;

	if let Some(packet) = message.packet {
		match packet.payload_variant {
			Some(PayloadVariant::Decoded(d)) => {
				let portnum = int_to_portnum(d.portnum)?;
				handle_portnum(&publish_packet.topic, portnum, d).await?;
			}
			Some(PayloadVariant::Encrypted(_)) => (),
			None => println!("MeshPacket = {:?}", packet),
		}
	}
	Ok(())
}

async fn handle_portnum(topic: &str, p: PortNum, d: Data) -> Result<(), Box<dyn Error>> {
	let user_id = get_user_id(topic).unwrap_or("UNKNOWN CLIENT");
	match p {
		PortNum::TextMessageApp => {
			println!("Decoded = {:?}", d);
			println!("  TextMessage = {:?}", String::from_utf8(d.payload)?);
		}
		PortNum::PositionApp => {
			println!("Decoded = {:?}", d);
			handle_position(user_id, d).await?
		}
		PortNum::TelemetryApp => {
			println!("Decoded = {:?}", d);
			handle_telemetry(user_id, d).await?
		}
		_ => println!("IGNORING {}", p.as_str_name()),
	}
	println!();
	Ok(())
}

async fn handle_position(user_id: &str, d: Data) -> Result<(), Box<dyn Error>> {
	let p = Position::decode(d.payload.as_ref())?;
	println!("  Payload = {p:?}");
	let timestamp = NaiveDateTime::from_timestamp_opt(p.time.into(), 0).ok_or(BadTimestamp(p.time))?;

	let latitude = p.latitude_i as f64 * COORDINATE_MULTIPLIER;
	let longitude = p.longitude_i as f64 * COORDINATE_MULTIPLIER;

	let datetime = DateTime::<Utc>::from_utc(timestamp, Utc);
	let est_datetime = datetime.with_timezone(&chrono_tz::Tz::America__New_York);

	println!("  Position = {user_id:}: ({longitude:.5}, {latitude:.5}) @ {est_datetime}");

	crate::postgres::insert_location(user_id, latitude, longitude, datetime).await?;

	Ok(())
}

async fn handle_telemetry(user_id: &str, d: Data) -> Result<(), Box<dyn Error>> {
	let t = Telemetry::decode(d.payload.as_ref())?;
	let timestamp = NaiveDateTime::from_timestamp_opt(t.time.into(), 0).ok_or(BadTimestamp(t.time))?;
	let datetime = DateTime::<Utc>::from_utc(timestamp, Utc);

	match t.variant {
		Some(telemetry::Variant::DeviceMetrics(m)) => {
			println!("  Telemetry: {user_id:}: {} {m:?}", t.time);
			crate::postgres::insert_telemetry(user_id, m.battery_level, m.voltage, datetime).await?;
		}
		Some(telemetry::Variant::EnvironmentMetrics(m)) => {
			println!("  Telemetry: {user_id:}: {} {m:?}", t.time)
		}
		Some(telemetry::Variant::AirQualityMetrics(m)) => {
			println!("  Telemetry: {user_id:}: {} {m:?}", t.time)
		}
		None => todo!(),
	};

	Ok(())
}
