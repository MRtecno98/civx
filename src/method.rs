use std::fmt::Display;

use serde::{Serialize, de::DeserializeOwned};

use crate::Result;

pub trait Method<'c> {
	type Input;
	type Output: DeserializeOwned;

	type Type: MethodType<'c, Self>;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str;

	#[allow(unused_variables)]
	fn post_request(request: Option<Self::Input>, output: &mut Self::Output, client: &'c crate::CivitAI) {}
}

pub trait MethodType<'c, M: Method<'c> + ?Sized> {
	fn url(_input: &M::Input) -> Result<impl AsRef<str>> {
		Ok(M::ENDPOINT)
	}

	fn apply(_input: &M::Input, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
		request
	}
}

pub struct NoArgs;
pub struct Query;
pub struct Path;
pub struct Body;

impl<'c, M: Method<'c> + ?Sized> MethodType<'c, M> for NoArgs {}

impl<'c, M: Method<'c> + ?Sized> MethodType<'c, M> for Query where M::Input: Serialize {
	fn url(input: &M::Input) -> Result<impl AsRef<str>> {
		Ok(format!("{}?{}", M::ENDPOINT, serde_url_params::to_string(input)?))
	}
}

impl<'c, M: Method<'c> + ?Sized> MethodType<'c, M> for Path where M::Input: Display {
	fn url(input: &M::Input) -> Result<impl AsRef<str>> {
		Ok(M::ENDPOINT.replace("{}", &input.to_string()))
	}
}

impl<'c, M: Method<'c> + ?Sized> MethodType<'c, M> for Body where M::Input: Serialize {
	fn apply(input: &M::Input, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
		request.json(input)
	}
}
