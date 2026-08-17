use bon::Builder;
use serde::{Serialize, Serializer};

mod articles;
mod collections;
mod creators;
mod enums;
mod images;
mod models;
mod permissions;
mod tags;
mod users;
mod vault;

pub use articles::*;
pub use collections::*;
pub use creators::*;
pub use enums::*;
pub use images::*;
pub use models::*;
pub use permissions::*;
pub use tags::*;
pub use users::*;
pub use vault::*;

macro_rules! impl_builder_send {
	($module:ident, $builder:tt, $method:ty) => {
		impl<'c, S: $module::State> $builder<'c, S> {
			pub(crate) fn client(self, client: &'c CivitAI) -> $builder<'c, S> {
				$builder { _client: Some(client), ..self }
			}

			pub async fn send(self) -> $crate::Result<<$method as $crate::Method<'c>>::Output> 
			where S: $module::IsComplete {
				let client = self._client.ok_or($crate::error::Error::ClientNotSet)?;
				client.request::<$method>(self.build()).await
			}
		}
	};
}

macro_rules !impl_pagination {
	(field) => {
		#[serde(flatten)]
		#[builder(with = |limit: Option<u32>, page: Option<u32>, cursor: Option<String>| 
			Pagination { limit, page, cursor })]
		pub pagination: Option<Pagination>,
	};

	($($name:tt)*) => {
		impl $crate::queries::Paginate for $($name)* {
			fn pagination<'a>(&'a mut self) -> Option<$crate::queries::PaginationView<'a>> {
				Some(self.pagination.get_or_insert_default().into())
			}
		}
	}
}

macro_rules! paginated_post_req {
	() => {
		fn post_request(request: Option<Self::Input>, output: &mut Self::Output, client: &'c crate::CivitAI) {
			output.request = request;
			output.client = Some(client);
		}
	};
}

pub(crate) use impl_builder_send;
pub(crate) use impl_pagination;
pub(crate) use paginated_post_req;

fn serialize_comma_separated<S: Serializer, I: ToString>(vec: &Option<Vec<I>>, serializer: S) -> Result<S::Ok, S::Error> {
	match vec {
		Some(v) => serializer.serialize_str(
			&v.iter().map(I::to_string).collect::<Vec<_>>().join(",")),
		None => serializer.serialize_none()
	}
}

#[derive(Serialize, Builder, Default)]
pub struct Pagination {
	pub limit: Option<u32>,
	pub page: Option<u32>,
	pub cursor: Option<String>,
}

#[derive(Debug)]
pub struct PaginationView<'a> {
	pub limit: Option<&'a mut Option<u32>>,
	pub page: Option<&'a mut Option<u32>>,
	pub cursor: Option<&'a mut Option<String>>,
}

pub trait Paginate {
	fn pagination(&mut self) -> Option<PaginationView<'_>>;
}

impl<'a> From<&'a mut Pagination> for PaginationView<'a> {
	fn from(pagination: &'a mut Pagination) -> Self {
		PaginationView {
			limit: Some(&mut pagination.limit),
			page: Some(&mut pagination.page),
			cursor: Some(&mut pagination.cursor),
		}
	}
}

impl PaginationView<'_> {
	pub fn replace_cursor(&mut self, new_cursor: Option<String>) -> &mut Self {
		if let Some(cursor) = &mut self.cursor {
			**cursor = new_cursor;
		}
		
		self
	}

	pub fn replace_limit(&mut self, new_limit: Option<u32>) -> &mut Self {
		if let Some(limit) = &mut self.limit {
			**limit = new_limit;
		}

		self
	}

	pub fn replace_page(&mut self, new_page: Option<u32>) -> &mut Self {
		if let Some(page) = &mut self.page {
			**page = new_page;
		}

		self
	}
}
