use std::error::Error;

use crate::errors::HarvesterError::BadTimestamp;
use crate::mqtt::decrypt_data;
use crate::protobufs::meshtastic::{
	int_to_portnum, mesh_packet::PayloadVariant, Data, PortNum, Position, ServiceEnvelope, Telemetry,
};
use crate::protobufs::meshtastic::{telemetry, NodeInfo};
use chrono::{DateTime, NaiveDateTime, Utc};
use prost::Message;

const COORDINATE_MULTIPLIER: f64 = 0.0000001;

pub async fn handle(publish_packet: rumqttc::Publish) -> Result<(), Box<dyn Error>> {
	// Only care about raw protobuf packets
	if !publish_packet.topic.contains("/c/") {
		return Ok(());
	}

	let message = ServiceEnvelope::decode(publish_packet.payload)?;

	if let Some(packet) = message.packet {
		match packet.payload_variant {
			Some(PayloadVariant::Decoded(d)) => {
				let portnum = int_to_portnum(d.portnum)?;
				handle_portnum(packet.from, portnum, d).await?;
			}
			Some(PayloadVariant::Encrypted(e)) => {
				let data = decrypt_data(packet.id.into(), packet.from.into(), e);
				match data {
					Ok(Some(d)) => {
						let portnum = int_to_portnum(d.portnum)?;
						handle_portnum(packet.from, portnum, d).await?;
					}
					Ok(None) => (),
					Err(_) => (),
				};
			}
			None => println!("MeshPacket = {:?}", packet),
		}
	}
	Ok(())
}

async fn handle_portnum(node_num: u32, p: PortNum, d: Data) -> Result<(), Box<dyn Error>> {
	match p {
		PortNum::UnknownApp => handle_unknown(node_num, d).await?,
		PortNum::TextMessageApp => handle_text_message(node_num, d).await?,
		PortNum::RemoteHardwareApp => handle_remote_hardware(node_num, d).await?,
		PortNum::PositionApp => handle_position(node_num, d).await?,
		PortNum::NodeinfoApp => handle_nodeinfo(node_num, d).await?,
		PortNum::RoutingApp => handle_routing(node_num, d).await?,
		PortNum::AdminApp => handle_admin(node_num, d).await?,
		PortNum::TextMessageCompressedApp => handle_text_message_compressed(node_num, d).await?,
		PortNum::WaypointApp => handle_waypoint(node_num, d).await?,
		PortNum::AudioApp => handle_audio(node_num, d).await?,
		PortNum::DetectionSensorApp => handle_detection_sensor(node_num, d).await?,
		PortNum::ReplyApp => handle_reply(node_num, d).await?,
		PortNum::IpTunnelApp => handle_ip_tunnel(node_num, d).await?,
		PortNum::PaxcounterApp => handle_paxcounter(node_num, d).await?,
		PortNum::SerialApp => handle_serial(node_num, d).await?,
		PortNum::StoreForwardApp => handle_store_forward(node_num, d).await?,
		PortNum::RangeTestApp => handle_range_test(node_num, d).await?,
		PortNum::TelemetryApp => handle_telemetry(node_num, d).await?,
		PortNum::ZpsApp => handle_zps(node_num, d).await?,
		PortNum::SimulatorApp => handle_simulator(node_num, d).await?,
		PortNum::TracerouteApp => handle_traceroute(node_num, d).await?,
		PortNum::NeighborinfoApp => handle_neighborinfo(node_num, d).await?,
		PortNum::AtakPlugin => handle_atak_plugin(node_num, d).await?,
		PortNum::PrivateApp => handle_private(node_num, d).await?,
		PortNum::AtakForwarder => handle_atak_forwarder(node_num, d).await?,
		PortNum::Max => handle_max(node_num, d).await?,
	}
	Ok(())
}

async fn handle_max(_: u32, _: Data) -> Result<(), Box<dyn Error>> {
	// Seems like this is kind of meaningless as a portnum, it's just a constant for the upper limit
	Ok(())
}

async fn handle_atak_forwarder(_: u32, _: Data) -> Result<(), Box<dyn Error>> {
	// AndroidTacticalAssaultKit or Android Team Awareness Kit?
	Ok(())
}

async fn handle_private(_: u32, _: Data) -> Result<(), Box<dyn Error>> {
	// Custom data from users, nothinc constructive to do with this at this point
	Ok(())
}

async fn handle_atak_plugin(_: u32, _: Data) -> Result<(), Box<dyn Error>> {
	// AndroidTacticalAssaultKit or Android Team Awareness Kit?
	Ok(())
}

async fn handle_neighborinfo(_: u32, _d: Data) -> Result<(), Box<dyn Error>> {
	// let payload = NeighborInfo::decode(d.payload.as_ref())?;
	// println!("{payload:#?}");

	// looks like this:
	// 	NeighborInfo {
	//     node_id: 3806707028,
	//     last_sent_by_id: 3806630800,
	//     node_broadcast_interval_secs: 60,
	//     neighbors: [
	//         Neighbor {
	//             node_id: 3806671764,
	//             snr: 6.25,
	//             last_rx_time: 0,
	//             node_broadcast_interval_secs: 0,
	//         },
	//         Neighbor {
	//             node_id: 3806707612,
	//             snr: 5.75,
	//             last_rx_time: 0,
	//             node_broadcast_interval_secs: 0,
	//         },
	// 				...
	//     ],
	// }
	Ok(())
}

async fn handle_traceroute(_node_num: u32, _d: Data) -> Result<(), Box<dyn Error>> {
	Ok(())
}

