use civx::{CivitAI, queries::ListModels};
use clap::{Parser, builder::{Styles, styling::{AnsiColor, Effects}}};

use crate::{args::{Command, ModelAction}, auth::AuthError, error::Error};

use futures::TryStreamExt;

mod args;
mod tables;
mod auth;
mod error;

pub const DEFAULT_LIMIT: u32 = 10;

pub(crate) fn clap_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
}

#[tokio::main]
async fn main() {
	if let Err(err) = entry().await {
		error::handle_error(err);
	}
}

async fn entry() -> Result<(), Error> {
	let cli = args::Args::parse();
	let auth_client = auth::AuthClient::new();

	let token = match auth_client.login().await {
		Ok(token) => Some(token.into_secret()),

		Err(AuthError::NotLoggedIn | AuthError::TokenRequest(_)) => None,

		Err(err) => Err(err)?
	};
	
	let client = match token {
		Some(token) => CivitAI::new_auth(token)?,
		None => CivitAI::new()?
	};

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

		Command::Login => {
			if auth_client.is_session_present()? {
				println!("Already logged in, use `civx logout` to log out first");
			} else {
				match auth_client.login_session().await {
					Err(AuthError::CannotOpenURL) => {
						println!("Cannot open URL in browser, login from a headless shell is not supported yet.");
					},

					_ => {
						if auth_client.is_session_present()? {
							println!("Login successful");
						} else {
							println!("Login failed, no token was received from the server");
						}
					}
				}				
			}
		},

		Command::Logout => {
			auth_client.logout()?;
			println!("Logout successful");
		},

		_ => todo!()
	};

	Ok(())
}