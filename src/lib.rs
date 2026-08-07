pub mod queries;
pub mod enums;

mod method;
mod client;

pub use method::*;
pub use client::CivitAI;
pub use error::Result;

pub const API_BASE: &str = "https://civitai.com/";

#[cfg(test)]
mod tests;

mod error {
    use std::fmt::Display;

	pub type Result<T> = std::result::Result<T, Error>;

	#[derive(Debug)]
	pub enum Error {
		InitError(reqwest::Error),
		EnumError(&'static str),
		QueryFormatError(serde_url_params::Error),
		UrlError(url::ParseError),
		ClientNotSet,
	}

	impl Display for Error {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			match self {
				Error::InitError(e) => write!(f, "Client initialization error: {}", e),
				Error::EnumError(e) => write!(f, "Enum conversion error: {}", e),
				Error::QueryFormatError(e) => write!(f, "Query format error: {}", e),
				Error::UrlError(e) => write!(f, "URL error: {}", e),
				Error::ClientNotSet => write!(f, "Client not set"),
			}
		}
	}

	impl From<reqwest::Error> for Error {
		fn from(err: reqwest::Error) -> Self {
			Error::InitError(err)
		}
	}

	impl From<serde_url_params::Error> for Error {
		fn from(err: serde_url_params::Error) -> Self {
			Error::QueryFormatError(err)
		}
	}

	impl From<url::ParseError> for Error {
		fn from(err: url::ParseError) -> Self {
			Error::UrlError(err)
		}
	}

	impl std::error::Error for Error {}
}
