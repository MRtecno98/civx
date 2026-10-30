use std::{borrow::Cow, sync::Arc, time::Duration};

use axum::{Router, extract::{Query, State}, routing::get};
use chrono::Utc;
use civx::oauth::CivitAIOAuth;
use keyring::Entry;
use oauth2::{AccessToken, AuthorizationCode, ConfigurationError, CsrfToken, HttpClientError, PkceCodeChallenge, RedirectUrl, RefreshToken, RequestTokenError, Scope, StandardErrorResponse, StandardRevocableToken, TokenResponse, basic::BasicErrorResponseType, reqwest};
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
	Revoke(ConfigurationError),
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

impl From<ConfigurationError> for AuthError {
	fn from(err: ConfigurationError) -> Self {
		AuthError::Revoke(err)
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
			AuthError::Revoke(err) => write!(f, "Token revoke error: {}", err),
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

pub struct AuthClient {
	oauth: CivitAIOAuth,
	redirect: RedirectType,
	http_client: reqwest::Client,

	refresh_entry: Entry,
	access_entry: Entry,
	expiry_entry: Entry,
}

impl AuthClient {
	pub fn new() -> Self {
		Self {
			oauth: civx::oauth::new_client_id(CLIENT_ID)
				.set_redirect_uri(RedirectType::select().get_url()),

			http_client: reqwest::Client::builder()
				.redirect(reqwest::redirect::Policy::none())
				.build()
				.unwrap(),

			redirect: RedirectType::select(),

			refresh_entry: Entry::new(SECRET_SERVICE, SECRET_REFRESH).unwrap(),
			access_entry: Entry::new(SECRET_SERVICE, SECRET_ACCESS).unwrap(),
			expiry_entry: Entry::new(SECRET_SERVICE, SECRET_EXPIRY).unwrap(),
		}
	}

	pub async fn login_session(&self) -> Result<(), AuthError> {
		let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();

		let (url, csrf) = self.oauth
			.authorize_url(CsrfToken::new_random)
			.set_pkce_challenge(challenge)
			.set_redirect_uri(Cow::Owned(self.redirect.get_url()))
			.add_scopes([
				Scope::new("10701093".to_string()), // Read Only Preset
			]).url();
			
		open::that(url.as_str())?;
		println!("Please complete the login in your browser.");

		match self.redirect {
			RedirectType::Local => {
				let callback = create_server_and_wait().await?;

				if &callback.state != csrf.secret() {
					return Err(AuthError::CsrfMismatch);
				}

				let code = AuthorizationCode::new(callback.code);

				let token = self.oauth.exchange_code(code)
					.set_pkce_verifier(verifier)
					.request_async(&self.http_client).await?;

				if let Some(refresh_token) = token.refresh_token() {
					self.refresh_entry.set_password(refresh_token.secret())?;
				}
				
				self.access_entry.set_password(token.access_token().secret())?;
				self.expiry_entry.delete_credential().ok();

				if let Some(expires_in) = token.expires_in() {
					self.expiry_entry.set_password(&string_expiration(expires_in)?)?;
				}
			},

			RedirectType::Public => {
				if !PUBLIC_REDIRECT_ENABLED {
					return Err(AuthError::CannotOpenURL);
				}

				println!("Insert the token: ");
				let token: String = read!("{}\n");

				self.refresh_entry.set_password(&token)?;
				self.access_entry.delete_credential().ok();
				self.expiry_entry.delete_credential().ok();
			}
		}

		Ok(())
	}

	pub async fn login(&self) -> Result<AccessToken, AuthError> {
		match self.access_token_not_expired()? {
			Some(token) => Ok(token),

			None => {
				let refresh_token = self.refresh_token()?.ok_or(AuthError::NotLoggedIn)?;

				let token = self.oauth.exchange_refresh_token(&refresh_token)
					.request_async(&self.http_client).await?;

				self.access_entry.set_password(token.access_token().secret())?;
				self.expiry_entry.delete_credential().ok();

				if let Some(expires_in) = token.expires_in() {
					self.expiry_entry.set_password(&string_expiration(expires_in)?)?;
				}

				Ok(token.access_token().clone())
			},
		}
	}

	pub fn access_token(&self) -> Result<Option<AccessToken>, AuthError> {
		match self.access_entry.get_password() {
			Ok(token) => Ok(Some(AccessToken::new(token))),
			Err(keyring::Error::NoEntry) => Ok(None),
			Err(err) => Err(AuthError::Keyring(err)),
		}
	}

	pub fn refresh_token(&self) -> Result<Option<RefreshToken>, AuthError> {
		match self.refresh_entry.get_password() {
			Ok(token) => Ok(Some(RefreshToken::new(token))),
			Err(keyring::Error::NoEntry) => Ok(None),
			Err(err) => Err(AuthError::Keyring(err)),
		}
	}

	pub fn access_expired(&self) -> Result<bool, AuthError> {
		let expired = self.expiry_entry.get_password().ok().as_ref()
			.and_then(|e| chrono::DateTime::parse_from_rfc3339(e).ok())
			.map(|dt| dt < Utc::now())
			.unwrap_or(true);

		Ok(expired)
	}

	pub fn access_token_not_expired(&self) -> Result<Option<AccessToken>, AuthError> {
		if self.access_expired()? {
			Ok(None)
		} else {
			self.access_token()
		}
	}

	pub fn is_session_present(&self) -> Result<bool, AuthError> {
		Ok(self.refresh_token()?.is_some())
	}

	#[expect(unused)]
	pub fn is_access_present(&self) -> Result<bool, AuthError> {
		Ok(self.access_token()?.is_some())
	}

	pub fn delete_session(&self) -> Result<(), AuthError> {
		self.refresh_entry.delete_credential().ok();
		self.delete_access().ok();

		Ok(())
	}

	pub fn delete_access(&self) -> Result<(), AuthError> {
		self.access_entry.delete_credential().ok();
		self.expiry_entry.delete_credential().ok();

		Ok(())
	}

	#[expect(unused)]
	pub fn revoke_access(&self) -> Result<(), AuthError> {
		if let Some(access_token) = self.access_token()? {
			self.oauth.revoke_token(StandardRevocableToken::AccessToken(access_token))?;
		}

		self.delete_access()?;
		Ok(())
	}

	// a.k.a. revoke_session
	pub fn logout(&self) -> Result<(), AuthError> {
		if let Some(refresh_token) = self.refresh_token()? {
			self.oauth.revoke_token(StandardRevocableToken::RefreshToken(refresh_token))?;
		}

		self.delete_session()?;
		Ok(())
	}
}

fn string_expiration(duration: Duration) -> Result<String, AuthError> {
	let expiry_time = Utc::now() + chrono::Duration::from_std(duration).unwrap();
	Ok(expiry_time.to_rfc3339())
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
