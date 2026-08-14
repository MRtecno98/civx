use chrono::{DateTime, Utc};
use serde::Deserialize;
use url::Url;

use crate::{enums::{Availability, NsfwLevel, PublishingStatus}, models::Image};

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ArticleInfo {
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

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Article {
	#[serde(flatten)]
	pub info: ArticleInfo,
	pub content: String,
	// TODO: There's way more undocumented fields here
}

impl ArticleInfo {
	pub async fn get_content(&self, client: &crate::CivitAI) -> crate::Result<String> {
		Ok(client.get_article(self.id).await?.content)
	}
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArticleStats {
	#[serde(alias = "favoriteCountAllTime")]
	pub favorite_count: u64,
	#[serde(alias = "collectedCountAllTime")]
	pub collected_count: u64,
	#[serde(alias = "commentCountAllTime")]
	pub comment_count: u64,
	#[serde(alias = "likeCountAllTime")]
	pub like_count: u64,
	#[serde(alias = "heartCountAllTime")]
	pub heart_count: u64,
	#[serde(alias = "viewCountAllTime")]
	pub view_count: u64,
	#[serde(alias = "tippedAmountCountAllTime")]
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
