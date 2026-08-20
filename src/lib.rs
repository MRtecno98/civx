//! # CivX
//! 
//! An asynchronous Rust client for the new 
//! [CivitAI site api](https://developer.civitai.com/site/).
//!
//! CivX provides Rust structs for most API calls and responses, with support 
//! for pagination, authentication, and file download and verification.
//! 
//! Available methods can be found in the [`queries`] module, 
//! while the returned data structures are in the [`models`] module.
//! For more information about available methods and authentication check out the 
//! [official documentation](https://developer.civitai.com/site/reference/).
//! 
//! ### Authentication
//! Authentication is required for some endpoints, and can be performed by providing a
//! bearer token to the [`CivitAI`] client. You can obtain a token from your
//! user page on CivitAI.
//! 
//! ### Pagination
//! This crate supports both cursor and page-based pagination. Either may be more 
//! suited to a particular use case, you can check out both the 
//! [reference](https://developer.civitai.com/site/reference/#pagination) and the 
//! [`models`] documentation for more information.
//! 
//! ### File download and verification
//! Downloading is performed through the [`File`](crate::models::File) struct, 
//! which calculates the hash in-flight while streaming the download to the destination,
//! and verifies it against the expected hash.
//! 
//! ---
//! 
//! ## Feature flags
//! - `enums`: Downloads and generates code for all available enums (such as base 
//!   models, model types, file types, etc.) from the API itself at compile time and 
//!   generates Rust wrappers to use them in requests. For library consumers this
//!   guarantees having an up-to-date enum list at the cost of a network call for each 
//!   compilation. *Requires a network connection at compile time.*
//! 
//! Other feature flags are used for development and not a concern for library consumers.

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

	/// Error types returned by CivX.
	#[derive(Debug)]
	pub enum Error {
		/// An error returned by the CivitAI API.
		/// see [ApiError](crate::error::ApiError) for more details.
		Api(ApiError),

		/// An error raised by the underlying HTTP client.
		Request(reqwest::Error),

		/// A value treated as a particular enum was not found in the enum list.
		/// 
		/// ### Fields
		/// - `&'static str`: The name of the enum type.
		/// - `String`: The missing value.
		MissingEnum(&'static str, String),

		/// An error occurred while serializing a request 
		/// into an URL query string.
		QueryFormat(serde_url_params::Error),

		/// An error occurred while parsing a URL.
		UrlParse(url::ParseError),

		/// An error occurred while performing an IO operation.
		Io(io::Error),

		/// No versions were published for a resource 
		/// that was expected to have at least one.
		NoVersionsPublished,

		/// The hash of a file was missing and verification was required.
		MissingHash,

		/// The hash verification of a file failed.
		HashMismatch { expected: String, actual: String },

		/// A request with a base url different from 
		/// [API_BASE](crate::API_BASE) was attempted by the client.
		InvalidEndpoint,

		/// A client reference is missing from a request or paginated response,
		/// this mostly happens if the type was constructed manually instead of through the client.
		ClientNotSet,

		/// A reference to a request is missing from a paginated response, 
		/// this mostly happens if the type was constructed manually instead of through the client.
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
				Error::Api(err) => write!(f, "{err}"),
				Error::Request(e) => write!(f, "Request error: {e}"),
				Error::MissingEnum(name, value) => write!(f, "Missing enum {name}: '{value}'"),
				Error::QueryFormat(e) => write!(f, "Query format error: {e}"),
				Error::UrlParse(e) => write!(f, "URL parse error: {e}"),
				Error::Io(e) => write!(f, "IO error: {e}"),
				Error::NoVersionsPublished => write!(f, "No versions published for this resource"),
				Error::MissingHash => write!(f, "Missing hash"),
				Error::HashMismatch { expected, actual } => write!(f, "Hash mismatch: expected {expected}, got {actual}"),
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
					write!(f, "\n\t- {issue}")?;
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
