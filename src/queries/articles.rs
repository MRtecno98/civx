use bon::Builder;
use serde::Serialize;

use crate::{CivitAI, Method, Path, Query, enums::SortKind, queries::{Pagination, impl_builder_send, serialize_comma_separated}};

/// An article is a long-form post published on Civitai — a guide, workflow write-up, 
/// changelog, or announcement. These endpoints expose the same public article feed 
/// that powers the website.
#[derive(Serialize, Builder)]
pub struct ListArticles<'a> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'a CivitAI>,

	#[serde(flatten)]
	#[builder(with = 
		|limit: Option<u32>, page: Option<u32>, cursor: Option<String>| 
			Pagination { limit, page, cursor })]
	pub pagination: Option<Pagination>,

	#[serde(serialize_with = "serialize_comma_separated", 
			skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<u32>>,

	pub username: Option<String>,
	pub sort: Option<SortKind>,
	pub nsfw: Option<bool>,
}

impl_builder_send!(list_articles_builder, ListArticlesBuilder, ListArticles<'a>);

impl Method for ListArticles<'_> {
	type Input = Self;
	type Output = serde_json::Value;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/articles";
}

/// Returns the full article object — the same fields as a list item plus the article 
/// body/content — with the moderator-only moderatorNsfwLevel field stripped and the 
/// coverImage clamped to the region's public browsing ceiling (a cover above the 
/// ceiling is dropped). Returns 404 if the article doesn't exist or is a draft / 
/// unpublished / private article (the two cases are indistinguishable):
pub struct GetArticle;

impl Method for GetArticle {
	type Input = u32;
	type Output = serde_json::Value;

	type Type = Path;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/articles/{id}";
}
