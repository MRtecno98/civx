use std::{cell::RefCell, ops::{Deref, DerefMut}};

pub use std::error::Error;
pub use crate::CivitAI;
pub use wiremock::{*, matchers::*};

thread_local! {
	pub(crate) static TEST_API_BASE: RefCell<String> = RefCell::new(crate::API_BASE.to_string());
}

pub(crate) fn set_api_base(base: impl AsRef<str>)  -> String {
	TEST_API_BASE.replace(base.as_ref().to_string())
}

pub(crate) fn reset_api_base() -> String {
	TEST_API_BASE.replace(crate::API_BASE.to_string())
}

pub(crate) struct ApiBaseGuard<T> {
	inner: T,
}

impl<T> ApiBaseGuard<T> {
	pub(crate) fn new(inner: T) -> Self {
		Self { inner }
	}
}

impl<T> Drop for ApiBaseGuard<T> {
	fn drop(&mut self) {
		crate::tests::reset_api_base();
	}
}

impl<T> Deref for ApiBaseGuard<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<T> DerefMut for ApiBaseGuard<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

#[cfg(feature = "network-tests")]
macro_rules! auth_token {
	() => {
		option_env!("TEST_TOKEN").expect(
			"TEST_TOKEN variable not set.\nPlease create a file named `test_token` in the project root with your token.")
	}
}

macro_rules! fixture {
	($name:ident) => {
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/", stringify!($name), ".json"))
	};
}

macro_rules! fixture_response {
	($name:ident, $code:expr) => {
		ResponseTemplate::new($code)
			.set_body_raw(fixture!($name), "application/json")
	};

	($name:ident) => {
		fixture_response!($name, 200)
	};
}

macro_rules! mock_client {
	() => {{
		let server = wiremock::MockServer::start().await;
		$crate::tests::set_api_base(&server.uri());

		$crate::tests::ApiBaseGuard::new(server)
	}};

	($http:expr, $path:expr, $fixture:ident, $block:block) => {{
		use $crate::tests::*;

		let server = mock_client!();

		const TOKEN: &str = "test-token";

		let path_str = $path;
		let (path_str, query) = path_str.split_once('?').unwrap_or((path_str, ""));

		let mut mock = Mock::given(method($http))
			.and(path(path_str))
			.and(header("Authorization", format!("Bearer {}", TOKEN)));

		if !query.is_empty() {
			for (key, value) in query.split('&').filter_map(|kv| kv.split_once('=')) {
				mock = mock.and(query_param(key, value));
			}
		}

		mock.respond_with(
			ResponseTemplate::new(200)
				.set_body_raw(fixture!($fixture), "application/json"))
			.mount(&server).await;

		$block

		crate::tests::reset_api_base();

		Ok(())
	}};
}

#[cfg(feature = "network-tests")]
pub(crate) use auth_token;
pub(crate) use fixture;
pub(crate) use mock_client;

#[tokio::test]
#[cfg(feature = "network-tests")]
#[ignore = "requires a token file in the project root"]
async fn it_works() -> Result<(), Box<dyn Error>> {
	use crate::air::AirIdent;

	let civitai = CivitAI::new_auth(auth_token!())?;

	let result = civitai.lookup_users()
		.ids(vec![123,456,789])
		.send().await?;

	for user in result.iter() {
		println!("User: {} ({})", user.username, user.id);
	}

	let model = civitai.get_model(2731187).await?;

	println!("Model: {} ({})", model.name, model.air());
	for ver in model.model_versions {
		println!("\tVersion: {}", ver.name);
		for file in ver.files {
			print!("\t\tFile: {} ({})", file.name, file.file_type);
			if file.primary {
				print!(" [PRIMARY]");
			}
			println!()
		}
	}

	let me = civitai.get_me().await?;
	println!("Me: {} ({})", me.username, me.id);

	Ok(())
}
