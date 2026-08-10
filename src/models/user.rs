use serde::Deserialize;

use crate::enums::{MembershipTier, NsfwLevel, UserStatus};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUser {
	pub id: i64,
	pub username: String,

	#[serde(default)]
	pub tier: MembershipTier,

	pub status: UserStatus,
	pub is_member: bool,
	pub subscriptions: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserLookup {
	pub id: i64,
	pub username: String,

	#[serde(deserialize_with = "crate::enums::nsfw_from_str")]
	pub avatar_nsfw: NsfwLevel,
}
