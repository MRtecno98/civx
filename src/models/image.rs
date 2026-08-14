use chrono::{DateTime, Utc};
use serde::Deserialize;
use url::Url;

use crate::enums::{BaseModel, MediaType, ModelType, NsfwLevel};

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Image {
	pub id: i64,
	pub url: Url,

	#[serde(deserialize_with = "zero_or_string")]
	pub hash: Option<String>,

	pub width: u32,
	pub height: u32,

	#[serde(rename = "type")]
	pub media_type: MediaType,
	
	pub nsfw: bool,
	// pub nsfw_level: String // Deprecated
	pub browsing_level: NsfwLevel,
	pub created_at: DateTime<Utc>,

	pub post_id: i64,
	pub username: String,

	pub base_model: Option<BaseModel>,
	pub model_version_ids: Vec<u64>,

	pub stats: ImageStats,

	pub meta: Option<GenerationMetadata>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GenerationMetadata {
	#[serde(rename = "Size")]
	pub size: Option<String>,
	pub seed: Option<u64>,
	pub steps: Option<u32>,
	pub sampler: Option<String>,
	pub cfg_scale: Option<f32>,
	pub clip_skip: Option<u32>,
	pub prompt: Option<String>,
	pub negative_prompt: Option<String>,

	#[serde(default, rename = "civitaiResources")]
	pub resources: Vec<ImageResource>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImageResource {
	#[serde(rename = "type")]
	pub file_type: ModelType,

	pub weight: Option<f32>,
	pub model_version_id: i64,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ImageStats {
	pub cry_count: u64,
	pub laugh_count: u64,
	pub like_count: u64,
	pub dislike_count: u64,
	pub heart_count: u64,
	pub comment_count: u64,
}

fn zero_or_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
	use serde::de::Error;

	#[derive(Deserialize)]
	#[serde(untagged)]
    enum StringOrZero {
		String(String),
		Zero(u8),
	}

	match StringOrZero::deserialize(deserializer)? {
		StringOrZero::String(s) => Ok(Some(s)),
		StringOrZero::Zero(0) => Ok(None),
		_ => Err(Error::custom("expected a string or zero")),
	}
}
