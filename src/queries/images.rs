use bon::Builder;
use serde::Serialize;

use crate::{CivitAI, Method, Query, enums::{ImageSortKind, MediaType, NsfwLevel, Period}, models::{Image, Paginated}, queries::{Pagination, impl_builder_send, serialize_comma_separated}};

#[derive(Serialize, Builder)]
#[builder(on(String, into))]
pub struct ListImages<'a> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'a CivitAI>,

	#[serde(flatten)]
	#[builder(with = 
		|limit: Option<u32>, page: Option<u32>, cursor: Option<String>| 
			Pagination { limit, page, cursor })]
	pub pagination: Option<Pagination>,
	
	pub post_id: Option<u32>,
	pub model_id: Option<u32>,
	pub model_version_id: Option<u32>,

	pub image_id: Option<u32>,

	pub username: Option<String>,
	pub user_id: Option<u32>,

	pub period: Option<Period>,
	pub sort: Option<ImageSortKind>,
	pub browsing_level: Option<NsfwLevel>,

    #[serde(serialize_with = "serialize_comma_separated", 
			skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<u32>>,

	pub media_type: Option<MediaType>,

    #[serde(serialize_with = "serialize_comma_separated", 
			skip_serializing_if = "Option::is_none")]
	pub base_models: Option<Vec<String>>,

	pub with_meta: Option<bool>,
}

impl_builder_send!(list_images_builder, ListImagesBuilder, ListImages<'a>);

impl Method for ListImages<'_> {
	type Input = Self;
	type Output = Paginated<Image>;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/images";
}

#[cfg(test)]
mod tests {
	use crate::tests::*;
	use crate::enums::ImageSortKind;

	#[tokio::test]
	#[cfg(feature = "network-tests")]
	async fn online_list_images() -> Result<(), Box<dyn Error>> {
		CivitAI::new()?.list_images()
			.pagination(Some(10), None, None)
			.sort(ImageSortKind::MostReactions)
			.send().await?;

		Ok(())
	}

	#[tokio::test]
	async fn mock_list_images() -> Result<(), Box<dyn Error>> {
		mock_client!("GET", "/api/v1/images?limit=10&sort=Most Reactions", list_images, {
			CivitAI::new_auth(TOKEN)?.list_images()
				.pagination(Some(10), None, None)
				.sort(ImageSortKind::MostReactions)
				.send().await?;
		})
	}
}
