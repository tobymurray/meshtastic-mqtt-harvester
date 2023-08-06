use std::io::Result;

fn main() -> Result<()> {
	prost_build::compile_protos(&["protobufs/meshtastic/mqtt.proto"], &["protobufs/"])?;
	Ok(())
}
