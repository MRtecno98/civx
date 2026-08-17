use bon::Builder;
use serde::Serialize;

use crate::{CivitAI, Method, Path, Query, enums::ArticleSortKind, models::{Article, ArticleInfo, Page}, queries::{Pagination, impl_builder_send, impl_pagination, paginated_post_req, serialize_comma_separated}};

/// An article is a long-form post published on Civitai — a guide, workflow write-up, 
/// changelog, or announcement. These endpoints expose the same public article feed 
/// that powers the website.
#[derive(Serialize, Builder)]
#[builder(on(String, into))]
pub struct ListArticles<'c> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'c CivitAI>,

	#[serde(flatten)]
	#[builder(with = 
		|limit: Option<u32>, page: Option<u32>, cursor: Option<String>| 
			Pagination { limit, page, cursor })]
	pub pagination: Option<Pagination>,

	#[serde(serialize_with = "serialize_comma_separated", 
			skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<u32>>,

	pub username: Option<String>,
	pub sort: Option<ArticleSortKind>,
	pub nsfw: Option<bool>,
}

impl_builder_send!(list_articles_builder, ListArticlesBuilder, ListArticles<'c>);
impl_pagination!(ListArticles<'_>);

impl<'c> Method<'c> for ListArticles<'c> {
	type Input = Self;
	type Output = Page<'c, ArticleInfo, Self>;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/articles";

	paginated_post_req!();
}

/// Returns the full article object — the same fields as a list item plus the article 
/// body/content — with the moderator-only moderatorNsfwLevel field stripped and the 
/// coverImage clamped to the region's public browsing ceiling (a cover above the 
/// ceiling is dropped). Returns 404 if the article doesn't exist or is a draft / 
/// unpublished / private article (the two cases are indistinguishable):
pub struct GetArticle;

impl<'c> Method<'c> for GetArticle {
	type Input = i64;
	type Output = Article; // TODO: Docs don't mention even half the fields

	type Type = Path;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/articles/{}";
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests::*;

	#[tokio::test]
	#[cfg(feature = "network-tests")]
	async fn online_list_articles() -> Result<(), Box<dyn Error>> {
		CivitAI::new()?.list_articles()
			.nsfw(true)
			.sort(ArticleSortKind::MostBookmarks)
			.send().await?;

		Ok(())
	}

	#[tokio::test]
	#[cfg(feature = "network-tests")]
	async fn online_get_article() -> Result<(), Box<dyn Error>> {
		CivitAI::new()?.get_article(33738).await?;

		Ok(())
	}

	#[tokio::test]
	async fn mock_list_articles() -> Result<(), Box<dyn Error>> {
		mock_client!("GET", "/api/v1/articles", list_articles, {
			CivitAI::new_auth(TOKEN)?.list_articles()
				.nsfw(true)
				.sort(ArticleSortKind::MostBookmarks)
				.send().await?;
		})
	}

	#[tokio::test]
	async fn mock_get_article() -> Result<(), Box<dyn Error>> {
		mock_client!("GET", "/api/v1/articles/12345", get_article, {
			CivitAI::new_auth(TOKEN)?.get_article(12345).await?;
		})
	}
}
