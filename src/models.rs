//! Data structures modeling the returned types of the CivitAI API.
//! These are used as output types for all the requests in the [`queries`](crate::queries) module.
//! 
//! For more detail on specific fields check out the [official documentation](https://developer.civitai.com/site/reference/).
//! 
//! ### Pagination
//! For queries that support pagination a [`Page`] struct is returned, which 
//! contains the items for the current page, as well as metadata about the pagination state. 
//! 
//! The [`Page`] struct also provides methods to retrieve the next page of results, 
//! either by cursor or by page number, and a stream of all items across pages.
//! 
//! Both cursor and page-based pagination are supported, you should refer to the official doc to
//! determine which is more appropriate for your use case.

use std::{hint::unreachable_unchecked, ops::Deref};

use async_stream::try_stream;
use futures::TryStream;
use serde::{Deserialize, Deserializer};
use url::Url;

mod model;
mod files;
mod image;
mod article;
mod collection;
mod creator;
mod tags;
mod user;
mod enums;

pub use model::*;
pub use files::*;
pub use image::*;
pub use article::*;
pub use collection::*;
pub use creator::*;
pub use tags::*;
pub use user::*;
pub use enums::*;

use crate::{Method, Result, error::Error, queries::Paginate};

/// A page of results returned by a paginated query,
/// plus metadata about the pagination state.
/// 
/// ### Pagination strategies
/// CivX supports both cursor and page navigation, follow 
/// CivitAI's recommendations to know which to choose.
/// 
/// In a nutshell cursors remain consistent across catalogue changes and 
/// are best used for automatic iteration of a big section of the catalogue, 
/// while pages support seeking and are better suited for user interfaces.
/// 
/// Note that when using page navigation the requested page number times the 
/// request element limit *must be less than 1000*. Consider using cursors if 
/// you require deeper iteration.
/// 
/// # Examples
/// ```rust, no_run
/// # tokio_test::block_on(async {
/// # use std::pin::pin;
/// # let client = civx::CivitAI::new()?;
/// let models = client.list_models().send().await?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// # });
/// ```
/// 
/// Using cursors (and streams)
/// ```rust, no_run
/// # tokio_test::block_on(async {
/// # use std::pin::pin;
/// # use civx::{models::{Model, Page}, queries::ListModels};
/// # use futures::TryStreamExt;
/// # let models: Page<'_, Model, ListModels<'_>> = unsafe { std::mem::zeroed() };
/// // Using cursors (and streams)
/// let mut stream = pin!(models.stream());
///
/// while let Some(model) = stream.try_next().await? {
///     // automatically requests more cursors
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// # });
/// ```
/// 
/// Using pages, note that limit*page must be less than 1000.
/// ```rust, no_run
/// # tokio_test::block_on(async {
/// # use std::pin::pin;
/// # use civx::{models::{Model, Page}, queries::ListModels};
/// # use futures::TryStreamExt;
/// # let mut page: Page<'_, Model, ListModels<'_>> = unsafe { std::mem::zeroed() };
/// let (current_page, page_count) = page.index()
///     .expect("Request doesn't support page iteration");
/// 
/// // This drains the items from the page, and returns
/// // owned values for the contents.
/// // Iterating over a &mut is also equivalent
/// for element in page.items() {
///     // process the contents of the current page
/// }
///
/// // When you need to, you can request a new page.
/// // This consumes the old one so be sure to drain
/// // the items by iterating it first!
/// let new_page = page.seek_page(current_page + 1).await?
///     .expect("No more pages left"); 
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// # });
/// ```
/// 
#[derive(Deserialize, Debug, Clone)]
pub struct Page<'c, T, M: Method<'c, Output = Self>> where M::Input: Paginate {
	items: Vec<T>,

	#[serde(skip)]
	pub(crate) request: Option<M::Input>,
	#[serde(skip)]
	pub(crate) client: Option<&'c crate::CivitAI>,

	pub metadata: Option<Metadata>,
}

