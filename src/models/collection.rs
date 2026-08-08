use serde::Deserialize;
use url::Url;

use crate::enums::{Availability, ResourceType};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
	pub id: i64,
	pub name: String,
	pub description: String,

	#[serde(rename = "type")]
	pub resource_type: ResourceType,

	pub read: Availability,
	pub is_public: bool,
	pub item_count: Option<u64>,
	pub cover_image_url: Option<Url>,
	pub user: CollectionUser,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CollectionUser {
	pub id: i64,
	pub username: String,
}
