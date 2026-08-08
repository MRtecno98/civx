use bon::Builder;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{CivitAI, Method, NoArgs, Query, enums::{BaseModel, ModelType, SortKind}, queries::{impl_builder_send, serialize_comma_separated}};

pub struct GetVault;

impl Method for GetVault {
	type Input = ();
	type Output = serde_json::Value;

	type Type = NoArgs;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/vault/get";
}

#[derive(Serialize, Builder)]
#[builder(on(String, into))]
pub struct ListVault<'a> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'a CivitAI>,

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

impl_builder_send!(list_vault_builder, ListVaultBuilder, ListVault<'a>);

impl Method for ListVault<'_> {
	type Input = Self;
	type Output = serde_json::Value;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/vault/all";
}

#[derive(Serialize, Builder)]
#[builder(on(String, into))]
pub struct CheckInVault<'a> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'a CivitAI>,

	#[serde(serialize_with = "serialize_comma_separated", 
			skip_serializing_if = "Option::is_none")]
	pub model_version_ids: Option<Vec<u32>>,
}

impl_builder_send!(check_in_vault_builder, CheckInVaultBuilder, CheckInVault<'a>);

impl Method for CheckInVault<'_> {
	type Input = Self;
	type Output = serde_json::Value;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/vault/check-vault";
}

#[derive(Serialize, Builder)]
#[builder(on(String, into))]
pub struct ToggleVaultVersion<'a> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'a CivitAI>,

	pub model_version_id: u32,
}

impl_builder_send!(toggle_vault_version_builder, ToggleVaultVersionBuilder, ToggleVaultVersion<'a>);

impl Method for ToggleVaultVersion<'_> {
	type Input = Self;
	type Output = serde_json::Value;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::POST;
	const ENDPOINT: &'static str = "/api/v1/vault/toggle-version";
}
