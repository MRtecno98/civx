use std::collections::HashMap;

use bon::Builder;
use serde::Serialize;

use crate::{CivitAI, Method, Query, queries::{impl_builder_send, serialize_comma_separated}};

#[derive(Serialize, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct CheckPermissions<'c> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'c CivitAI>,

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

impl_builder_send!(check_permissions_builder, CheckPermissionsBuilder, CheckPermissions<'c>);

impl<'c> Method<'c> for CheckPermissions<'c> {
	type Input = Self;
	type Output = HashMap<String, bool>;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/permissions/check";
}

#[cfg(test)]
mod tests {
	use crate::tests::*;

	#[tokio::test]
	#[cfg(feature = "network-tests")]
	async fn online_check_permissions() -> Result<(), Box<dyn Error>> {
		CivitAI::new()?.check_permissions()
			.entity_ids(vec![2731187])
			.user_id(1234)
			.send().await?;

		Ok(())
	}

	#[tokio::test]
	async fn mock_check_permissions() -> Result<(), Box<dyn Error>> {
		mock_client!("GET", "/api/v1/permissions/check?entityIds=2731187&userId=1234", check_permissions);

		CivitAI::new_auth(TOKEN)?.check_permissions()
			.entity_ids(vec![2731187])
			.user_id(1234)
			.send().await?;

		Ok(())
	}
}
