use std::error::Error;
use crate::CivitAI;

#[tokio::test]
async fn it_works() -> Result<(), Box<dyn Error>> {
	let civitai = CivitAI::new()?;

	#[allow(unused)]
	let result = civitai.lookup_users()
		.ids(vec![123,456,789])
		.send().await?;

	let model = civitai.get_model(2731187).await?;

	println!("{}", model);

	Ok(())
}
