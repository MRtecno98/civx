use std::ops::Deref;

use serde::Deserialize;
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

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Paginated<T> {
	items: Vec<T>,

	pub metadata: Option<Metadata>,
}

impl<T> Deref for Paginated<T> {
	type Target = Vec<T>;

	fn deref(&self) -> &Self::Target {
		&self.items
	}
}

impl<T> IntoIterator for Paginated<T> {
	type Item = T;
	type IntoIter = std::vec::IntoIter<T>;

	fn into_iter(self) -> Self::IntoIter {
		self.items.into_iter()
	}
}

impl<'a, T> IntoIterator for &'a Paginated<T> {
	type Item = &'a T;
	type IntoIter = std::slice::Iter<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		self.items.iter()
	}
}

impl<'a, T> IntoIterator for &'a mut Paginated<T> {
	type Item = &'a mut T;
	type IntoIter = std::slice::IterMut<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		self.items.iter_mut()
	}
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Metadata {
	pub next_cursor: Option<String>,
	pub next_page: Option<Url>,
	pub current_page: Option<u32>,
	pub page_size: Option<u32>,
	pub total_items: Option<u64>,
	pub total_pages: Option<u32>,
}
