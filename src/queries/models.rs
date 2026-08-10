use bon::Builder;
use serde::Serialize;
use crate::{Body, CivitAI, Method, Path, Query, enums::{CheckpointType, ModelType, Period, SortKind}, models::{Model, ModelVersion, ModelVersionHashLookup, ModelVersionMinimal, Paginated}, queries::{Pagination, impl_builder_send}};

#[derive(Serialize, Builder)]
#[builder(on(String, into))]
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
	type Output = Paginated<Model>;

	type Type = Query;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/models";
}

pub struct GetModel;

impl Method for GetModel {
	type Input = i64;
	type Output = Model;

	type Type = Path;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/models/{}";
}

pub struct GetModelVersion;

impl Method for GetModelVersion {
	type Input = i64;
	type Output = ModelVersion;

	type Type = Path;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/model-versions/{}";
}

pub struct GetByHash;

impl Method for GetByHash {
	type Input = String;
	type Output = ModelVersion;

	type Type = Path;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/model-versions/by-hash/{}";
}

pub struct GetByHashBulk;

impl Method for GetByHashBulk {
	type Input = Vec<String>;
	type Output = Vec<ModelVersion>;

	type Type = Body;

	const METHOD: reqwest::Method = reqwest::Method::POST;
	const ENDPOINT: &'static str = "/api/v1/model-versions/by-hash";
}

pub struct GetIdsByHashBulk;

impl Method for GetIdsByHashBulk {
	type Input = Vec<String>;
	type Output = Vec<ModelVersionHashLookup>;

	type Type = Body;

	const METHOD: reqwest::Method = reqwest::Method::POST;
	const ENDPOINT: &'static str = "/api/v1/model-versions/by-hash/ids";
}

pub struct GetModelVersionMinimal;

impl Method for GetModelVersionMinimal {
	type Input = i64;
	type Output = ModelVersionMinimal;

	type Type = Path;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/model-versions/mini/{}";
}

#[cfg(test)]
mod tests {
	use crate::tests::*;

	#[tokio::test]
	#[cfg(feature = "network-tests")]
	async fn online_list_models() -> Result<(), Box<dyn Error>> {
		use crate::enums::SortKind;
		
		CivitAI::new()?.list_models()
			.pagination(Some(10), None, None)
			.sort(SortKind::HighestRated)
			.send().await?;

		Ok(())
	}

	#[tokio::test]
	#[cfg(feature = "network-tests")]
	async fn online_get_model() -> Result<(), Box<dyn Error>> {
		CivitAI::new()?.get_model(2731187).await?;

		Ok(())
	}

	#[tokio::test]
	#[cfg(feature = "network-tests")]
	async fn online_get_model_version() -> Result<(), Box<dyn Error>> {
		CivitAI::new()?.get_model_version(135867).await?;

		Ok(())
	}

	#[tokio::test]
	#[cfg(feature = "network-tests")]
	async fn online_get_model_version_by_hash() -> Result<(), Box<dyn Error>> {
		CivitAI::new()?.get_by_hash("0D9BD1B873A7863E128B4672E3E245838858F71469A3CEC58123C16C06F83BD7".into()).await?;

		Ok(())
	}

	#[tokio::test]
	#[cfg(feature = "network-tests")]
	async fn online_get_model_version_by_hash_bulk() -> Result<(), Box<dyn Error>> {
		CivitAI::new()?.get_by_hash_bulk(vec![
			"A5E5A941A3217247DBCECEEE5B67F8D6B1EF2514260E08A5757436BEC7035F93".into(),
			"B8821A5D58746D1A6306ECC99EDA3B0268FF3DA84C40D18CE68698E3BD402635".into(),
		]).await?;

		Ok(())
	}

	#[tokio::test]
	#[cfg(feature = "network-tests")]
	async fn online_get_ids_by_hash() -> Result<(), Box<dyn Error>> {
		CivitAI::new()?.get_ids_by_hash(vec![
			"A5E5A941A3217247DBCECEEE5B67F8D6B1EF2514260E08A5757436BEC7035F93".into(),
			"B8821A5D58746D1A6306ECC99EDA3B0268FF3DA84C40D18CE68698E3BD402635".into(),
		]).await?;

		Ok(())
	}

	#[tokio::test]
	#[cfg(feature = "network-tests")]
	async fn online_get_model_version_minimal() -> Result<(), Box<dyn Error>> {
		CivitAI::new()?.get_model_version_minimal(135867).await?;

		Ok(())
	}

	#[tokio::test]
	async fn mock_list_models() -> Result<(), Box<dyn Error>> {
		use crate::enums::SortKind;
		mock_client!("GET", "/api/v1/models?limit=10&sort=Highest Rated", list_models, {
			CivitAI::new_auth(TOKEN)?.list_models()
				.pagination(Some(10), None, None)
				.sort(SortKind::HighestRated)
				.send().await?;
		})
	}

	#[tokio::test]
	async fn mock_get_model() -> Result<(), Box<dyn Error>> {
		mock_client!("GET", "/api/v1/models/2731187", get_model, {
			CivitAI::new_auth(TOKEN)?.get_model(2731187).await?;
		})
	}

	#[tokio::test]
	async fn mock_get_model_version() -> Result<(), Box<dyn Error>> {
		mock_client!("GET", "/api/v1/model-versions/135867", get_model_version, {
			CivitAI::new_auth(TOKEN)?.get_model_version(135867).await?;
		})
	}

	#[tokio::test]
	async fn mock_get_model_version_by_hash() -> Result<(), Box<dyn Error>> {
		mock_client!("GET", 
			"/api/v1/model-versions/by-hash/0D9BD1B873A7863E128B4672E3E245838858F71469A3CEC58123C16C06F83BD7", get_by_hash, {
			CivitAI::new_auth(TOKEN)?.get_by_hash("0D9BD1B873A7863E128B4672E3E245838858F71469A3CEC58123C16C06F83BD7".into()).await?;
		})
	}

	#[tokio::test]
	async fn mock_get_model_version_by_hash_bulk() -> Result<(), Box<dyn Error>> {
		mock_client!("POST", "/api/v1/model-versions/by-hash", get_by_hash_bulk, {
			CivitAI::new_auth(TOKEN)?.get_by_hash_bulk(vec![
				"A5E5A941A3217247DBCECEEE5B67F8D6B1EF2514260E08A5757436BEC7035F93".into(),
				"B8821A5D58746D1A6306ECC99EDA3B0268FF3DA84C40D18CE68698E3BD402635".into(),
			]).await?;
		})
	}

	#[tokio::test]
	async fn mock_get_ids_by_hash() -> Result<(), Box<dyn Error>> {
		mock_client!("POST", "/api/v1/model-versions/by-hash/ids", get_ids_by_hash, {
			CivitAI::new_auth(TOKEN)?.get_ids_by_hash(vec![
				"A5E5A941A3217247DBCECEEE5B67F8D6B1EF2514260E08A5757436BEC7035F93".into(),
				"B8821A5D58746D1A6306ECC99EDA3B0268FF3DA84C40D18CE68698E3BD402635".into(),
			]).await?;
		})
	}

	#[tokio::test]
	async fn mock_get_model_version_minimal() -> Result<(), Box<dyn Error>> {
		mock_client!("GET", "/api/v1/model-versions/mini/135867", get_model_version_minimal, {
			CivitAI::new_auth(TOKEN)?.get_model_version_minimal(135867).await?;
		})
	}

}
