use std::{borrow::Cow, sync::Arc, time::Duration};

use axum::{Router, extract::{Query, State}, routing::get};
use chrono::Utc;
use civx::oauth::CivitAIOAuth;
use keyring::Entry;
use oauth2::{AccessToken, AuthorizationCode, CsrfToken, HttpClientError, PkceCodeChallenge, RedirectUrl, RefreshToken, RequestTokenError, Scope, StandardErrorResponse, TokenResponse, basic::BasicErrorResponseType, reqwest};
use serde::Deserialize;
use text_io::read;
use tokio::{net::TcpListener, sync::mpsc};

const CLIENT_ID: &str = "1ab9cf1b-dba5-41b6-9ecf-e73b095805e3";
const PUBLIC_REDIRECT_ENABLED: bool = false; // Set to true after setting up the public redirect

const LOCAL_BIND: &str = "127.0.0.1:61059";
const REDIRECT_URL_LOCAL: &str = "http://127.0.0.1:61059/civx";
const REDIRECT_URL_PUBLIC: &str = "https://mrtecno.dev/civx";

const SECRET_SERVICE: &str = "civx";
const SECRET_REFRESH: &str = "refresh_token";
const SECRET_ACCESS: &str = "access_token";
const SECRET_EXPIRY: &str = "expiry";

#[derive(Debug)]
pub enum AuthError {
	Io(std::io::Error),
	TokenRequest(RequestTokenError<
		HttpClientError<reqwest::Error>, 
		StandardErrorResponse<BasicErrorResponseType>
	>),
	NotLoggedIn,
	CannotOpenURL,
	Keyring(keyring::Error),
	NoCallbackReceived,
	CsrfMismatch,
}

impl From<std::io::Error> for AuthError {
	fn from(err: std::io::Error) -> Self {
		AuthError::Io(err)
	}
}

impl From<keyring::Error> for AuthError {
	fn from(err: keyring::Error) -> Self {
		AuthError::Keyring(err)
	}
}

impl From<RequestTokenError<HttpClientError<reqwest::Error>, StandardErrorResponse<BasicErrorResponseType>>> for AuthError {
	fn from(err: RequestTokenError<HttpClientError<reqwest::Error>, StandardErrorResponse<BasicErrorResponseType>>) -> Self {
		AuthError::TokenRequest(err)
	}
}

impl std::fmt::Display for AuthError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			AuthError::TokenRequest(err) => write!(f, "Token request error: {}", err),
			AuthError::Io(err) => write!(f, "IO error: {}", err),
			AuthError::NoCallbackReceived => write!(f, "No callback received"),
			AuthError::Keyring(err) => write!(f, "Keyring error: {}", err),
			AuthError::CsrfMismatch => write!(f, "CSRF token mismatch"),
			AuthError::NotLoggedIn => write!(f, "No refresh token found. Please log in first."),
			AuthError::CannotOpenURL => write!(f, "Cannot open URL in browser. Please open it manually."),
		}
	}
}

impl std::error::Error for AuthError {}

pub enum RedirectType {
	Local,
	Public,
}

impl RedirectType {
	pub fn select() -> RedirectType {
		if open::commands(REDIRECT_URL_LOCAL).is_empty() {
			RedirectType::Public
		} else {
			RedirectType::Local
		}
	}

	pub fn get_url(&self) -> RedirectUrl {
		match self {
			RedirectType::Local => RedirectUrl::new(REDIRECT_URL_LOCAL.to_string()).unwrap(),
			RedirectType::Public => RedirectUrl::new(REDIRECT_URL_PUBLIC.to_string()).unwrap(),
		}
	}
}

pub fn http_client() -> reqwest::Client {
	reqwest::Client::builder()
		.redirect(reqwest::redirect::Policy::none())
		.build()
		.unwrap()
}

pub fn oauth_client() -> CivitAIOAuth {
	civx::oauth::new_client_id(CLIENT_ID)
		.set_redirect_uri(RedirectType::select().get_url())
}

pub async fn login_session() -> Result<(), AuthError> {
	let (refresh, access) = secrets()?;
	let expiry = expiry()?;

	let oauth = oauth_client();

	let redirect = RedirectType::select();

	let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();

	let (url, csrf) = oauth
		.authorize_url(CsrfToken::new_random)
		.set_pkce_challenge(challenge)
		.set_redirect_uri(Cow::Owned(redirect.get_url()))
		.add_scopes([
			Scope::new("10701093".to_string()), // Read Only Preset
		]).url();
		
	open::that(url.as_str())?;
	println!("Please complete the login in your browser.");

	match redirect {
		RedirectType::Local => {
			let callback = create_server_and_wait().await?;

			if &callback.state != csrf.secret() {
				return Err(AuthError::CsrfMismatch);
			}

			let code = AuthorizationCode::new(callback.code);

			let token = oauth.exchange_code(code)
				.set_pkce_verifier(verifier)
				.request_async(&http_client()).await?;

			if let Some(refresh_token) = token.refresh_token() {
				refresh.set_password(refresh_token.secret())?;
			}
			
			access.set_password(token.access_token().secret())?;
			expiry.delete_credential().ok();

			if let Some(expires_in) = token.expires_in() {
				expiry.set_password(&string_expiration(expires_in)?)?;
			}
		},

		RedirectType::Public => {
			if !PUBLIC_REDIRECT_ENABLED {
				return Err(AuthError::CannotOpenURL);
			}

			println!("Insert the token: ");
			let token: String = read!("{}\n");

			refresh.set_password(&token)?;
			access.delete_credential().ok();
			expiry.delete_credential().ok();
		}
	}

	Ok(())
}

