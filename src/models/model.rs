use std::borrow::Cow;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use url::Url;

use crate::{AIR, Ecosystem, HasAir, Source, enums::{Availability, BaseModel, BaseModelType, ModelFileType, ModelType, ModerationStatus, NsfwLevel, PublishingStatus, ResourceType, UploadType, Usage}, models::{GenerationMetadata, files::{File, Hashes}}};

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Model {
	pub id: i64,
	pub name: String,
	pub description: String,

	#[serde(rename = "type")]
	pub model_type: ModelType,

	pub nsfw: bool,
	pub nsfw_level: NsfwLevel,
	pub availability: Availability,
	pub supports_generation: bool,
	pub allow_no_credit: bool,
	pub allow_commercial_use: Vec<Usage>,
	pub allow_derivatives: bool,
	pub allow_different_license: bool,
	pub minor: bool,
	pub poi: bool,
	pub sfw_only: bool,

	#[serde(default)]
	pub mode: ModerationStatus,

	pub stats: ModelStats,
	pub creator: ModelCreator,
	pub tags: Vec<String>,

	pub model_versions: Vec<ModelVersionEntry>,
}

impl HasAir for Model {
	fn air(&self) -> Cow<'_, AIR> {
		let ecosystem = if let Some(version_entry) = self.model_versions.first() {
			version_entry.base_model.clone().into()
		} else {
			Ecosystem::Other
		};

		let resource_type = self.model_type.clone().into();

		Cow::Owned(AIR {
			ecosystem,
			resource_type,
			source: Source::CivitAI,
			id: self.id.to_string(),
			version: None,
			file_id: None,
			format: None,
		})
	}
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct ModelStats {
	pub download_count: u64,
	pub thumbs_up_count: u64,
	pub thumbs_down_count: u64,
	pub comment_count: u64,
	pub tipped_amount_count: u64,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ModelCreator {
	pub username: String,
	pub image: Option<Url>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModelVersionEntry {
	// Fields taken from documentation
	pub id: i64,
	pub index: u32,
	pub name: String,
	pub base_model: BaseModel,
	pub base_model_type: BaseModelType,
	pub published_at: DateTime<Utc>,
	pub stats: ModelStats,

	pub download_url: Url,
	pub files: Vec<File>,

	// Fields not documented but present in the API response
	pub availability: Availability,
	pub description: Option<String>,
	pub trained_words: Option<Vec<String>>,
	pub vae_id: Option<u64>,
	pub paid_access: Option<PaidAccessInfo>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum PaidAccessInfo {
	Enabled(bool),

	#[serde(rename_all = "camelCase")]
	Detailed {
		permanent: bool,
		ends_at: DateTime<Utc>
	}
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModelVersion {
	pub id: i64,
	pub model_id: i64,
	pub name: String,
	pub description: Option<String>,
	pub base_model: BaseModel,
	pub base_model_type: BaseModelType,
	pub air: AIR,
	pub status: PublishingStatus,
	
	#[serde(default)]
	pub availability: Availability,
	
	pub nsfw_level: NsfwLevel,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	pub published_at: DateTime<Utc>,
	pub upload_type: UploadType,
	pub usage_control: Usage,
	pub trained_words: Vec<String>,

	// pub early_access_config: ???
	pub early_access_ends_at: Option<DateTime<Utc>>,
	// pub training_status: ???
	// pub training_details: ???

	pub stats: ModelStats,
	pub model: VersionModelInfo,

	pub files: Vec<File>,
	pub images: Vec<ModelImageEntry>,

	pub download_url: Url,
}

impl HasAir for ModelVersion {
	fn air(&self) -> Cow<'_, AIR> {
		Cow::Borrowed(&self.air)
	}
}

impl ModelVersion {
	pub fn primary_file(&self) -> Option<&File> {
		self.files.iter().find(|f| f.primary)
	}
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VersionModelInfo {
	pub name: String,
	#[serde(rename = "type")]
	pub model_type: ModelType,
	pub nsfw: bool,
	pub poi: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModelImageEntry {
	pub url: Url,
	pub nsfw_level: NsfwLevel,
	pub width: u32,
	pub height: u32,
	pub hash: String,
	#[serde(rename = "type")]
	pub resource_type: ResourceType,
	pub minor: bool,
	pub poi: bool,
	pub meta: Option<GenerationMetadata>,
	pub availability: Availability,
	pub has_meta: bool,
	pub has_positive_prompt: bool,
	pub on_site: bool,
	pub remix_of_id: Option<i64>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModelVersionMinimal {
	pub air: AIR,
	pub version_name: String,
	pub model_name: String,
	pub base_model: BaseModel,
	pub availability: Availability,
	pub published_at: DateTime<Utc>,

	pub size: f32,
	pub file_type: ModelFileType,
	pub file_name: String,
	pub hashes: Hashes,
	pub download_urls: Vec<Url>,

	pub format: String,
	pub can_generate: bool,
	pub is_featured: bool,
	pub require_auth: bool,
	pub check_permission: bool,
	pub early_access_ends_at: Option<DateTime<Utc>>,
	pub free_trial_limit: Option<u32>,
	pub additional_resource_charge: bool,
	pub minor: bool,
	pub sfw_only: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModelVersionHashLookup {
	pub model_version_id: i64,
	pub hash: String,
}
