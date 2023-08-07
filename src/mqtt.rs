use crate::packet_handlers::publish;
use rumqttc::Packet::{
	self, ConnAck, Connect, Disconnect, PingReq, PingResp, PubAck, PubComp, PubRec, PubRel, Publish, SubAck, Subscribe,
	UnsubAck, Unsubscribe,
};
use std::fmt::Debug;

fn handle_packet<T: Debug>(p: T) -> Result<(), prost::DecodeError> {
	println!("Received = {:?}", p);
	Ok(())
}

pub async fn handle_incoming(i: Packet) -> Result<(), prost::DecodeError> {
	match i {
		Publish(p) => publish::handle(p).await,
		Connect(p) => handle_packet(p),
		ConnAck(p) => handle_packet(p),
		PubAck(p) => handle_packet(p),
		PubRec(p) => handle_packet(p),
		PubRel(p) => handle_packet(p),
		PubComp(p) => handle_packet(p),
		Subscribe(p) => handle_packet(p),
		SubAck(p) => handle_packet(p),
		Unsubscribe(p) => handle_packet(p),
		UnsubAck(p) => handle_packet(p),
		Disconnect => {
			println!("Received = Disconnect packet");
			Ok(())
		}
		PingReq | PingResp => Ok(()),
	}
}
