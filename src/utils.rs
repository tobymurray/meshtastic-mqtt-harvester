use crate::meshtastic::PortNum;

pub fn int_to_portnum(portnum: i32) -> Option<PortNum> {
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

// Takes a topic that looks like "msh/2/c/LongFast/!abf849b0" and returns the user_id
pub fn get_user_id(topic: &str) -> Option<&str> {
	let parts: Vec<&str> = topic.split('/').collect();
	parts.last().cloned()
}
