use crate::{Method, NoArgs, models::Enums};

pub struct GetEnums;

impl Method for GetEnums {
	type Input = ();
	type Output = Enums;

	type Type = NoArgs;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/enums";
}

#[cfg(test)]
mod tests {
	use crate::CivitAI;
	use std::error::Error;

	#[tokio::test]
	async fn get_enums_deser() -> Result<(), Box<dyn Error>> {
		CivitAI::new()?.get_enums().await?;

		Ok(())
	}
}
