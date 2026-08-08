use serde::Deserialize;

use crate::enums::{MembershipTier, NsfwLevel, UserStatus};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUser {
	pub id: u64,
	pub username: String,
	pub tier: MembershipTier,
	pub status: UserStatus,
	pub is_member: bool,
	pub subscriptions: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UserLookup {
	pub id: u64,
	pub username: String,
	pub avatar_nsfw: NsfwLevel,
}
