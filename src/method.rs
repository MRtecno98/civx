use std::fmt::Display;

use serde::{Serialize, de::DeserializeOwned};
use serde_url_params;

use crate::Result;

pub trait Method {
	type Input;
	type Output: DeserializeOwned;

	type Type: MethodType<Self>;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str;
}

pub trait MethodType<M: Method + ?Sized> {
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

impl<M: Method + ?Sized> MethodType<M> for NoArgs {}

impl<M: Method<Input: Serialize> + ?Sized> MethodType<M> for Query {
	fn url(input: &M::Input) -> Result<impl AsRef<str>> {
		Ok(format!("{}?{}", M::ENDPOINT, serde_url_params::to_string(input)?))
	}
}

impl<M: Method<Input: Display> + ?Sized> MethodType<M> for Path {
	fn url(input: &M::Input) -> Result<impl AsRef<str>> {
		Ok(M::ENDPOINT.replace("{}", &input.to_string()))
	}
}

impl <M: Method<Input: Serialize> + ?Sized> MethodType<M> for Body {
	fn apply(input: &M::Input, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
		request.json(input)
	}
}