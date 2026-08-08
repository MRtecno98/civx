use bon::Builder;
use serde::Serialize;

use crate::{CivitAI, Method, Query, models::{Creator, Paginated}, queries::impl_builder_send};

#[derive(Serialize, Builder)]
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
