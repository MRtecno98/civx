use bon::Builder;
use serde::Serialize;

use crate::{CivitAI, Method, Query, queries::impl_builder_send};

#[derive(Serialize, Builder)]
pub struct ListTags<'a> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'a CivitAI>,
	
	pub limit: Option<u32>,
	pub page: Option<u32>,
	pub query: Option<String>,
}

impl_builder_send!(list_tags_builder, ListTagsBuilder, ListTags<'a>);

impl Method for ListTags<'_> {
	type Input = Self;
	type Output = serde_json::Value;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/tags";
}
