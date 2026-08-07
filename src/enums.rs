use bitmask_enum::bitmask;
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum SortKind {
	HighestRated,
	MostDownloaded,
	Newest,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Period {
	AllTime,
	Month,
	Week,
	Day,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum CheckpointType {
	Standard,
	Trained,
	Merge
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
pub enum MediaType {
	Image,
	Video,
	Audio,
}

include!(concat!(env!("OUT_DIR"), "/enums.rs"));

impl AsRef<str> for CheckpointType {
	fn as_ref(&self) -> &str {
		match self {
			CheckpointType::Standard => "Standard",
			CheckpointType::Trained => "Trained",
			CheckpointType::Merge => "Merge",
		}
	}
}

impl TryFrom<&str> for CheckpointType {
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"Standard" => Ok(CheckpointType::Standard),
			"Trained" => Ok(CheckpointType::Trained),
			"Merge" => Ok(CheckpointType::Merge),
			_ => Err(Error::MissingEnum("Invalid checkpoint type")),
		}
	}

	type Error = Error;
}

impl AsRef<str> for Period {
	fn as_ref(&self) -> &str {
		match self {
			Period::AllTime => "AllTime",
			Period::Month => "Month",
			Period::Week => "Week",
			Period::Day => "Day",
		}
	}
}

impl TryFrom<&str> for Period {
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"AllTime" => Ok(Period::AllTime),
			"Month" => Ok(Period::Month),
			"Week" => Ok(Period::Week),
			"Day" => Ok(Period::Day),
			_ => Err(Error::MissingEnum("Invalid period")),
		}
	}

	type Error = Error;
}

impl AsRef<str> for SortKind {
	fn as_ref(&self) -> &str {
		match self {
			SortKind::HighestRated => "HighestRated",
			SortKind::MostDownloaded => "MostDownloaded",
			SortKind::Newest => "Newest",
		}
	}
}

impl TryFrom<&str> for SortKind {
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"HighestRated" => Ok(SortKind::HighestRated),
			"MostDownloaded" => Ok(SortKind::MostDownloaded),
			"Newest" => Ok(SortKind::Newest),
			_ => Err(Error::MissingEnum("Invalid sort kind")),
		}
	}

	type Error = Error;
}
