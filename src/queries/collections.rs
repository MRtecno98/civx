use bon::Builder;
use serde::Serialize;

use crate::{CivitAI, Method, Path, Query, enums::CollectionSortKind, models::{Collection, Page}, queries::{Pagination, impl_builder_send, impl_pagination, paginated_post_req}};

#[derive(Serialize, Builder)]
#[builder(on(String, into))]
pub struct ListCollections<'c> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'c CivitAI>,

	#[serde(flatten)]
	#[builder(with = 
		|limit: Option<u32>, page: Option<u32>, cursor: Option<String>| 
			Pagination { limit, page, cursor })]
	pub pagination: Option<Pagination>,

	pub query: Option<String>,
	pub sort: Option<CollectionSortKind>,
	pub nsfw: Option<bool>,
}

impl_builder_send!(list_collections_builder, ListCollectionsBuilder, ListCollections<'c>);
impl_pagination!(ListCollections<'_>);

impl<'c> Method<'c> for ListCollections<'c> {
	type Input = Self;
	type Output = Page<'c, Collection, Self>;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/collections";

	paginated_post_req!();
}

pub struct GetCollection;

impl<'c> Method<'c> for GetCollection {
	type Input = u32;
	type Output = Collection;

	type Type = Path;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/collections/{}";
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests::*;

	#[tokio::test]
	#[cfg(feature = "network-tests")]
	async fn online_list_collections() -> Result<(), Box<dyn Error>> {
		CivitAI::new()?.list_collections()
			.sort(CollectionSortKind::MostFollowers)
			.pagination(Some(10), None, None)
			.send().await?;

		Ok(())
	}

	#[tokio::test]
	#[cfg(feature = "network-tests")]
	async fn online_get_collection() -> Result<(), Box<dyn Error>> {
		CivitAI::new()?.get_collection(10505430).await?;

		Ok(())
	}

	#[tokio::test]
	async fn mock_list_collections() -> Result<(), Box<dyn Error>> {
		mock_client!("GET", "/api/v1/collections", list_collections, {
			CivitAI::new_auth(TOKEN)?.list_collections()
				.sort(CollectionSortKind::MostFollowers)
				.pagination(Some(10), None, None)
				.send().await?;
		})
	}

	#[tokio::test]
	async fn mock_get_collection() -> Result<(), Box<dyn Error>> {
		mock_client!("GET", "/api/v1/collections/10505430", get_collection, {
			CivitAI::new_auth(TOKEN)?.get_collection(10505430).await?;
		})
	}
}
