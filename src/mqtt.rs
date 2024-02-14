use crate::{packet_handlers::publish, protobufs::meshtastic::Data};
use aes::cipher::KeyIvInit;
use aes::cipher::StreamCipher;
use crypto::common::generic_array::GenericArray;
use prost::DecodeError;
use prost::Message;
use rumqttc::Packet::{
	self, ConnAck, Connect, Disconnect, PingReq, PingResp, PubAck, PubComp, PubRec, PubRel, Publish, SubAck, Subscribe,
	UnsubAck, Unsubscribe,
};
use std::{error::Error, fmt::Debug};

type Aes128Ctr128LE = ctr::Ctr128LE<aes::Aes128>;

fn handle_packet<T: Debug>(_: T) -> Result<(), Box<dyn Error>> {
	// println!("Received = {:?}", p);
	Ok(())
}

pub async fn handle_incoming(i: Packet) -> Result<(), Box<dyn Error>> {
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

pub fn decrypt_data(packet_id: u64, from_node_id: u64, encrypted_data: Vec<u8>) -> Result<Option<Data>, DecodeError> {
	let decrypted = decrypt_payload(packet_id, from_node_id, encrypted_data);

	if decrypted.is_empty() {
		return Ok(None);
	}

	let data = Data::decode(decrypted.as_slice())?;
	Ok(Some(data))
}

fn decrypt_payload(packet_id: u64, from_node_id: u64, mut encrypted_data: Vec<u8>) -> Vec<u8> {
	let default_key: [u8; 16] = [
		0xd4, 0xf1, 0xbb, 0x3a, 0x20, 0x29, 0x07, 0x59, 0xf0, 0xbc, 0xff, 0xab, 0xcf, 0x4e, 0x69, 0x01,
	]; // Redacted
	let default_key = GenericArray::from_slice(&default_key);

	// Convert packet_id and node_id to little-endian byte arrays
	let nonce_packet_id = packet_id.to_le_bytes();
	let nonce_from_node = from_node_id.to_le_bytes();

	let mut nonce = vec![0; 16];

	// Copy the lower 8 bytes from nonce_packet_id
	nonce[..8].copy_from_slice(&nonce_packet_id);

	// Copy the next 8 bytes from nonce_from_node
	nonce[8..16].copy_from_slice(&nonce_from_node);

	let nonce = GenericArray::from_slice(&nonce);

	let mut cipher = Aes128Ctr128LE::new(default_key, nonce);
	for chunk in encrypted_data.chunks_mut(3) {
		cipher.apply_keystream(chunk);
	}

	encrypted_data
}
