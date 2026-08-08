use chrono::{DateTime, Utc};
use serde::Deserialize;
use url::Url;

use crate::enums::{BaseModel, MediaType, ModelType, NsfwLevel};

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Image {
	pub id: u64,
	pub url: Url,
	pub hash: String,
	pub width: u32,
	pub height: u32,

	#[serde(rename = "type")]
	pub media_type: MediaType,
	
	pub nsfw: bool,
	// pub nsfw_level: String // Deprecated
	pub browsing_level: NsfwLevel,
	pub created_at: DateTime<Utc>,

	pub post_id: u64,
	pub username: String,

	pub base_model: BaseModel,
	pub model_version_ids: Vec<u64>,

	pub stats: ImageStats,

	pub meta: GenerationMetadata,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GenerationMetadata {
	#[serde(rename = "Size")]
	pub size: String,
	pub seed: u64,
	pub steps: u32,
	pub sampler: String,
	pub cfg_scale: f32,
	pub clip_skip: u32,
	pub prompt: String,
	pub negative_prompt: String,

	#[serde(rename = "civitaiResources")]
	pub resources: Vec<ImageResource>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImageResource {
	#[serde(rename = "type")]
	pub file_type: ModelType,

	pub weight: Option<f32>,
	pub model_version_id: u64,
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
