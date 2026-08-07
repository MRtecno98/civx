use bon::Builder;
use serde::Serialize;

use crate::{CivitAI, Method, Query, queries::{impl_builder_send, serialize_comma_separated}};

#[derive(Serialize, Builder)]
pub struct CheckPermissions<'a> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'a CivitAI>,

	#[serde(serialize_with = "serialize_comma_separated", 
			skip_serializing_if = "Option::is_none")]
	pub entity_ids: Option<Vec<u32>>,

	// From the API docs:
	// "The kind of entity. Currently only model versions are supported."
	// pub entity_type: Option<EntityType>,

	// From the API docs:
	// "Which permission to check. Currently only Generate is supported."
	// pub permission: Option<Permission>,

	pub user_id: Option<u32>,
}

impl_builder_send!(check_permissions_builder, CheckPermissionsBuilder, CheckPermissions<'a>);

impl Method for CheckPermissions<'_> {
	type Input = Self;
	type Output = serde_json::Value;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/permissions/check";
}
