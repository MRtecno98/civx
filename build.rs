use std::{env, path::PathBuf};

use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde_json::Value;

const ENUMS_FILENAME: &str = "enums.rs";
const ENUMS_URL: &str = "https://civitai.com/api/v1/enums";
const JSON_FILENAME: &str = "enums.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let dest = PathBuf::from(
		env::var("OUT_DIR")
			.expect("OUT_DIR not found"))
		.join(ENUMS_FILENAME);

	let code = prettyplease::unparse(
		&syn::parse2(generate_enums().await?)?);

	std::fs::write(&dest, code)?;

	cargo_emit::rerun_if_changed!(JSON_FILENAME);

	let token_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR")
		.expect("CARGO_MANIFEST_DIR not found"))
		.join("tests/test_token");

	if token_path.exists() {
		let content = std::fs::read_to_string(&token_path)?;
		cargo_emit::rustc_env!("TEST_TOKEN", "{}", content.trim());
	}

	cargo_emit::rerun_if_changed!("tests");
	cargo_emit::rerun_if_changed!("tests/test_token");

	Ok(())
}

#[cfg(feature = "enums")]
async fn fetch_enums() -> Result<Value, Box<dyn std::error::Error>> {
	let response = reqwest::Client::new().get(ENUMS_URL)
		.timeout(std::time::Duration::from_secs(3)).send().await;

	if let Err(err) = &response && err.is_timeout() {
		if std::fs::exists(JSON_FILENAME)? {
			let json = std::fs::read_to_string(JSON_FILENAME)?;
			Ok(serde_json::from_str(&json)?)
		} else {
			Err("Failed to fetch enums and no cached JSON file found".into())
		}
	} else {
		let value = response?.error_for_status()?.json().await?;

		std::fs::write(JSON_FILENAME, 
			serde_json::to_string_pretty(&value)?)?;

		Ok(value)
	}
}

#[cfg(not(feature = "enums"))]
async fn fetch_enums() -> Result<Value, Box<dyn std::error::Error>> {
	Ok(if std::fs::exists(JSON_FILENAME)? {
		let json = std::fs::read_to_string(JSON_FILENAME)?;
		serde_json::from_str(&json)?
	} else {
		let value: Value = reqwest::get(ENUMS_URL).await?.json().await?;

		std::fs::write(JSON_FILENAME,
			serde_json::to_string_pretty(&value)?)?;

		value
	})
}

fn fix_enums(value: &mut Value) {
	if value["ActiveBaseModel"].as_array_mut().is_some_and(
			|m| m.contains(&Value::String("Krea 2".to_string()))) {
		value["ActiveBaseModel"].as_array_mut().unwrap().push("Krea2".into());
	}

	if value["BaseModel"].as_array_mut().is_some_and(
			|m| m.contains(&Value::String("Krea 2".to_string()))) {
		value["BaseModel"].as_array_mut().unwrap().push("Krea2".into());
	}
}

async fn generate_enums() -> Result<TokenStream, Box<dyn std::error::Error>> {
	let mut value = fetch_enums().await?;
	fix_enums(&mut value);

	let dict = value.as_object()
		.ok_or("Expected JSON object")?;

	let mut result = TokenStream::new();
	for (name, variants) in dict.iter() {
		result.extend(generate_enum(name, variants.as_array()
			.ok_or(format!("Expected array for enum '{}'", name))?
			.iter()
			.map(|v| v.as_str()
				.ok_or(format!("Expected string in array for enum '{}'", name))
				.map(|s| s.to_string()))
			.collect::<Result<Vec<String>, String>>()?
			.iter()
			.map(|s| s.as_str())
			.collect::<Vec<&str>>()
			.as_slice()));
	}

	Ok(result)
}

fn sanitize(s: &str) -> String {
	s.replace(|c: char| !c.is_ascii_alphanumeric() && c != '_', "")
}

fn generate_enum(name: &str, variants: &[&str]) -> TokenStream {
	let name_ident = format_ident!("{}", sanitize(name));
	let variants_ident = variants.iter()
		.map(|&v| format_ident!("{}", sanitize(v)))
		.collect::<Vec<_>>();

	// Because of fix_enums there may copies of the same ident for multiple strings
	// When going from Ident to String we use the deduped list and the normal one
	// for the reverse.
	let variants_ident_deduped = variants_ident.iter()
		.unique()
		.cloned()
		.collect::<Vec<_>>();

	quote! {
		#[derive(Debug, Clone, PartialEq, Eq)]
		pub enum #name_ident {
			#(#variants_ident_deduped,)*
			Unknown(String)
		}

		impl std::fmt::Display for #name_ident {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				match self {
					#(#name_ident::#variants_ident_deduped => write!(f, #variants),)*
					#name_ident::Unknown(s) => write!(f, "{}", s)
				}
			}
		}

		impl<T> From<T> for #name_ident where T: AsRef<str> {
			fn from(s: T) -> Self {
				match s.as_ref() {
					#(#variants => #name_ident::#variants_ident,)*
					_ => #name_ident::Unknown(s.as_ref().to_string())
				}
			}
		}

		impl<'de> serde::Deserialize<'de> for #name_ident {
			fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
			where
				D: serde::Deserializer<'de>,
			{
				let s: &'de str = serde::Deserialize::deserialize(deserializer)?;
				Ok(#name_ident::from(s))
			}
		}

		impl serde::Serialize for #name_ident {
			fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
			where
				S: serde::Serializer,
			{
				serializer.serialize_str(self.to_string().as_str())
			}
		}
	}
}
