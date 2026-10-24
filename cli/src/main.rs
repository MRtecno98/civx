use civx::{CivitAI, queries::ListModels};
use clap::{Parser, builder::{Styles, styling::{AnsiColor, Effects}}};

use crate::args::{Command, ModelAction};

use futures::TryStreamExt;

mod args;
mod tables;

pub const DEFAULT_LIMIT: u32 = 10;

pub(crate) fn clap_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
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

			tables::model_table(
				&client.request::<ListModels<'_>>(query).await?.stream_n(count).try_collect::<Vec<_>>().await?)
		},

		Command::Models { action: ModelAction::Search { 
			query: _, hash: Some(hash) 
		} } => tables::print_version(
				&client.get_by_hash(hash).await?),

		Command::Models { action: ModelAction::Get { id } }
			=> tables::print_model(
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

			tables::version_table(
				&client.get_by_hash_bulk(hashes).await?);
		}

		Command::Models { action: ModelAction::Version { id, mini: false } } 
			=> tables::print_version(
				&client.get_model_version(id).await?),

		Command::Models { action: ModelAction::Version { id, mini: true } } 
			=> tables::print_version_minimal(
				&client.get_model_version_minimal(id).await?),

		Command::Whoami => {
			tables::print_whoami(&client.get_me().await?);
		},

		_ => todo!()
	};

	Ok(())
}
