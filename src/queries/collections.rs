use bon::Builder;
use serde::Serialize;

use crate::{CivitAI, Method, Path, Query, enums::SortKind, models::{Collection, Paginated}, queries::{Pagination, impl_builder_send}};

#[derive(Serialize, Builder)]
pub struct ListCollections<'a> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'a CivitAI>,

	#[serde(flatten)]
	#[builder(with = 
		|limit: Option<u32>, page: Option<u32>, cursor: Option<String>| 
			Pagination { limit, page, cursor })]
	pub pagination: Option<Pagination>,

	pub query: Option<String>,
	pub sort: Option<SortKind>,
	pub nsfw: Option<bool>,
}

impl_builder_send!(list_collections_builder, ListCollectionsBuilder, ListCollections<'a>);

impl Method for ListCollections<'_> {
	type Input = Self;
	type Output = Paginated<Collection>;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/collections";
}

pub struct GetCollection;

impl Method for GetCollection {
	type Input = u32;
	type Output = Collection;

	type Type = Path;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/collections/{id}";
}
