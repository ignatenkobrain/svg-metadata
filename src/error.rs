use roxmltree::Error as XMLError;
use std::error::Error;
use std::fmt;
use std::io::Error as IoError;
use std::num::ParseFloatError;

#[derive(Debug)]
/// The error type of the library, which gets
/// returned on parsing issues.
pub struct MetadataError {
    details: String,
}

impl MetadataError {
    pub fn new(msg: &str) -> MetadataError {
        MetadataError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MetadataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MetadataError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<ParseFloatError> for MetadataError {
    fn from(_: ParseFloatError) -> MetadataError {
        MetadataError::new("Cannot convert string to float")
    }
}

impl From<IoError> for MetadataError {
    fn from(e: IoError) -> MetadataError {
        MetadataError::new(&e.to_string())
    }
}

impl From<XMLError> for MetadataError {
    fn from(e: XMLError) -> MetadataError {
        MetadataError::new(&e.to_string())
    }
}
