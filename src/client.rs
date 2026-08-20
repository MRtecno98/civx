use url::Url;

use crate::{AIR, Method, MethodType, Result, error::{ApiError, Error}, models::Model, queries::*};

/// This is the main entry point for interacting with the API,
/// and manages authentication and request building. It is recommended to create a single instance of this
/// for each identity that needs to interact with the API, and reuse it for all requests.
/// 
/// The same usage tips apply as with [`reqwest::Client`], which this library uses under the hood. 
/// See its documentation for more details.
#[derive(Debug)]
pub struct CivitAI {
	client: reqwest::Client,
	token: Option<String>
}

macro_rules! impl_method {
	($method:ty, $name:ident, noargs) => {
		#[inline]
		#[doc = concat!("See [`", stringify!($method), "`](crate::queries::", stringify!($method), ") for more details.")]
		pub async fn $name(&self) -> $crate::Result<<$method as Method<'_>>::Output> {
			self.request::<$method>(()).await
		}
	};

	($method:ty, $name:ident, args($input:ident)) => {
		#[inline]
		#[doc = concat!("See [`", stringify!($method), "`](crate::queries::", stringify!($method), ") for more details.")]
		pub async fn $name(&self, $input: <$method as Method<'_>>::Input) -> $crate::Result<<$method as Method<'_>>::Output> {
			self.request::<$method>($input).await
		}
	};

	($method:ty, $builder:ident, $name:ident) => {
		#[inline]
		#[doc = concat!("See [`", stringify!($method), "`](crate::queries::", stringify!($method), ") for more details.")]
		pub fn $name(&self) -> $builder<'_> {
			<$method>::builder().client(self)
		}
	};
}

impl CivitAI {
	/// Creates a new instance of the CivitAI client with the given authentication token and HTTP client.
	/// 
	/// You can use this if you want to bring your own HTTP client with custom configuration, such as a proxy or custom headers,
	/// or to reuse it across multiple instances of the CivitAI client or other api clients.
	#[must_use]
	pub const fn new_client_auth(token: String, client: reqwest::Client) -> Self {
		Self {
			client,
			token: Some(token)
		}
	}

	/// Creates a new instance of the CivitAI client with the given authentication token.
	/// 
	/// This will create a new HTTP client with default configuration, which is suitable for most use cases.
	pub fn new_auth(token: impl AsRef<str>) -> Result<Self> {
		Ok(Self {
			token: Some(token.as_ref().into()),
			..Self::new()?
		})
	}

	/// Creates a new instance of the CivitAI client with the given HTTP client and no authentication.
	/// 
	/// Note that some endpoints require authentication, and will return an error if you try to access them without a token.
	/// 
	/// You can use this if you want to bring your own HTTP client with custom configuration, such as a proxy or custom headers,
	/// or to reuse it across multiple instances of the CivitAI client or other api clients.
	#[must_use]
	pub const fn new_client(client: reqwest::Client) -> Self {
		Self {
			client,
			token: None
		}
	}

	/// Creates a new instance of the CivitAI client with no authentication.
	/// 
	/// Note that some endpoints require authentication, and will return an error if you try to access them without a token.
	/// 
	/// This will create a new HTTP client with default configuration, which is suitable for most use cases.
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

	/// Sends a request to the API with the given input, and returns the output.
	/// 
	/// This is the main entry point for sending requests to the API, and is used by all other methods in this library.
	/// 
	/// You can also use this method to send requests of a generic type, if you need to write generic code
	/// around this library. 
	/// 
	/// See also the [`Method`] trait for more details on how to implement your own request types.
	/// 
	/// # Examples
	/// ```rust, no_run
	/// # use civx::{CivitAI, queries::GetModel};
	/// # tokio_test::block_on(async {
	/// let civitai = CivitAI::new()?;
	/// 
	/// let model = civitai.request::<GetModel>(123456).await
	///     .expect("Failed to get model 123456");
	/// # Ok::<(), Box<dyn std::error::Error>>(())
	/// # });
	/// ```
	pub async fn request<'c, M: Method<'c>>(&'c self, input: M::Input) -> Result<M::Output> {
		let url = M::Type::url(&input)?.as_ref().to_owned();

		self.request_url::<M>(url, Some(input)).await
	}

	/// Tries to make an educated guess at an AIR identifier somewhat related
	/// to the given model, by looking at its first version and returning its AIR identifier.
	/// 
	/// Usually AIRs are associated with a model version, and not the model itself, 
	/// so this is a convenience method if you don't care about which version of the model 
	/// you are using. Note that some models upload vastly different artifacts as
	/// versions (such as alternatives for different base models), that may not be
	/// all compatible. So use this at your own risk, if you need to be sure,
	/// use [`CivitAI::get_model_version`] and use the AIR from the version instead.
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