fn string_expiration(duration: Duration) -> Result<String, AuthError> {
	let expiry_time = Utc::now() + chrono::Duration::from_std(duration).unwrap();
	Ok(expiry_time.to_rfc3339())
}

pub fn secrets() -> Result<(Entry, Entry), AuthError> {
	let refresh = Entry::new(SECRET_SERVICE, SECRET_REFRESH)?;
	let access = Entry::new(SECRET_SERVICE, SECRET_ACCESS)?;

	Ok((refresh, access))
}

pub fn expiry() -> Result<Entry, AuthError> {
	let expiry = Entry::new(SECRET_SERVICE, SECRET_EXPIRY)?;

	Ok(expiry)
}

pub async fn login() -> Result<AccessToken, AuthError> {
	let oauth = oauth_client();

	let (refresh, access) = secrets()?;
	let expiry = expiry()?;

	let expired = expiry.get_password().ok().as_ref()
		.and_then(|e| chrono::DateTime::parse_from_rfc3339(e).ok())
		.map(|dt| dt < Utc::now())
		.unwrap_or(true);

	match access.get_password() {
		Ok(token) if !expired => Ok(AccessToken::new(token)),

		Err(keyring::Error::NoEntry) | Ok(_) => {
			let refresh_token = match refresh.get_password() {
				Ok(token) => RefreshToken::new(token),

				Err(keyring::Error::NoEntry) => return Err(AuthError::NotLoggedIn),
				Err(err) => return Err(AuthError::Keyring(err)),
			};

			let token = oauth.exchange_refresh_token(&refresh_token)
				.request_async(&http_client()).await?;

			access.set_password(token.access_token().secret())?;
			expiry.delete_credential().ok();

			if let Some(expires_in) = token.expires_in() {
				expiry.set_password(&string_expiration(expires_in)?)?;
			}

			Ok(token.access_token().clone())
		},
		
		Err(err) => Err(AuthError::Keyring(err)),
	}
}

pub fn session_present() -> Result<bool, AuthError> {
	let (refresh, _) = secrets()?;

	match refresh.get_password() {
		Ok(_) => Ok(true),
		Err(keyring::Error::NoEntry) => Ok(false),
		Err(err) => Err(AuthError::Keyring(err)),
	}
}

#[allow(unused)]
pub fn delete_session() -> Result<(), AuthError> {
	let (_, access) = secrets()?;

	access.delete_credential().ok();

	Ok(())
}

pub fn logout() -> Result<(), AuthError> {
	let (refresh, access) = secrets()?;

	refresh.delete_credential().ok();
	access.delete_credential().ok();

	Ok(())
}

#[derive(Debug, Deserialize)]
struct LoginCallback {
	code: String,
	state: String,
}

struct ServerState {
	shutdown: mpsc::Sender<()>,
	callback: mpsc::Sender<LoginCallback>,
}

async fn create_server_and_wait() -> Result<LoginCallback, AuthError> {
	let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);
	let (callback_tx, mut callback_rx) = mpsc::channel::<LoginCallback>(1);

	let state = Arc::new(ServerState {
		shutdown: shutdown_tx,
		callback: callback_tx,
	});

	let router = Router::new()
		.route("/civx", get(login_callback))
		.with_state(state);

	let tcp = TcpListener::bind(LOCAL_BIND).await?;

	axum::serve(tcp, router)
		.with_graceful_shutdown(async move {
			shutdown_rx.recv().await;
		}).await?;

	callback_rx.recv().await.ok_or_else(|| AuthError::NoCallbackReceived)
}

async fn login_callback(State(state): State<Arc<ServerState>>, Query(callback): Query<LoginCallback>) -> String {
	state.shutdown.send(()).await.ok();
	state.callback.send(callback).await.ok();

	"Login successful! You can close this window now.".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oauth_redirect_url_local_valid() {
        RedirectType::Local.get_url();
    }

    #[test]
    fn oauth_redirect_url_public_valid() {
        RedirectType::Public.get_url();
    }
}
