use bon::Builder;
use serde::Serialize;

use crate::{CivitAI, Method, NoArgs, Query, models::{CurrentUser, Page, UserLookup}, queries::{Paginate, PaginationView, impl_builder_send, paginated_post_req, serialize_comma_separated}};

pub struct GetMe;

impl<'c> Method<'c> for GetMe {
	type Input = ();
	type Output = CurrentUser;

	type Type = NoArgs;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/me";
}

#[derive(Serialize, Builder)]
#[builder(on(String, into))]
pub struct LookupUsers<'c> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'c CivitAI>,

	#[serde(serialize_with = "serialize_comma_separated", 
			skip_serializing_if = "Option::is_none")]
	pub ids: Option<Vec<u32>>,

	pub query: Option<String>,
}

impl_builder_send!(lookup_users_builder, LookupUsersBuilder, LookupUsers<'c>);

impl Paginate for LookupUsers<'_> {
    fn pagination(&mut self) -> Option<PaginationView<'_>> {
		None
	}
}

impl<'c> Method<'c> for LookupUsers<'c> {
	type Input = Self;
	type Output = Page<'c, UserLookup, Self>;

	type Type = Query;
	
	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/users";
	
	paginated_post_req!();
}

#[cfg(test)]
mod tests {
	use crate::tests::*;

	#[tokio::test]
	#[cfg(feature = "network-tests")]
	async fn online_lookup_users() -> Result<(), Box<dyn Error>> {
		CivitAI::new()?.lookup_users()
			.ids(vec![123, 456, 789])
			.send().await?;

		Ok(())
	}

	#[tokio::test]
	#[cfg(feature = "network-tests")]
	#[ignore = "requires a token file in the project root"]
	async fn online_get_me() -> Result<(), Box<dyn Error>> {
		CivitAI::new_auth(auth_token!())?.get_me().await?;

		Ok(())
	}

	#[tokio::test]
	async fn mock_lookup_users() -> Result<(), Box<dyn Error>> {
		mock_client!("GET", "/api/v1/users?ids=123,456,789", lookup_users, {
			CivitAI::new_auth(TOKEN)?.lookup_users()
				.ids(vec![123, 456, 789])
				.send().await?;
		})
	}

	#[tokio::test]
	async fn mock_get_me() -> Result<(), Box<dyn Error>> {
		mock_client!("GET", "/api/v1/me", get_me, {
			CivitAI::new_auth(TOKEN)?.get_me().await?;
		})
	}
}
