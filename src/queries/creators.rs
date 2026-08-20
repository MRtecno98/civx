use bon::Builder;
use serde::Serialize;

use crate::{CivitAI, Method, Query, models::{Creator, Page}, queries::{Paginate, PaginationView, impl_builder_send, paginated_post_req}};

#[derive(Serialize, Builder)]
#[builder(on(String, into))]
pub struct ListCreators<'c> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'c CivitAI>,

	pub limit: Option<u32>,
	pub page: Option<u32>,
	pub query: Option<String>,
}

impl_builder_send!(list_creators_builder, ListCreatorsBuilder, ListCreators<'c>);

impl<'c> Method<'c> for ListCreators<'c> {
	type Input = Self;
	type Output = Page<'c, Creator, Self>;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/creators";
	
	paginated_post_req!();
}

impl<'c> Paginate for ListCreators<'c> {
	fn pagination(&mut self) -> Option<PaginationView<'_>> {
		Some(PaginationView {
			limit: Some(&mut self.limit),
			page: Some(&mut self.page),
			cursor: None,
		})
	}
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
		mock_client!("GET", "/api/v1/creators?limit=10&query=test", list_creators);

		CivitAI::new_auth(TOKEN)?.list_creators()
			.limit(10)
			.query("test")
			.send().await?;

		Ok(())
	}
}