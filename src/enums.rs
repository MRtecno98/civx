use bitmask_enum::bitmask;
use serde::{Deserialize, Deserializer, Serialize};

use crate::error::Error;

include!(concat!(env!("OUT_DIR"), "/enums.rs"));

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum CheckpointType {
	Standard,
	Trained,
	Merge
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Period {
	AllTime,
	Month,
	Week,
	Day,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(into = "&str", try_from = "&str")]
pub enum SortKind {
	HighestRated,
	MostDownloaded,
	Newest,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(into = "&str", try_from = "&str")]
pub enum ArticleSortKind {
	Newest,
	MostBookmarks,
	MostReactions,
	MostComments,
	MostCollected,
	RecentlyUpdated,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(into = "&str", try_from = "&str")]
pub enum CollectionSortKind {
	Newest,
	MostFollowers,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(into = "&str", try_from = "&str")]
pub enum ImageSortKind {
	MostReactions,
	MostComments,
	MostCollected,
	Newest,
	Oldest,
	Random,
}

#[bitmask(u8)]
#[derive(Serialize, Deserialize)]
#[serde(into = "u8", from = "u8")]
pub enum NsfwLevel {
	None,
	Soft,
	Mature,
	X,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Usage {
	Image,
	Rent,
	Sell,
	RentCivit,
	Download,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
	Image,
	Video,
	Audio,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub enum Availability {
	#[default]
	Public,
	// TODO: Doc doesn't say others
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub enum ModerationStatus {
	#[default]
	Healthy,
	Archived,
	TakenDown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ScanResult {
	Success,
	// TODO: Doc doesn't say others
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum PublishingStatus {
	Published,
	// TODO: Doc doesn't say others
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum UploadType {
	Created,
	// TODO: Doc doesn't say others
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(into = "&str", try_from = "&str")]
pub enum ResourceType {
	Model,
	Image,
	Video,
	Article,
	Post,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MembershipTier {
	#[default]
	Free,
	Founder,
	Bronze,
	Silver,
	Gold,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
	Active,
	Muted,
	Banned
}

impl From<SortKind> for &str {
	fn from(value: SortKind) -> Self {
		match value {
			SortKind::HighestRated => "Highest Rated",
			SortKind::MostDownloaded => "Most Downloaded",
			SortKind::Newest => "Newest",
		}
	}
}

impl TryFrom<&str> for SortKind {
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"Highest Rated" => Ok(SortKind::HighestRated),
			"Most Downloaded" => Ok(SortKind::MostDownloaded),
			"Newest" => Ok(SortKind::Newest),
			_ => Err(Error::missing_enum::<SortKind>(value)),
		}
	}

	type Error = Error;
}

impl From<ArticleSortKind> for &str {
	fn from(value: ArticleSortKind) -> Self {
		match value {
			ArticleSortKind::MostBookmarks => "Most Bookmarks",
			ArticleSortKind::MostReactions => "Most Reactions",
			ArticleSortKind::MostComments => "Most Comments",
			ArticleSortKind::MostCollected => "Most Collected",
			ArticleSortKind::Newest => "Newest",
			ArticleSortKind::RecentlyUpdated => "Recently Updated",
		}
	}
}

impl TryFrom<&str> for ArticleSortKind {
	type Error = Error;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"Most Bookmarks" => Ok(ArticleSortKind::MostBookmarks),
			"Most Reactions" => Ok(ArticleSortKind::MostReactions),
			"Most Comments" => Ok(ArticleSortKind::MostComments),
			"Most Collected" => Ok(ArticleSortKind::MostCollected),
			"Newest" => Ok(ArticleSortKind::Newest),
			"Recently Updated" => Ok(ArticleSortKind::RecentlyUpdated),
			_ => Err(Error::missing_enum::<ArticleSortKind>(value)),
		}
	}
}

impl From<CollectionSortKind> for &str {
	fn from(value: CollectionSortKind) -> Self {
		match value {
			CollectionSortKind::Newest => "Newest",
			CollectionSortKind::MostFollowers => "Most Followers",
		}
	}
}

impl TryFrom<&str> for CollectionSortKind {
	type Error = Error;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"Newest" => Ok(CollectionSortKind::Newest),
			"Most Followers" => Ok(CollectionSortKind::MostFollowers),
			_ => Err(Error::missing_enum::<CollectionSortKind>(value)),
		}
	}
}

impl From<ImageSortKind> for &str {
	fn from(value: ImageSortKind) -> Self {
		match value {
			ImageSortKind::MostReactions => "Most Reactions",
			ImageSortKind::MostComments => "Most Comments",
			ImageSortKind::MostCollected => "Most Collected",
			ImageSortKind::Newest => "Newest",
			ImageSortKind::Oldest => "Oldest",
			ImageSortKind::Random => "Random",
		}
	}
}

impl TryFrom<&str> for ImageSortKind {
	type Error = Error;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"Most Reactions" => Ok(ImageSortKind::MostReactions),
			"Most Comments" => Ok(ImageSortKind::MostComments),
			"Most Collected" => Ok(ImageSortKind::MostCollected),
			"Newest" => Ok(ImageSortKind::Newest),
			"Oldest" => Ok(ImageSortKind::Oldest),
			"Random" => Ok(ImageSortKind::Random),
			_ => Err(Error::missing_enum::<ImageSortKind>(value)),
		}
	}
}

impl From<ResourceType> for &str {
	fn from(value: ResourceType) -> Self {
		match value {
			ResourceType::Model => "model",
			ResourceType::Image => "image",
			ResourceType::Article => "article",
			ResourceType::Post => "post",
			ResourceType::Video => "video",
		}
	}
}

impl TryFrom<&str> for ResourceType {
	type Error = Error;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match &value.to_lowercase()[..] {
			"model" => Ok(ResourceType::Model),
			"image" => Ok(ResourceType::Image),
			"article" => Ok(ResourceType::Article),
			"post" => Ok(ResourceType::Post),
			"video" => Ok(ResourceType::Video),
			_ => Err(Error::missing_enum::<ResourceType>(value)),
		}
	}
}

impl From<&str> for NsfwLevel {
	fn from(value: &str) -> Self {
		match value {
			"None" => NsfwLevel::None,
			"Soft" => NsfwLevel::Soft,
			"Mature" => NsfwLevel::Mature,
			"X" => NsfwLevel::X,
			_ => NsfwLevel::None, // Default to None if unknown
		}
	}
}

impl Default for NsfwLevel {
	fn default() -> Self {
		NsfwLevel::None
	}
}

pub fn nsfw_from_str<'de, D>(deserializer: D) -> std::result::Result<NsfwLevel, D::Error>
where D: Deserializer<'de>, {
	let s = String::deserialize(deserializer)?;
	Ok(NsfwLevel::from(s.as_str()))
}
