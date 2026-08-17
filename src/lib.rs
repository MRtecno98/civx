pub mod queries;
pub mod models;
pub mod enums;
pub mod reader;

mod method;
mod client;
mod air;
mod files;

pub use method::*;
pub use air::*;

pub use client::CivitAI;
pub use error::Result;
pub use files::hashes;

pub const API_BASE: &str = "https://civitai.com/";

#[cfg(test)]
mod tests;

mod error {
    use std::fmt::Display;

	pub type Result<T> = std::result::Result<T, Error>;

	#[derive(Debug)]
	pub enum Error {
		Request(reqwest::Error),
		MissingEnum(&'static str),
		QueryFormat(serde_url_params::Error),
		UrlParse(url::ParseError),
		Io(std::io::Error),
		NoVersionsPublished,
		MissingHash,
		HashMismatch { expected: String, actual: String },
		InvalidEndpoint,
		ClientNotSet,
		RequestNotSet,
	}

	impl Display for Error {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {			
			match self {
				Error::Request(e) => write!(f, "Request error: {}", e),
				Error::MissingEnum(e) => write!(f, "Missing enum: {}", e),
				Error::QueryFormat(e) => write!(f, "Query format error: {}", e),
				Error::UrlParse(e) => write!(f, "URL parse error: {}", e),
				Error::Io(e) => write!(f, "IO error: {}", e),
				Error::NoVersionsPublished => write!(f, "No versions published for this resource"),
				Error::MissingHash => write!(f, "Missing hash"),
				Error::HashMismatch { expected, actual } => write!(f, "Hash mismatch: expected {}, got {}", expected, actual),
				Error::InvalidEndpoint => write!(f, "Invalid endpoint"),
				Error::ClientNotSet => write!(f, "Client not set"),
				Error::RequestNotSet => write!(f, "Request not set"),
			}
		}
	}

	impl From<reqwest::Error> for Error {
		fn from(err: reqwest::Error) -> Self {
			Error::Request(err)
		}
	}

	impl From<serde_url_params::Error> for Error {
		fn from(err: serde_url_params::Error) -> Self {
			Error::QueryFormat(err)
		}
	}

	impl From<url::ParseError> for Error {
		fn from(err: url::ParseError) -> Self {
			Error::UrlParse(err)
		}
	}

	impl From<std::io::Error> for Error {
		fn from(err: std::io::Error) -> Self {
			Error::Io(err)
		}
	}

	impl std::error::Error for Error {}
}
