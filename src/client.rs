use url::Url;

use crate::{AIR, Method, MethodType, Result, error::{ApiError, Error}, models::Model, queries::*};

#[derive(Debug)]
pub struct CivitAI {
	client: reqwest::Client,
	token: Option<String>
}

macro_rules! impl_method {
	($method:ty, $name:ident, noargs) => {
		#[inline]
		pub async fn $name(&self) -> $crate::Result<<$method as Method<'_>>::Output> {
			self.request::<$method>(()).await
		}
	};

	($method:ty, $name:ident, args($input:ident)) => {
		#[inline]
		pub async fn $name(&self, $input: <$method as Method<'_>>::Input) -> $crate::Result<<$method as Method<'_>>::Output> {
			self.request::<$method>($input).await
		}
	};

	($method:ty, $builder:ident, $name:ident) => {
		#[inline]
		pub fn $name(&self) -> $builder<'_> {
			<$method>::builder().client(self)
		}
	};
}

impl CivitAI {
	#[must_use]
	pub const fn new_client_auth(token: String, client: reqwest::Client) -> Self {
		Self {
			client,
			token: Some(token)
		}
	}

	pub fn new_auth(token: impl AsRef<str>) -> Result<Self> {
		Ok(Self {
			token: Some(token.as_ref().into()),
			..Self::new()?
		})
	}

	#[must_use]
	pub const fn new_client(client: reqwest::Client) -> Self {
		Self {
			client,
			token: None
		}
	}

	pub fn new() -> Result<Self> {
		Ok(Self::new_client(
			reqwest::Client::builder()
					.cookie_store(true).build()?))
	}

	#[inline]
	fn base_url() -> impl AsRef<str> {
		#[cfg(not(test))]
		{
			crate::API_BASE
		}

		#[cfg(test)]
		{
			crate::tests::TEST_API_BASE.with_borrow(String::clone)
		}
	}

	pub(crate) fn make_request(&self, method: reqwest::Method, url: &str) -> Result<reqwest::RequestBuilder> {
		let base_url = Url::parse(Self::base_url().as_ref())?;

		let url = match Url::parse(url) {
			Ok(url) => url,
			Err(_) => base_url.join(url)?,
		};

		if url.domain() != base_url.domain() {
			return Err(Error::InvalidEndpoint);
		}

		let mut request = self.client.request(method, url);

		if let Some(token) = &self.token {
			request = request.bearer_auth(token);
		}

		Ok(request)
	}

	pub(crate) async fn request_url<'c, M: Method<'c>>(&'c self, url: impl AsRef<str>, input: Option<M::Input>) -> Result<M::Output> {
		let request = self.make_request(M::METHOD, url.as_ref())?;

		let request = match &input {
			Some(input) => M::Type::apply(input, request),
			None => request,
		};

		let mut result = 
			ApiError::check(request.send().await?).await?.json().await?;

		<M as Method>::post_request(input, &mut result, self);

		Ok(result)
	}

	pub async fn request<'c, M: Method<'c>>(&'c self, input: M::Input) -> Result<M::Output> {
		let url = M::Type::url(&input)?.as_ref().to_owned();

		self.request_url::<M>(url, Some(input)).await
	}

	pub async fn get_air(&self, model: &Model) -> Result<AIR> {
		if let Some(version) = model.model_versions.first() {
			Ok(self.get_model_version(version.id).await?.air)
		} else {
			Err(Error::NoVersionsPublished)
		}
	}

	impl_method!(ListModels, ListModelsBuilder, list_models);
	impl_method!(GetModel, get_model, args(id));
	impl_method!(GetModelVersion, get_model_version, args(version_id));
	impl_method!(GetModelVersionMinimal, get_model_version_minimal, args(version_id));
	impl_method!(GetByHash, get_by_hash, args(hash));
	impl_method!(GetByHashBulk, get_by_hash_bulk, args(hashes));
	impl_method!(GetIdsByHashBulk, get_ids_by_hash, args(hashes));

	impl_method!(ListImages, ListImagesBuilder, list_images);
	
	impl_method!(ListArticles, ListArticlesBuilder, list_articles);
	impl_method!(GetArticle, get_article, args(id));

	impl_method!(ListCollections, ListCollectionsBuilder, list_collections);
	impl_method!(GetCollection, get_collection, args(id));

	impl_method!(ListCreators, ListCreatorsBuilder, list_creators);
	impl_method!(GetEnums, get_enums, noargs);

	impl_method!(ListTags, ListTagsBuilder, list_tags);

	impl_method!(GetMe, get_me, noargs);
	impl_method!(LookupUsers, LookupUsersBuilder, lookup_users);

	impl_method!(CheckPermissions, CheckPermissionsBuilder, check_permissions);

	impl_method!(GetVault, get_vault, noargs);
	impl_method!(ListVault, ListVaultBuilder, list_vault);
	impl_method!(CheckInVault, CheckInVaultBuilder, check_in_vault);
	impl_method!(ToggleVaultVersion, ToggleVaultVersionBuilder, toggle_vault_version);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	#[should_panic(expected = "invalid endpoint")]
	async fn nonrelative_url_should_fail() {
		let civitai = CivitAI::new().unwrap();

		#[allow(unused_must_use)]
		civitai.make_request(reqwest::Method::GET, "https://invalid-url.com")
			.expect("invalid endpoint");
	}
}