impl<'c, T, M: Method<'c, Output = Self>> Page<'c, T, M> where M::Input: Paginate {
	/// Seeks a specific page of results, either by cursor, page number, or a direct URL.
	/// 
	/// Note that cursors can only access the next page of results, 
	/// while page numbers can seek to any page. Because of this, if you need
	/// a seekable interface (such as a page slider) you should use pagewise iteration.
	/// 
	/// This method consumes the current page, so be sure to drain the items by iterating it first.
	/// (see [`Page::items`])
	/// 
	/// If you are using page numbers notice that [`NextPage`] implements [`From<u32>`] 
	/// so you can pass a page number directly.
	/// 
	/// Also if you're doing cursor iteration or just want the next page instead of a specific one
	/// you may be more interested in using [`Page::next`] or [`Page::next_page`] instead, 
	/// which will automatically fetch the next cursor from the page metadata.
	/// 
	/// # Examples
	/// ```rust, no_run
	/// # tokio_test::block_on(async {
	/// # use std::pin::pin;
	/// # use civx::{models::{Model, Page}, queries::ListModels};
	/// # let page: Page<'_, Model, ListModels<'_>> = unsafe { std::mem::zeroed() };
	/// let new_page = page.seek_page(2).await?; 
	/// # Ok::<(), Box<dyn std::error::Error>>(())
	/// # });
	/// ```
	pub async fn seek_page(self, page: impl Into<NextPage<'_>>) -> Result<Option<Self>> {
		let next = page.into();

		let Some(client) = self.client else {
			return Err(Error::ClientNotSet);
		};

		let result = match next {
			NextPage::Url(url) =>
				client.request_url::<'c, M>(url, None).await?,

			_ => {
				let Some(mut request) = self.request else {
					return Err(Error::RequestNotSet);
				};

				let Some(mut pagination) = request.pagination() else {
					return Ok(None);
				};

				match next {
					NextPage::Cursor(cursor) => 
						pagination.replace_cursor(Some(cursor.to_owned()))
							.replace_page(None),

					NextPage::Page(page) => 
						pagination.replace_page(Some(page))
							.replace_cursor(None),

					NextPage::Url(_) => unsafe { unreachable_unchecked() }
				};
				
				client.request::<'c, M>(request).await?
			},
		};

		Ok(Some(result))
	}

	/// Automatically fetches the next page of results, prioritizing
	/// cursor-based pagination if available, and falling back to page-based pagination otherwise.
	/// 
	/// See also [`Metadata::next`] if you just want to know what the next page is without actually fetching it,
	/// and for more reasoning about the prioritization.
	/// 
	/// If you want to restrict iteration to page-based pagination, use [`Page::next_page`] instead.
	/// 
	/// # Examples
	/// ```rust, no_run
	/// # tokio_test::block_on(async {
	/// # use std::pin::pin;
	/// # use civx::{models::{Model, Page}, queries::ListModels};
	/// # let page: Page<'_, Model, ListModels<'_>> = unsafe { std::mem::zeroed() };
	/// let next_page = page.next().await?;
	/// # Ok::<(), Box<dyn std::error::Error>>(())
	/// # });
	pub async fn next(self) -> Result<Option<Self>> {
		let Some(metadata) = self.metadata.clone() else {
			return Ok(None);
		};

		let Some(next) = metadata.next() else {
			return Ok(None);
		};

		self.seek_page(next).await
	}

	/// Automatically fetches the next page of results, forcing page-based iteration.
	/// 
	/// An `Ok(None)` result indicates that no more pages left to fetch.
	/// If the request doesn't support page-based iteration, it will appear as if there 
	/// were no more pages.
	/// 
	/// As with all page-based iteration, the request limit*page must be less than 1000, otherwise the request will fail.
	/// If you need to iterate deeper than that, consider using cursor-based iteration instead (see [`Page::next`]).
	pub async fn next_page(self) -> Result<Option<Self>> {
		let Some(metadata) = self.metadata.clone() else {
			return Ok(None);
		};

		let Some(next) = metadata.next_page() else {
			return Ok(None);
		};

		self.seek_page(next).await
	}

	/// Similar to [`Page::stream`], but limits the number of items returned to `n`.
	/// 
	/// # Examples
	/// ```rust, no_run
	/// # tokio_test::block_on(async {
	/// # use std::pin::pin;
	/// # use civx::{models::{Model, Page}, queries::ListModels};
	/// # use futures::TryStreamExt;
	/// # let page: Page<'_, Model, ListModels<'_>> = unsafe { std::mem::zeroed() };
	/// let mut stream = pin!(page.stream_n(15));
	/// assert_eq!(stream.try_collect::<Vec<_>>().await?.len(), 15);
	/// # Ok::<(), Box<dyn std::error::Error>>(())
	/// # });
	pub fn stream_n(self, n: usize) -> impl TryStream<Ok = T, Error = Error, Item = Result<T>> {
		try_stream! {
			let mut current_page = Some(self);
			let mut count = 0;

			while let Some(mut page) = current_page.take() {
				for item in page.items() {
					yield item;

					count += 1;
					if count >= n {
						return;
					}
				}

				current_page = page.next().await?;
			}
		}
	}

	/// Returns a stream of all items across pages, using [`Page::next`] under the hood to 
	/// automatically fetch the next page of results.
	/// 
	/// A version of this method using [`Page::next_page`] is omitted by design,
	/// as any use case that doesn't care about the page structure should prefer cursor-based iteration, 
	/// which is more robust to catalogue changes.
	/// 
	/// # Examples
	/// ```rust, no_run
	/// # tokio_test::block_on(async {
	/// # use std::pin::pin;
	/// # use civx::{models::{Model, Page}, queries::ListModels};
	/// # use futures::TryStreamExt;
	/// # let page: Page<'_, Model, ListModels<'_>> = unsafe { std::mem::zeroed() };
	/// let mut stream = pin!(page.stream());
	/// 
	/// while let Some(model) = stream.try_next().await? {
	///     // will automatically request more cursors
	/// }
	/// # Ok::<(), Box<dyn std::error::Error>>(())
	/// # });
	pub fn stream(self) -> impl TryStream<Ok = T, Error = Error, Item = Result<T>> {
		self.stream_n(usize::MAX)
	}

	/// Returns a draining iterator over the items in the current page.
	/// Use this method to consume the items in the current page before fetching the next one.
	pub fn items(&mut self) -> std::vec::Drain<'_, T> {
		self.items.drain(..)
	}

	/// Returns the number of pages and the current page number, if available.
	/// 
	/// As per the API documentation, this information may not be provided by the server if
	/// it's not cheap to compute.
	pub fn page_count(&self) -> Option<u32> {
		self.metadata.as_ref().and_then(|m| m.total_pages)
			.filter(|n| !((*n == 0) ^ self.items.is_empty()))
	}

	/// Returns the total number of items across all pages, if available.
	/// 
	/// As per the API documentation, this information may not be provided by the server if
	/// it's not cheap to compute.
	pub fn total_items(&self) -> Option<u64> {
		self.metadata.as_ref().and_then(|m| m.total_items)
			.filter(|n| !((*n == 0) ^ self.items.is_empty()))
	}

	/// Returns the current page number, if available.
	/// 
	/// As per the API documentation, this information may not be provided by the server if
	/// it's not cheap to compute.
	pub fn current_page(&self) -> Option<u32> {
		self.metadata.as_ref().and_then(|m| m.current_page)
	}

	/// Returns the current page number and the total number of pages, only if both are available.
	/// 
	/// This is mostly a convenience method for use in user interfaces, to avoid two checks
	/// where both values are usually needed together.
	pub fn index(&self) -> Option<(u32, u32)> {
		match (self.current_page(), self.page_count()) {
			(Some(current), Some(total)) => Some((current, total)),
			_ => None,
		}
	}
}

