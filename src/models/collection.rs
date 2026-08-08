use serde::Deserialize;
use url::Url;

use crate::enums::{Availability, ResourceType};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
	pub id: u64,
	pub name: String,
	pub description: String,

	#[serde(rename = "type")]
	pub resource_type: ResourceType,

	pub read: Availability,
	pub is_public: bool,
	pub item_count: u64,
	pub cover_image_url: Url,
	pub user: CollectionUser,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CollectionUser {
	pub id: u64,
	pub username: String,
}