async fn handle_simulator(_node_num: u32, _d: Data) -> Result<(), Box<dyn Error>> {
	Ok(())
}

async fn handle_zps(_: u32, _: Data) -> Result<(), Box<dyn Error>> {
	// Experimental tools for estimating node position without a GPS
	Ok(())
}

async fn handle_range_test(_node_num: u32, _d: Data) -> Result<(), Box<dyn Error>> {
	Ok(())
}

async fn handle_store_forward(_node_num: u32, _d: Data) -> Result<(), Box<dyn Error>> {
	Ok(())
}

async fn handle_serial(_node_num: u32, _d: Data) -> Result<(), Box<dyn Error>> {
	Ok(())
}

async fn handle_paxcounter(_node_num: u32, _d: Data) -> Result<(), Box<dyn Error>> {
	Ok(())
}

async fn handle_ip_tunnel(_node_num: u32, _d: Data) -> Result<(), Box<dyn Error>> {
	Ok(())
}

async fn handle_reply(_node_num: u32, _d: Data) -> Result<(), Box<dyn Error>> {
	Ok(())
}

async fn handle_detection_sensor(_node_num: u32, _d: Data) -> Result<(), Box<dyn Error>> {
	Ok(())
}

async fn handle_audio(_node_num: u32, _d: Data) -> Result<(), Box<dyn Error>> {
	Ok(())
}

async fn handle_waypoint(_node_num: u32, _d: Data) -> Result<(), Box<dyn Error>> {
	Ok(())
}

async fn handle_text_message_compressed(_node_num: u32, _d: Data) -> Result<(), Box<dyn Error>> {
	Ok(())
}

async fn handle_admin(_node_num: u32, _d: Data) -> Result<(), Box<dyn Error>> {
	Ok(())
}

async fn handle_routing(_node_num: u32, _d: Data) -> Result<(), Box<dyn Error>> {
	Ok(())
}

async fn handle_nodeinfo(_node_num: u32, d: Data) -> Result<(), Box<dyn Error>> {
	let payload = NodeInfo::decode(d.payload.as_ref())?;

	println!("{payload:#?}");
	Ok(())
}

async fn handle_remote_hardware(_node_num: u32, _d: Data) -> Result<(), Box<dyn Error>> {
	Ok(())
}

async fn handle_text_message(_node_num: u32, d: Data) -> Result<(), Box<dyn Error>> {
	println!("  TextMessage = {:?}", String::from_utf8(d.payload)?);

	Ok(())
}

async fn handle_unknown(_node_num: u32, _d: Data) -> Result<(), Box<dyn Error>> {
	Ok(())
}

async fn handle_position(node_num: u32, d: Data) -> Result<(), Box<dyn Error>> {
	// // Looks like:
	// {
	// 	latitude_i: 371344705,
	// 	longitude_i: -932760985,
	// 	altitude: 368,
	// 	time: 1707927571,
	// 	location_source: LocUnset,
	// 	altitude_source: AltUnset,
	// 	timestamp: 0,
	// 	timestamp_millis_adjust: 0,
	// 	altitude_hae: 0,
	// 	altitude_geoidal_separation: 0,
	// 	pdop: 0,
	// 	hdop: 0,
	// 	vdop: 0,
	// 	gps_accuracy: 0,
	// 	ground_speed: 0,
	// 	ground_track: 0,
	// 	fix_quality: 0,
	// 	fix_type: 0,
	// 	sats_in_view: 0,
	// 	sensor_id: 0,
	// 	next_update: 0,
	// 	seq_number: 0
	// }
	let p = Position::decode(d.payload.as_ref())?;
	println!("  Payload = {p:?}");
	let timestamp = NaiveDateTime::from_timestamp_opt(p.time.into(), 0).ok_or(BadTimestamp(p.time))?;

	let latitude = p.latitude_i as f64 * COORDINATE_MULTIPLIER;
	let longitude = p.longitude_i as f64 * COORDINATE_MULTIPLIER;

	let datetime = DateTime::<Utc>::from_utc(timestamp, Utc);
	let est_datetime = datetime.with_timezone(&chrono_tz::Tz::America__New_York);

	println!("  Position = {node_num:}: ({longitude:.5}, {latitude:.5}) @ {est_datetime}");

	crate::postgres::insert_location(&node_num.to_string(), latitude, longitude, datetime).await?;

	Ok(())
}

async fn handle_telemetry(node_num: u32, d: Data) -> Result<(), Box<dyn Error>> {
	let t = Telemetry::decode(d.payload.as_ref())?;
	let timestamp = NaiveDateTime::from_timestamp_opt(t.time.into(), 0).ok_or(BadTimestamp(t.time))?;
	let datetime = DateTime::<Utc>::from_utc(timestamp, Utc);

	match t.variant {
		Some(telemetry::Variant::DeviceMetrics(m)) => {
			println!("  Telemetry: {node_num}: {} {m:?}", t.time);
			crate::postgres::insert_telemetry(&node_num.to_string(), m.battery_level, m.voltage, datetime).await?;
		}
		Some(telemetry::Variant::EnvironmentMetrics(m)) => {
			println!("  Telemetry: {node_num:}: {} {m:?}", t.time)
		}
		Some(telemetry::Variant::AirQualityMetrics(m)) => {
			println!("  Telemetry: {node_num:}: {} {m:?}", t.time)
		}
		Some(telemetry::Variant::PowerMetrics(m)) => {
			println!("  Telemetry: {node_num:}: {} {m:?}", t.time)
		}
		None => {
			eprintln!("Encountered telemetry with no variant")
		}
	};

	Ok(())
}
