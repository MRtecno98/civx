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
    use std::{fmt::Display, io};

	use reqwest::Response;
	use serde::Deserialize;

	pub type Result<T> = std::result::Result<T, Error>;

	#[derive(Debug)]
	pub enum Error {
		Api(ApiError),
		Request(reqwest::Error),
		MissingEnum(&'static str, String),
		QueryFormat(serde_url_params::Error),
		UrlParse(url::ParseError),
		Io(io::Error),
		NoVersionsPublished,
		MissingHash,
		HashMismatch { expected: String, actual: String },
		InvalidEndpoint,
		ClientNotSet,
		RequestNotSet,
	}

	impl Error {
		pub fn missing_enum<T: 'static>(value: impl AsRef<str>) -> Self {
			Self::MissingEnum(std::any::type_name::<T>(), value.as_ref().to_string())
		}
	}

	impl Display for Error {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			match self {
				Error::Api(err) => write!(f, "{}", err),
				Error::Request(e) => write!(f, "Request error: {}", e),
				Error::MissingEnum(name, value) => write!(f, "Missing enum {}: '{}'", name, value),
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

	#[derive(Deserialize, Debug)]
	pub struct ApiError {
		#[serde(alias = "error")]
		pub message: String,

		pub code: Option<String>,
		pub issues: Option<Vec<String>>,
	}

	impl ApiError {
		pub async fn check(response: Response) -> Result<Response> {
			if response.status().is_success() {
				Ok(response)
			} else {
				Err(response.json::<ApiError>().await?.into())
			}
		}
	}

	impl Display for ApiError {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			if let Some(code) = &self.code {
				write!(f, "api error ({}): {}", code.to_uppercase(), self.message.to_lowercase())?;
			} else {
				write!(f, "api error: {}", self.message.to_lowercase())?;
			}

			if let Some(issues) = &self.issues {
				for issue in issues {
					write!(f, "\n\t- {}", issue)?;
				}
			}

			Ok(())
		}
	}

	impl From<ApiError> for Error {
		fn from(err: ApiError) -> Self {
			Error::Api(err)
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
