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
		impl<'a, S: $module::State> $builder<'a, S> {
			pub(crate) fn client(self, client: &'a CivitAI) -> $builder<'a, S> {
				$builder { _client: Some(client), ..self }
			}

			pub async fn send(self) -> $crate::Result<<$method as $crate::Method>::Output> 
			where S: $module::IsComplete {
				let client = self._client.ok_or($crate::error::Error::ClientNotSet)?;
				client.request::<$method>(self.build()).await
			}
		}
	};
}

pub(crate) use impl_builder_send;

fn serialize_comma_separated<S: Serializer, I: ToString>(vec: &Option<Vec<I>>, serializer: S) -> Result<S::Ok, S::Error> {
	match vec {
		Some(v) => serializer.serialize_str(
			&v.iter().map(I::to_string).collect::<Vec<_>>().join(",")),
		None => serializer.serialize_none()
	}
}

#[derive(Serialize, Builder)]
pub struct Pagination {
	pub limit: Option<u32>,
	pub page: Option<u32>,
	pub cursor: Option<String>,
}
