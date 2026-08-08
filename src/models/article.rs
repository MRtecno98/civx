use chrono::{DateTime, Utc};
use serde::Deserialize;
use url::Url;

use crate::enums::{Availability, NsfwLevel, PublishingStatus};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Article {
	pub id: u64,
	pub title: String,
	pub published_at: DateTime<Utc>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	pub nsfw_level: NsfwLevel,
	pub availability: Availability,
	pub status: PublishingStatus,
	pub stats: ArticleStats,
	pub user: ArticleUser,
	pub tags: Vec<ArticleTag>,
	pub cover_image: CoverImage,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArticleStats {
	pub favorite_count: u64,
	pub collected_count: u64,
	pub comment_count: u64,
	pub like_count: u64,
	pub heart_count: u64,
	pub view_count: u64,
	pub tipped_amount_count: u64,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CoverImage {
	pub id: u64,
	pub url: Url,
	pub nsfw_level: NsfwLevel,
	pub width: u32,
	pub height: u32,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArticleUser {
	pub id: u64,
	pub username: String,
	pub image: Url,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArticleTag {
	pub id: u64,
	pub name: String,
	pub is_category: bool,
}