impl<'c, T, M: Method<'c, Output = Self>> Deref for Page<'c, T, M> where M::Input: Paginate {
	type Target = Vec<T>;

	fn deref(&self) -> &Self::Target {
		&self.items
	}
}

impl<'c, T, M: Method<'c, Output = Self>> IntoIterator for Page<'c, T, M> where M::Input: Paginate {
	type Item = T;
	type IntoIter = std::vec::IntoIter<T>;

	fn into_iter(self) -> Self::IntoIter {
		self.items.into_iter()
	}
}

impl<'a, 'c, T, M: Method<'c, Output = Page<'c, T, M>>> IntoIterator for &'a Page<'c, T, M> where M::Input: Paginate {
	type Item = &'a T;
	type IntoIter = std::slice::Iter<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		self.items.iter()
	}
}

impl<'a, 'c, T, M: Method<'c, Output = Page<'c, T, M>>> IntoIterator for &'a mut Page<'c, T, M> where M::Input: Paginate {
	type Item = T;
	type IntoIter = std::vec::Drain<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		self.items()
	}
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
	#[serde(default, deserialize_with = "opt_str_or_num_to_str")]
	pub next_cursor: Option<String>,
	pub next_page: Option<Url>,
	pub current_page: Option<u32>,
	pub page_size: Option<u32>,
	pub total_items: Option<u64>,
	pub total_pages: Option<u32>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NextPage<'a> {
	Cursor(&'a str),
	Url(&'a Url),
	Page(u32)
}

impl From<u32> for NextPage<'_> {
	fn from(page: u32) -> Self {
		Self::Page(page)
	}
}

