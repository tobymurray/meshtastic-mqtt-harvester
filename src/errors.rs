use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum HarvesterError {
	BadTimestamp(u32),
	UnrecognizedPortNum(i32),
}

impl Error for HarvesterError {}

impl fmt::Display for HarvesterError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			HarvesterError::BadTimestamp(timestamp) => write!(f, "Can't parse seconds '{timestamp}' as seconds"),
			HarvesterError::UnrecognizedPortNum(portnum) => write!(f, "Unrecognized port num: '{portnum}'"),
		}
	}
}
