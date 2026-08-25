use civx::{CivitAI, models::{CurrentUser, Model, ModelVersion, ModelVersionMinimal}, queries::ListModels};
use clap::{Parser, builder::{Styles, styling::{AnsiColor, Effects}}};
use comfy_table::{ContentArrangement, Table};
use itertools::Itertools;

use crate::args::{Command, ModelAction};

use futures::TryStreamExt;

mod args;

pub const DEFAULT_LIMIT: u32 = 10;

pub(crate) fn clap_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
}

fn model_table(models: &[Model]) {
	let mut table = Table::new();

	table.set_content_arrangement(ContentArrangement::Dynamic);

	table.set_header(vec!["Model", "ID", "Type", "Base Models", "NSFW", "Creator", "Tags"]);

	for model in models {
		let base_models = model.model_versions.iter()
			.map(|v| v.base_model.clone())
			.unique()
			.map(|bm| bm.to_string())
			.collect::<Vec<_>>()
			.join(", ");

		table.add_row(vec![
			model.name.clone(),
			model.id.to_string(),
			model.model_type.to_string(),
			base_models,
			model.nsfw.to_string(),
			model.creator.as_ref().map_or("N/A".to_string(), |c| c.username.clone()),
			model.tags.join(", "),
		]);
	}

	println!("{table}");
}

fn version_table(versions: &[ModelVersion]) {
	println!("Debug: Version table with {} versions", versions.len());

}

fn print_model(model: &Model) {
	println!("Debug: Model with id {} and name {}", model.id, model.name);

}

fn print_version(version: &ModelVersion) {
	println!("Debug: Version with id {} and name {}", version.id, version.name);

}

fn print_version_minimal(version: &ModelVersionMinimal) {
	println!("Debug: Version minimal with air {} and name {}", version.air, version.model_name);

}

fn print_whoami(current: &CurrentUser) {
	println!("Debug: Current user with id {} and username {}", current.id, current.username);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let cli = args::Args::parse();

	let client = CivitAI::new()?;

	match cli.command {
		Command::Models { action: ModelAction::Search {
			query, hash: None 
		} } => {
			let count = query.pagination.as_ref().and_then(|p| p.limit).unwrap_or(DEFAULT_LIMIT) as usize;

			model_table(
				&client.request::<ListModels<'_>>(query).await?.stream_n(count).try_collect::<Vec<_>>().await?)
		},

		Command::Models { action: ModelAction::Search { 
			query: _, hash: Some(hash) 
		} } => print_version(
				&client.get_by_hash(hash).await?),

		Command::Models { action: ModelAction::Get { id } }
			=> print_model(
				&client.get_model(id).await?),

		Command::Models { action: ModelAction::BulkSearch { 
			hashes, input 
		} } => {
			let hashes = match (hashes, input) {
				(Some(hashes), None) => hashes,

				(None, Some(input)) => {
					tokio::fs::read_to_string(input).await?
						.lines().map(&str::to_string).collect::<Vec<_>>()
				}

				_ => unreachable!()
			};

			version_table(
				&client.get_by_hash_bulk(hashes).await?);
		}

		Command::Models { action: ModelAction::Version { id, mini: false } } 
			=> print_version(
				&client.get_model_version(id).await?),

		Command::Models { action: ModelAction::Version { id, mini: true } } 
			=> print_version_minimal(
				&client.get_model_version_minimal(id).await?),

		Command::Whoami => {
			print_whoami(&client.get_me().await?);
		},

		_ => todo!()
	};

	Ok(())
}
