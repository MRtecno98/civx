use std::fmt::Display;
use owo_colors::OwoColorize;

use crate::auth::AuthError;

#[derive(Debug)]
pub enum Error {
	Auth(AuthError),
	CivitAI(civx::Error),
	Io(std::io::Error),
}

impl From<AuthError> for Error {
	fn from(err: AuthError) -> Self {
		Self::Auth(err)
	}
}

impl From<civx::Error> for Error {
	fn from(err: civx::Error) -> Self {
		match err {
			civx::Error::Io(err) => Self::Io(err),
			_ => Self::CivitAI(err)
		}
	}
}

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Self::Io(err)
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Auth(err) => write!(f, "Authentication error: {err}"),
			Self::CivitAI(err) => write!(f, "CivitAI error: {err}"),
			Self::Io(err) => write!(f, "IO error: {err}"),
		}
	}
}

impl std::error::Error for Error {}

pub fn handle_error(err: Error) -> ! {
	eprint!("{} ", "Error:".red());

	match err {
		Error::Auth(err) => eprintln!("{} {}", "(auth)".bright_black(), err),

		Error::CivitAI(civx::Error::Api(err)) => {
			eprintln!("{} {}", "(API)".bright_black(), err.message);

			if err.code.is_some() || err.issues.is_some() {
				eprintln!();
			}

			if let Some(code) = &err.code {
				eprintln!("{} {}", "code:".bright_black(), code);
			}

			if let Some(issues) = &err.issues {
				for issue in issues {
					eprintln!("{} {}", "issue:".bright_black(), issue);
				}
			}
		},

		Error::CivitAI(_) => eprintln!("{} {}", "(client)".bright_black(), err),
		Error::Io(err) => eprintln!("{} {}", "(IO)".bright_black(), err),
	}

	eprintln!("\n{}", "Exiting due to error. (code: 1)".red());

	std::process::exit(1)
}
