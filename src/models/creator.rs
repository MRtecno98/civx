use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
	pub username: String,
	pub model_count: u64,
	pub link: Url,
	pub image: Url,
}
