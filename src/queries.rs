//! This module contains all the requests that can be made to the Site API.
//! Each request has a corresponding input and output struct, and for complex inputs 
//! a builder is provided to make it easier to construct the request.
//! 
//! # Examples
//! 
//! All queries can be called through dedicated methods on the `CivitAI` client, 
//! but it's also possible to choose a request type programmatically.
//! 
//! ```rust, no_run
//! # tokio_test::block_on(async {
//! # use civx::models::CurrentUser;
//! # let client = civx::CivitAI::new()?;
//! let me: CurrentUser = client.get_me().await?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```
//! 
//! ```rust, no_run
//! # tokio_test::block_on(async {
//! # use civx::{queries::GetMe, models::CurrentUser};
//! # let client = civx::CivitAI::new()?;
//! 
//! // Can be used with any generic T: Method
//! // One disadvantage is that no-argument methods still require an
//! // empty unit type as a placeholder
//! let me: CurrentUser = client.request::<GetMe>(()).await?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```
//! 

use bon::Builder;
use civx_derive::civx;
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

macro_rules! impl_pagination {
	(field) => {
		#[serde(flatten)]
		#[builder(with = |limit: Option<u32>, page: Option<u32>, cursor: Option<String>| 
			Pagination { limit, page, cursor })]
		pub pagination: Option<Pagination>,
	};

	($($name:tt)*) => {
		impl $crate::queries::Paginate for $($name)* {
			fn pagination(&mut self) -> Option<$crate::queries::PaginationView<'_>> {
				Some(self.pagination.get_or_insert_default().into())
			}
		}
	}
}

macro_rules! paginated_post_req {
	($request:pat, $output:pat, $client:pat, $after:block) => {
		fn post_request(request: Option<Self::Input>, output: &mut Self::Output, client: &'c crate::CivitAI) {
			output.request = request;
			output.client = Some(client);

			{
				let $client = client;
				let $request = request;
				let $output = output;

				$after;
			}
		}
	};

	() => {
		paginated_post_req!(_, _, _, {});
	}
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

#[civx(clap)]
#[derive(Serialize, Debug, Builder, Clone, Default)]
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
