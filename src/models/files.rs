use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;
use url::Url;

use crate::enums::{ModelFileType, ScanResult};

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct File {
	pub id: i64,
	pub name: String,

	#[serde(rename = "type")]
	pub file_type: ModelFileType,

	#[serde(rename = "sizeKB")]
	pub size_kb: f32,
	pub metadata: HashMap<String, Value>,

	pub pickle_scan_result: ScanResult,
	pub virus_scan_result: ScanResult,

	pub hashes: Hashes,

	pub download_url: Url,

	#[serde(default)]
	pub primary: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Hashes {
	#[serde(rename = "AutoV1")]
	pub auto_v1: Option<String>,

	#[serde(rename = "AutoV2")]
	pub auto_v2: Option<String>,

	#[serde(rename = "AutoV3")]
	pub auto_v3: Option<String>,

	#[serde(rename = "SHA256")]
	pub sha256: Option<String>,

	#[serde(rename = "CRC32")]
	pub crc32: Option<String>,

	#[serde(rename = "BLAKE3")]
	pub blake3: Option<String>,
}
