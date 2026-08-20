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

	pub async fn next(self) -> Result<Option<Self>> {
		let Some(metadata) = self.metadata.clone() else {
			return Ok(None);
		};

		let Some(next) = metadata.next() else {
			return Ok(None);
		};

		self.seek_page(next).await
	}

	pub async fn next_page(self) -> Result<Option<Self>> {
		let Some(metadata) = self.metadata.clone() else {
			return Ok(None);
		};

		let Some(next) = metadata.next_page() else {
			return Ok(None);
		};

		self.seek_page(next).await
	}

	pub fn stream(self) -> impl TryStream<Ok = T, Error = Error, Item = Result<T>> {
		try_stream! {
			let mut current_page = Some(self);

			while let Some(mut page) = current_page.take() {
				for item in page.items.drain(..) {
					yield item;
				}

				current_page = page.next().await?;
			}
		}
	}

	pub fn items(&mut self) -> std::vec::Drain<'_, T> {
		self.items.drain(..)
	}

	pub fn page_count(&self) -> Option<u32> {
		self.metadata.as_ref().and_then(|m| m.total_pages)
			.filter(|n| !((*n == 0) ^ self.items.is_empty()))
	}

	pub fn total_items(&self) -> Option<u64> {
		self.metadata.as_ref().and_then(|m| m.total_items)
			.filter(|n| !((*n == 0) ^ self.items.is_empty()))
	}

	pub fn current_page(&self) -> Option<u32> {
		self.metadata.as_ref().and_then(|m| m.current_page)
	}

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
		let next_page_url = self.next_page.as_ref().map(NextPage::Url);
		let next_page = self.current_page.map(|p| p + 1).map(NextPage::Page);

		next_cursor.or(next_page_url).or(next_page)
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
				fixture!(pagination_url).replace("%BASE%", &format!("{}{}", &server.uri(), DummyMethod::ENDPOINT)), 
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
