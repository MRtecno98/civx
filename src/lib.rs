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
		Init(reqwest::Error),
		MissingEnum(&'static str),
		QueryFormat(serde_url_params::Error),
		UrlParse(url::ParseError),
		ClientNotSet,
	}

	impl Display for Error {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			match self {
				Error::Init(e) => write!(f, "Client initialization error: {}", e),
				Error::MissingEnum(e) => write!(f, "Missing enum: {}", e),
				Error::QueryFormat(e) => write!(f, "Query format error: {}", e),
				Error::UrlParse(e) => write!(f, "URL parse error: {}", e),
				Error::ClientNotSet => write!(f, "Client not set"),
			}
		}
	}

	impl From<reqwest::Error> for Error {
		fn from(err: reqwest::Error) -> Self {
			Error::Init(err)
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

	impl std::error::Error for Error {}
}
