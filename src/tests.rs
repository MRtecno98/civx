use std::error::Error;
use crate::CivitAI;

#[tokio::test]
async fn it_works() -> Result<(), Box<dyn Error>> {
	let civitai = CivitAI::new()?;

	let result = civitai.lookup_users()
		.ids(vec![123,456,789])
		.send().await?;

	for user in result.iter() {
		println!("User: {} ({})", user.username, user.id);
	}

	let model = civitai.get_model(2731187).await?;

	println!("Model: {}", model.name);
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

	Ok(())
}
