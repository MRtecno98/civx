use chrono::{DateTime, Utc};
use serde::Deserialize;
use url::Url;

use crate::{enums::{Availability, NsfwLevel, PublishingStatus}, models::Image};

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Article {
	pub id: i64,
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
	pub cover_image: Option<CoverImage>,
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
	pub id: i64,
	pub url: String, // It's just an UUID
	pub nsfw_level: NsfwLevel,
	pub width: u32,
	pub height: u32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ArticleUser {
	pub id: i64,
	pub username: Option<String>,

	#[serde(flatten)]
	pub image: Option<ArticleUserPropic>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ArticleUserPropic {
	#[serde(rename_all = "camelCase")]
	Image {
		image: Url,
	},

	#[serde(rename_all = "camelCase")]
	Detailed {
		profile_picture: Option<Box<Image>>,
	}
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArticleTag {
	pub id: i64,
	pub name: String,
	pub is_category: bool,
}
