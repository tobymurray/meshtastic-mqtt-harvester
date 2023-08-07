#![allow(non_snake_case)]

use crate::errors::HarvesterError;
// Something like target/debug/build/meshtastic-mqtt-harvester-<hash>/out/meshtastic.rs
include!(concat!(env!("OUT_DIR"), "/meshtastic.rs"));

pub fn int_to_portnum(portnum: i32) -> Result<PortNum, crate::errors::HarvesterError> {
	match portnum {
		0 => Ok(PortNum::UnknownApp),
		1 => Ok(PortNum::TextMessageApp),
		2 => Ok(PortNum::RemoteHardwareApp),
		3 => Ok(PortNum::PositionApp),
		4 => Ok(PortNum::NodeinfoApp),
		5 => Ok(PortNum::RoutingApp),
		6 => Ok(PortNum::AdminApp),
		7 => Ok(PortNum::TextMessageCompressedApp),
		8 => Ok(PortNum::WaypointApp),
		9 => Ok(PortNum::AudioApp),
		32 => Ok(PortNum::ReplyApp),
		33 => Ok(PortNum::IpTunnelApp),
		64 => Ok(PortNum::SerialApp),
		65 => Ok(PortNum::StoreForwardApp),
		66 => Ok(PortNum::RangeTestApp),
		67 => Ok(PortNum::TelemetryApp),
		68 => Ok(PortNum::ZpsApp),
		69 => Ok(PortNum::SimulatorApp),
		70 => Ok(PortNum::TracerouteApp),
		71 => Ok(PortNum::NeighborinfoApp),
		256 => Ok(PortNum::PrivateApp),
		257 => Ok(PortNum::AtakForwarder),
		511 => Ok(PortNum::Max),
		_ => Err(HarvesterError::UnrecognizedPortNum(portnum)),
	}
}
