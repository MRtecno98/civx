//! This module contains most enums seen in the CivitAI API, such as base models, model types, file types, etc.
//! 
//! Most of these are generated from the API itself (through the [`GetEnums`](crate::queries::GetEnums) endpoint), 
//! and are kept mostly up-to-date with the API in successive library releases. 
//! 
//! If you need to ensure that you have the latest enums available you can enable the `enums` feature flag, 
//! which will download the latest enums at compile time and generate Rust code for them. Note that this will perform
//! a network request on each compilation. Alternatively, each generated enum has a `Unknown(String)` variant that can be 
//! used to represent any unknown value, which is useful for forward compatibility with the API.
//! 
//! Some of the enums are missing from the API endpoint and as such are hardcoded into the library though usually
//! these are not expected to change much.
//! 
//! All enums implement `From<&str>` and `TryFrom<&str>` and are de/serializable with serde.
//! 
//! # Examples
//! ```rust
//! # use civx::enums::BaseModel;
//! let base_model: BaseModel = "SDXL 1.0".parse().unwrap();
//! assert_eq!(base_model, BaseModel::SDXL10);
//! ```
//! <br/>
//! 
//! ```rust
//! # use civx::enums::ActiveBaseModel;
//! let active_base_model: ActiveBaseModel = "SD 1.5".parse().unwrap();
//! assert_eq!(active_base_model, ActiveBaseModel::SD15);
//! ```
//! <br/>
//! 
//! ```rust
//! # use civx::enums::BaseModel;
//! let some_cool_base_model: BaseModel = "SomeCoolBaseModel".parse().unwrap();
//! assert_eq!(some_cool_base_model, BaseModel::Unknown("SomeCoolBaseModel".to_string()));
//! ```
//! <br/>
//! 
//! ```rust, should_panic
//! # use civx::enums::ResourceType;
//! // This will panic because "SomeCoolResourceType" is not a known resource type 
//! // and hardcoded enums don't have an Unknown variant.
//! let resource_type: ResourceType = "SomeCoolResourceType".parse().unwrap();
//! ```

use std::{hash::Hash, str::FromStr};

use bitmask_enum::bitmask;
use serde::{Deserialize, Serialize};

use crate::error::Error;

include!(concat!(env!("OUT_DIR"), "/enums.rs"));

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckpointType {
	Standard,
	Trained,
	Merge
}

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Period {
	AllTime,
	Month,
	Week,
	Day,
}

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(into = "&str", try_from = "&str")]
pub enum SortKind {
	HighestRated,
	MostDownloaded,
	Newest,
}

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(into = "&str", try_from = "&str")]
pub enum ArticleSortKind {
	Newest,
	MostBookmarks,
	MostReactions,
	MostComments,
	MostCollected,
	RecentlyUpdated,
}

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(into = "&str", try_from = "&str")]
pub enum CollectionSortKind {
	Newest,
	MostFollowers,
}

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
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

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Usage {
	Image,
	Rent,
	Sell,
	RentCivit,
	Download,
}

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
	Image,
	Video,
	Audio,
}

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Availability {
	#[default]
	Public,
	// TODO: Doc doesn't say others
}

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModerationStatus {
	#[default]
	Healthy,
	Archived,
	TakenDown,
}

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScanResult {
	Success,
	Pending,
	// TODO: Doc doesn't say others
}

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PublishingStatus {
	Published,
	// TODO: Doc doesn't say others
}

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UploadType {
	Created,
	// TODO: Doc doesn't say others
}

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(into = "&str", try_from = "&str")]
pub enum ResourceType {
	Model,
	Image,
	Video,
	Article,
	Post,
}

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum MembershipTier {
	#[default]
	Free,
	Founder,
	Bronze,
	Silver,
	Gold,
}

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
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

impl FromStr for SortKind {
	type Err = Error;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		match value {
			"Highest Rated" => Ok(SortKind::HighestRated),
			"Most Downloaded" => Ok(SortKind::MostDownloaded),
			"Newest" => Ok(SortKind::Newest),
			_ => Err(Error::missing_enum::<SortKind>(value)),
		}
	}
}

impl TryFrom<&str> for SortKind {
	type Error = Error;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		value.parse()
	}
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

impl FromStr for ArticleSortKind {
	type Err = Error;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
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

impl TryFrom<&str> for ArticleSortKind {
	type Error = Error;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		value.parse()
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

impl FromStr for CollectionSortKind {
	type Err = Error;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		match value {
			"Newest" => Ok(CollectionSortKind::Newest),
			"Most Followers" => Ok(CollectionSortKind::MostFollowers),
			_ => Err(Error::missing_enum::<CollectionSortKind>(value)),
		}
	}
}

impl TryFrom<&str> for CollectionSortKind {
	type Error = Error;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		value.parse()
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

impl FromStr for ImageSortKind {
	type Err = Error;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
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

impl TryFrom<&str> for ImageSortKind {
	type Error = Error;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		value.parse()
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

impl FromStr for ResourceType {
	type Err = Error;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
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

impl TryFrom<&str> for ResourceType {
	type Error = Error;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		value.parse()
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

impl FromStr for NsfwLevel {
	type Err = Error;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Ok(value.into())
	}
}

#[cfg(feature = "clap")]
impl clap::ValueEnum for NsfwLevel {
	fn value_variants<'a>() -> &'a [Self] {
		&[
			NsfwLevel::None,
			NsfwLevel::Soft,
			NsfwLevel::Mature,
			NsfwLevel::X,
		]
	}

	fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
		if self.contains(NsfwLevel::None) {
			Some(clap::builder::PossibleValue::new("none"))
		} else if self.contains(NsfwLevel::Soft) {
			Some(clap::builder::PossibleValue::new("soft"))
		} else if self.contains(NsfwLevel::Mature) {
			Some(clap::builder::PossibleValue::new("mature"))
		} else if self.contains(NsfwLevel::X) {
			Some(clap::builder::PossibleValue::new("x"))
		} else {
			None
		}
	}
}

impl Default for NsfwLevel {
	fn default() -> Self {
		NsfwLevel::None
	}
}

impl BaseModel {
	/// Returns true if the base model is an active base model (i.e. not unknown).
	/// 
	/// Note that if the enum list is outdated, this will return false for any new 
	/// active base models that are not yet in the enum list.
	/// 
	/// # Examples
	/// ```rust
	/// # use civx::enums::{BaseModel, ActiveBaseModel};
	/// let base_model: BaseModel = "SDXL".parse().unwrap();
	/// assert!(base_model.is_active());
	/// ```
	#[must_use]
	pub fn is_active(&self) -> bool {
		matches!(ActiveBaseModel::from(self.to_string()), ActiveBaseModel::Unknown(_))
	}
}