impl Metadata {
	pub fn has_next_page(&self) -> bool {
		self.next_cursor.is_some() || self.next_page.is_some()
	}

	pub fn next(&self) -> Option<NextPage<'_>> {
		let next_cursor = self.next_cursor.as_ref().map(|page| NextPage::Cursor(page));

		next_cursor.or(self.next_page())
	}

	pub fn next_page(&self) -> Option<NextPage<'_>> {
		let next_page_url = self.next_page.as_ref().map(NextPage::Url);
		let next_page = self.current_page.map(|p| p + 1).map(NextPage::Page);

		next_page_url.or(next_page)
	}
}

fn opt_str_or_num_to_str<'de, D>(deserializer: D) -> std::result::Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNum {
        Str(String),
        Num(i64),
    }

    let opt = Option::<StringOrNum>::deserialize(deserializer)?;
    
    Ok(opt.map(|val| match val {
        StringOrNum::Str(s) => s,
        StringOrNum::Num(n) => n.to_string(),
    }))
}

#[cfg(test)]
#[allow(unused)]
mod tests {
	use bon::Builder;
	use serde::Serialize;

	use super::*;
	use crate::{NoArgs, Query, queries::{Pagination, impl_pagination, paginated_post_req}, tests::*};

	#[derive(Serialize, Debug, Default)]
	pub struct DummyMethod {
		#[serde(flatten)]
		pagination: Option<Pagination>,
	}

	impl<'c> Method<'c> for DummyMethod {
		type Input = Self;
		type Output = Page<'c, String, Self>;

		type Type = Query;

		const ENDPOINT: &'static str = "/test-path";
		const METHOD: reqwest::Method = reqwest::Method::GET;

		paginated_post_req!(_, output, _, {
			assert_eq!(output.items, vec!["test-data"]);
		});
	}

	impl_pagination!(DummyMethod);

	macro_rules! with_dummy_mock {
		() => {{
			Mock::given(method("GET"))
				.and(path(DummyMethod::ENDPOINT))
		}};
	}

	macro_rules! initiate_request {
		(@$fixture:ident, $client:ident, $server:ident) => {
			initiate_request!(fixture_response!($fixture), $client, $server)
		};

		($response:expr, $client:ident, $server:ident) => {{
			let _guard = with_dummy_mock!()
				.and(query_param_is_missing("cursor"))
				.and(query_param_is_missing("page"))
				.respond_with($response)
				.mount_as_scoped(&$server).await;

			$client.request::<DummyMethod>(DummyMethod::default()).await?
		}};
	}

	#[tokio::test]
	async fn pagination_cursor() -> Result<()> {
		let server = mock_client!();
		let client = CivitAI::new()?;

		let page = initiate_request!(@pagination_cursor, client, server);

		{
			let _guard = with_dummy_mock!()
				.and(query_param("cursor", "123|456|789"))
				.respond_with(fixture_response!(pagination_cursor))
				.mount_as_scoped(&server).await;

			page.next().await?.unwrap();
		}
		
		Ok(())
	}

	#[tokio::test]
	async fn pagination_cursor_num() -> Result<()> {
		let server = mock_client!();
		let client = CivitAI::new()?;

		let page = initiate_request!(@pagination_cursor_num, client, server);

		{
			let _guard = with_dummy_mock!()
				.and(query_param("cursor", "123456"))
				.respond_with(fixture_response!(pagination_cursor_num))
				.mount_as_scoped(&server).await;

			page.next().await?.unwrap();
		}
		
		Ok(())
	}

	#[tokio::test]
	async fn pagination_url() -> Result<()> {
		let server = mock_client!();
		let client = CivitAI::new()?;

		let response = ResponseTemplate::new(200)
			.set_body_raw(
				fixture!(pagination_url).replace("%BASE%", &format!("{}{}", server.uri(), DummyMethod::ENDPOINT)), 
				"application/json");

		let page = initiate_request!(response.clone(), client, server);

		{
			let _guard = with_dummy_mock!()
				.and(query_param("page", "2"))
				.respond_with(response)
				.mount_as_scoped(&server).await;

			page.next().await?.unwrap();
		}

		Ok(())
	}

	#[tokio::test]
	async fn pagination_multiple() -> Result<()> {
		let server = mock_client!();
		let client = CivitAI::new()?;

		let page = initiate_request!(@pagination_multiple, client, server);

		assert_eq!(page.metadata.unwrap().next().unwrap(), NextPage::Cursor("123|456|789"));
		
		Ok(())
	}
}
