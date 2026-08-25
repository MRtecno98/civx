use bon::Builder;
use chrono::{DateTime, Utc};
use civx_derive::civx;
use serde::Serialize;

use crate::{CivitAI, Method, NoArgs, Query, enums::{BaseModel, ModelType, SortKind}, queries::{impl_builder_send, serialize_comma_separated}};

pub struct GetVault;

impl<'c> Method<'c> for GetVault {
	type Input = ();
	type Output = serde_json::Value;

	type Type = NoArgs;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/vault/get";
}

#[civx(clap)]
#[derive(Serialize, Builder, Debug)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct ListVault<'c> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'c CivitAI>,

	pub limit: Option<u32>,
	pub page: Option<u32>,

	pub query: Option<String>,

	#[serde(serialize_with = "serialize_comma_separated", 
			skip_serializing_if = "Option::is_none")]
	pub types: Option<Vec<ModelType>>,

	#[serde(serialize_with = "serialize_comma_separated", 
			skip_serializing_if = "Option::is_none")]
	pub categories: Option<Vec<String>>,

	#[serde(serialize_with = "serialize_comma_separated", 
			skip_serializing_if = "Option::is_none")]
	pub base_models: Option<Vec<BaseModel>>,

	pub date_created_from: Option<DateTime<Utc>>,
	pub date_added_from: Option<DateTime<Utc>>,

	pub sort: Option<SortKind>,
}

impl_builder_send!(list_vault_builder, ListVaultBuilder, ListVault<'c>);

impl<'c> Method<'c> for ListVault<'c> {
	type Input = Self;
	type Output = serde_json::Value;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/vault/all";
}

#[civx(clap)]
#[derive(Serialize, Builder, Debug)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct CheckInVault<'c> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'c CivitAI>,

	#[serde(serialize_with = "serialize_comma_separated", 
			skip_serializing_if = "Option::is_none")]
	pub model_version_ids: Option<Vec<u32>>,
}

impl_builder_send!(check_in_vault_builder, CheckInVaultBuilder, CheckInVault<'c>);

impl<'c> Method<'c> for CheckInVault<'c> {
	type Input = Self;
	type Output = serde_json::Value;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/vault/check-vault";
}

#[civx(clap)]
#[derive(Serialize, Builder, Debug)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct ToggleVaultVersion<'c> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'c CivitAI>,

	pub model_version_id: u32,
}

impl_builder_send!(toggle_vault_version_builder, ToggleVaultVersionBuilder, ToggleVaultVersion<'c>);

impl<'c> Method<'c> for ToggleVaultVersion<'c> {
	type Input = Self;
	type Output = serde_json::Value;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::POST;
	const ENDPOINT: &'static str = "/api/v1/vault/toggle-version";
}

#[cfg(test)]
mod tests {
	use crate::tests::*;

	#[tokio::test]
	async fn mock_get_vault() -> Result<(), Box<dyn Error>> {
		mock_client!("GET", "/api/v1/vault/get", get_vault);

		CivitAI::new_auth(TOKEN)?.get_vault().await?;
		
		Ok(())
	}

	#[tokio::test]
	async fn mock_list_vault() -> Result<(), Box<dyn Error>> {
		mock_client!("GET", "/api/v1/vault/all", list_vault);
		
		CivitAI::new_auth(TOKEN)?.list_vault().send().await?;

		Ok(())
	}

	#[tokio::test]
	async fn mock_check_in_vault() -> Result<(), Box<dyn Error>> {
		mock_client!("GET", "/api/v1/vault/check-vault", check_in_vault);

		CivitAI::new_auth(TOKEN)?.check_in_vault().send().await?;

		Ok(())
	}

	#[tokio::test]
	async fn mock_toggle_vault_version() -> Result<(), Box<dyn Error>> {
		mock_client!("POST", "/api/v1/vault/toggle-version?modelVersionId=123", toggle_vault_version);

		CivitAI::new_auth(TOKEN)?.toggle_vault_version()
			.model_version_id(123)
			.send().await?;

		Ok(())
	}
}