use bon::Builder;
use serde::Serialize;

use crate::{CivitAI, Method, Query, enums::{MediaType, NsfwLevel, Period, SortKind}, models::{Image, Paginated}, queries::{Pagination, impl_builder_send, serialize_comma_separated}};

#[derive(Serialize, Builder)]
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
	pub sort: Option<SortKind>,
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
