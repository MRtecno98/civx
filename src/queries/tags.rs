use bon::Builder;
use civx_derive::civx;
use serde::Serialize;

use crate::{CivitAI, Method, Query, models::{Page, Tag}, queries::{Paginate, PaginationView, impl_builder_send, paginated_post_req}};

#[civx(clap)]
#[derive(Serialize, Builder, Debug)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct ListTags<'c> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'c CivitAI>,
	
	pub limit: Option<u32>,
	pub page: Option<u32>,
	pub query: Option<String>,
}

impl_builder_send!(list_tags_builder, ListTagsBuilder, ListTags<'c>);

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

impl<'c> Paginate for ListTags<'c> {
	fn pagination(&mut self) -> Option<PaginationView<'_>> {
		Some(PaginationView {
			limit: Some(&mut self.limit),
			page: Some(&mut self.page),
			cursor: None,
		})
	}
}

impl<'c> Method<'c> for ListTags<'c> {
	type Input = Self;
	type Output = Page<'c, Tag, Self>;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/tags";
	
	paginated_post_req!();
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
		mock_client!("GET", "/api/v1/tags?limit=10", list_tags);

		CivitAI::new_auth(TOKEN)?.list_tags()
			.limit(10)
			.send().await?;

		Ok(())
	}
}
