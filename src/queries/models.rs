use bon::Builder;
use serde::Serialize;
use serde_json::Value;
use crate::{Body, CivitAI, Method, Path, Query, enums::{CheckpointType, ModelType, Period, SortKind}, queries::{Pagination, impl_builder_send}};

#[derive(Serialize, Builder)]
pub struct ListModels<'a> {
	#[serde(skip)]
	#[builder(field)]
	_client: Option<&'a CivitAI>,

	#[serde(flatten)]
	#[builder(with = 
		|limit: Option<u32>, page: Option<u32>, cursor: Option<String>| 
			Pagination { limit, page, cursor })]
	pub pagination: Option<Pagination>,

	pub query: Option<String>,
	pub ids: Option<Vec<u32>>,
	pub tags: Option<Vec<String>>,
	pub username: Option<String>,

	pub types: Option<Vec<ModelType>>,

	pub base_models: Option<Vec<String>>,

	pub checkpoint_type: Option<CheckpointType>,
	pub sort: Option<SortKind>,
	pub period: Option<Period>,

	pub nsfw: Option<bool>,
	pub supports_generation: Option<bool>,
	pub from_platform: Option<String>,
	pub early_access: Option<bool>,
	pub primary_file_only: Option<bool>,

	pub favorites: Option<bool>,
	pub hidden: Option<bool>,
}

impl_builder_send!(list_models_builder, ListModelsBuilder, ListModels<'a>);

impl Method for ListModels<'_> {
	type Input = Self;
	type Output = Value;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/models";
}

pub struct GetModel;

impl Method for GetModel {
	type Input = u32;
	type Output = Value;

	type Type = Path;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/models/{}";
}

pub struct GetModelVersion;

impl Method for GetModelVersion {
	type Input = u32;
	type Output = Value;

	type Type = Path;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/model-versions/{}";
}

pub struct GetByHash;

impl Method for GetByHash {
	type Input = String;
	type Output = Value;

	type Type = Path;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/model-versions/by-hash/{}";
}

pub struct GetByHashBulk;

impl Method for GetByHashBulk {
	type Input = Vec<String>;
	type Output = Value;

	type Type = Body;

	const METHOD: reqwest::Method = reqwest::Method::POST;
	const ENDPOINT: &'static str = "/api/v1/model-versions/by-hash";
}

pub struct GetIdsByHashBulk;

impl Method for GetIdsByHashBulk {
	type Input = Vec<String>;
	type Output = Value;

	type Type = Body;

	const METHOD: reqwest::Method = reqwest::Method::POST;
	const ENDPOINT: &'static str = "/api/v1/model-versions/by-hash/ids";
}

pub struct GetModelVersionMinimal;

impl Method for GetModelVersionMinimal {
	type Input = u32;
	type Output = Value;

	type Type = Path;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/model-versions/mini/{}";
}
