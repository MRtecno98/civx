use std::ops::Deref;

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

use crate::{Method, Result, error::Error, queries::{Paginate, PaginationView}};

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
	pub async fn next(self) -> Result<Option<Self>> {
		let Some(metadata) = self.metadata else {
			return Ok(None);
		};

		let Some(next) = metadata.next() else {
			return Ok(None);
		};

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
						pagination.replace_cursor(Some(cursor.to_owned())),

					NextPage::Page(page) => 
						pagination.replace_page(Some(page)),

					NextPage::Url(_) => unreachable!() 
				};
				
				client.request::<'c, M>(request).await?
			},
		};

		Ok(Some(result))
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
	type Item = &'a mut T;
	type IntoIter = std::slice::IterMut<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		self.items.iter_mut()
	}
}

impl Paginate for () {
	fn pagination(&mut self) -> Option<PaginationView<'_>> {
		None
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

pub enum NextPage<'a> {
	Cursor(&'a str),
	Url(&'a Url),
	Page(u32)
}

impl Metadata {
	pub fn has_next_page(&self) -> bool {
		self.next_cursor.is_some() || self.next_page.is_some()
	}

	pub fn next(&self) -> Option<NextPage<'_>> {
		let next_cursor = self.next_cursor.as_ref().map(|page| NextPage::Cursor(page));
		let next_page_url = self.next_page.as_ref().map(NextPage::Url);
		let next_page = self.current_page.clone().map(|p| p + 1).map(NextPage::Page);

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
