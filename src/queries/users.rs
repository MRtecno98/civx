use bon::Builder;
use serde::Serialize;

use crate::{CivitAI, Method, NoArgs, Query, models::{CurrentUser, Paginated, UserLookup}, queries::{impl_builder_send, serialize_comma_separated}};

pub struct GetMe;

impl Method for GetMe {
	type Input = ();
	type Output = CurrentUser;

	type Type = NoArgs;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/me";
}

#[derive(Serialize, Builder)]
#[builder(on(String, into))]
pub struct LookupUsers<'a> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'a CivitAI>,

	#[serde(serialize_with = "serialize_comma_separated", 
			skip_serializing_if = "Option::is_none")]
	pub ids: Option<Vec<u32>>,

	pub query: Option<String>,
}

impl_builder_send!(lookup_users_builder, LookupUsersBuilder, LookupUsers<'a>);

impl Method for LookupUsers<'_> {
	type Input = Self;
	type Output = Paginated<UserLookup>;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/users";
}
