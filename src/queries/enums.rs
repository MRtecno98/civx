use crate::{Method, NoArgs, models::Enums};

pub struct GetEnums;

impl<'c> Method<'c> for GetEnums {
	type Input = ();
	type Output = Enums;

	type Type = NoArgs;

	const METHOD: reqwest::Method = reqwest::Method::GET;
	const ENDPOINT: &'static str = "/api/v1/enums";
}

#[cfg(test)]
mod tests {
	use crate::tests::*;

	#[tokio::test]
	#[cfg(feature = "network-tests")]
	async fn online_get_enums() -> Result<(), Box<dyn Error>> {
		CivitAI::new()?.get_enums().await?;

		Ok(())
	}

	#[tokio::test]
	async fn mock_get_enums() -> Result<(), Box<dyn Error>> {
		mock_client!("GET", "/api/v1/enums", get_enums);

		CivitAI::new_auth(TOKEN)?.get_enums().await?;

		Ok(())
	}
}
