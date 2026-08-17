use bon::Builder;
use serde::Serialize;

use crate::{CivitAI, Method, Query, models::{Paginated, Tag}, queries::impl_builder_send};

#[derive(Serialize, Builder)]
#[builder(on(String, into))]
pub struct ListTags<'a> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'a CivitAI>,
	
	pub limit: Option<u32>,
	pub page: Option<u32>,
	pub query: Option<String>,
}

impl_builder_send!(list_tags_builder, ListTagsBuilder, ListTags<'a>);

impl Default for ListTags<'_> {
	fn default() -> Self {
		Self {
			_client: None,
			limit: Some(200),
			page: None,
			query: None,
		}
	}
}
	type Input = Self;
	type Output = Paginated<Tag>;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/tags";
}

#[cfg(test)]
mod tests {
	use crate::tests::*;

	#[tokio::test]
	#[cfg(feature = "network-tests")]
	async fn online_list_tags() -> Result<(), Box<dyn Error>> {
		CivitAI::new()?.list_tags()
			.limit(10)
			.send().await?;

		Ok(())
	}

	#[tokio::test]
	async fn mock_list_tags() -> Result<(), Box<dyn Error>> {
		mock_client!("GET", "/api/v1/tags?limit=10", list_tags, {
			CivitAI::new_auth(TOKEN)?.list_tags()
				.limit(10)
				.send().await?;
		})
	}
}
