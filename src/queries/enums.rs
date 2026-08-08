use crate::{Method, NoArgs, models::Enums};

pub struct GetEnums;

impl Method for GetEnums {
	type Input = ();
	type Output = Enums;

	type Type = NoArgs;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/enums";
}
