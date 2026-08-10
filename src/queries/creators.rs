use bon::Builder;
use serde::Serialize;

use crate::{CivitAI, Method, Query, models::{Creator, Paginated}, queries::impl_builder_send};

#[derive(Serialize, Builder)]
#[builder(on(String, into))]
pub struct ListCreators<'a> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'a CivitAI>,

	pub limit: Option<u32>,
	pub page: Option<u32>,
	pub query: Option<String>,
}

impl_builder_send!(list_creators_builder, ListCreatorsBuilder, ListCreators<'a>);

impl Method for ListCreators<'_> {
	type Input = Self;
	type Output = Paginated<Creator>;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/creators";
}

#[cfg(test)]
mod tests {
	use crate::tests::*;

	#[tokio::test]
	#[cfg(feature = "network-tests")]
	async fn online_list_creators() -> Result<(), Box<dyn Error>> {
		CivitAI::new()?.list_creators()
			.limit(10)
			.query("test")
			.send().await?;

		Ok(())
	}

	#[tokio::test]
	async fn mock_list_creators() -> Result<(), Box<dyn Error>> {
		mock_client!("GET", "/api/v1/creators?limit=10&query=test", list_creators, {
			CivitAI::new_auth(TOKEN)?.list_creators()
				.limit(10)
				.query("test")
				.send().await?;
		})
	}
}