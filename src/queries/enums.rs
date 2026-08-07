use crate::{Method, NoArgs};

pub struct GetEnums;

impl Method for GetEnums {
	type Input = ();
	type Output = serde_json::Value;

	type Type = NoArgs;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/enums";
}
