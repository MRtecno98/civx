use std::time::Duration;
use oauth2::basic::{
    BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenType,
};
use oauth2::{
    AccessToken, Client, EndpointNotSet, EndpointSet, RefreshToken, Scope,
    StandardRevocableToken, TokenResponse,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub static AUTH_URL: &str = "https://auth.civitai.com/api/auth/oauth/authorize";
pub static TOKEN_URL: &str = "https://auth.civitai.com/api/auth/oauth/token";
pub static REVOCATION_URL: &str = "https://auth.civitai.com/api/auth/oauth/revoke";
// Missing introspection
// Missing device flow

pub fn new_client_id(client_id: impl AsRef<str>) -> CivitAIOAuth {
	new_client(oauth2::ClientId::new(client_id.as_ref().to_string()))
}

pub fn new_client(client_id: oauth2::ClientId) -> CivitAIOAuth {
	Client::new(client_id)
		.set_auth_uri(oauth2::AuthUrl::new(AUTH_URL.to_string()).unwrap())
		.set_token_uri(oauth2::TokenUrl::new(TOKEN_URL.to_string()).unwrap())
		.set_revocation_url(oauth2::RevocationUrl::new(REVOCATION_URL.to_string()).unwrap())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivitaiTokenResponse {
    access_token: AccessToken,

    #[serde(
        deserialize_with = "deserialize_token_type",
        serialize_with = "serialize_token_type"
    )]
    token_type: BasicTokenType,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    expires_in: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    refresh_token: Option<RefreshToken>,

    #[serde(
        rename = "scope",
        default,
        deserialize_with = "deserialize_scopes",
        serialize_with = "serialize_scopes",
        skip_serializing_if = "Option::is_none"
    )]
    scopes: Option<Vec<Scope>>,
}

impl TokenResponse for CivitaiTokenResponse {
    type TokenType = BasicTokenType;

    fn access_token(&self) -> &AccessToken {
        &self.access_token
    }

    fn token_type(&self) -> &Self::TokenType {
        &self.token_type
    }

    fn expires_in(&self) -> Option<Duration> {
        self.expires_in.map(Duration::from_secs)
    }

    fn refresh_token(&self) -> Option<&RefreshToken> {
        self.refresh_token.as_ref()
    }

    fn scopes(&self) -> Option<&Vec<Scope>> {
        self.scopes.as_ref()
    }
}

pub type CivitAIOAuth = Client<
    BasicErrorResponse,
    CivitaiTokenResponse,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
	EndpointSet, 	// HasAuthUrl
	EndpointNotSet, // HasDeviceAuthUrl
	EndpointNotSet, // HasIntrospectionUrl
	EndpointSet, 	// HasRevocationUrl
	EndpointSet, 	// HasTokenUrl
>;

#[derive(Deserialize)]
#[serde(untagged)]
enum ScopeItem {
    Str(String),
    Num(u64),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ScopeRepr {
    Delimited(String),
    List(Vec<ScopeItem>),
}

fn deserialize_scopes<'de, D>(d: D) -> Result<Option<Vec<Scope>>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<ScopeRepr>::deserialize(d)?.map(|repr| match repr {
        ScopeRepr::Delimited(s) => s.split_whitespace().map(|s| Scope::new(s.to_owned())).collect(),
        ScopeRepr::List(items) => items
            .into_iter()
            .map(|item| match item {
                ScopeItem::Str(s) => Scope::new(s),
                ScopeItem::Num(n) => Scope::new(n.to_string()),
            })
            .collect(),
    }))
}

fn serialize_scopes<S>(scopes: &Option<Vec<Scope>>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match scopes {
        Some(v) => s.serialize_some(
            &v.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" "),
        ),
        None => s.serialize_none(),
    }
}

fn deserialize_token_type<'de, D>(d: D) -> Result<BasicTokenType, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    Ok(match s.to_ascii_lowercase().as_str() {
        "bearer" => BasicTokenType::Bearer,
        "mac" => BasicTokenType::Mac,
        _ => BasicTokenType::Extension(s),
    })
}

fn serialize_token_type<S>(tt: &BasicTokenType, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(match tt {
        BasicTokenType::Bearer => "bearer",
        BasicTokenType::Mac => "mac",
        BasicTokenType::Extension(ext) => ext,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
	
    fn parse(v: Value) -> CivitaiTokenResponse {
        serde_json::from_value(v).expect("token response should parse")
    }

    fn scope_strs(r: &CivitaiTokenResponse) -> Option<Vec<&str>> {
        r.scopes().map(|v| v.iter().map(|s| s.as_str()).collect())
    }

    #[test]
    fn oauth_auth_url_valid() {
        oauth2::AuthUrl::new(AUTH_URL.to_string()).unwrap();
    }

    #[test]
    fn oauth_token_url_valid() {
        oauth2::TokenUrl::new(TOKEN_URL.to_string()).unwrap();
    }

    #[test]
    fn oauth_revocation_url_valid() {
        oauth2::RevocationUrl::new(REVOCATION_URL.to_string()).unwrap();
    }

    #[test]
    fn scope_as_json_array() {
        let r = parse(json!({
            "access_token": "abc",
            "token_type": "Bearer",
            "scope": ["10701093", "extra"]
        }));
        assert_eq!(scope_strs(&r), Some(vec!["10701093", "extra"]));
    }

    #[test]
    fn scope_as_numeric_array() {
        let r = parse(json!({
            "access_token": "abc",
            "token_type": "bearer",
            "scope": [10701093, 42]
        }));
        assert_eq!(scope_strs(&r), Some(vec!["10701093", "42"]));
    }

    #[test]
    fn scope_as_mixed_array() {
        let r = parse(json!({
            "access_token": "abc",
            "token_type": "bearer",
            "scope": ["read", 7]
        }));
        assert_eq!(scope_strs(&r), Some(vec!["read", "7"]));
    }

    #[test]
    fn scope_as_space_delimited_string() {
        let r = parse(json!({
            "access_token": "abc",
            "token_type": "bearer",
            "scope": "read  write\tadmin"
        }));
        assert_eq!(scope_strs(&r), Some(vec!["read", "write", "admin"]));
    }

    #[test]
    fn scope_empty_string_and_empty_array() {
        let a = parse(json!({ "access_token": "a", "token_type": "bearer", "scope": "" }));
        let b = parse(json!({ "access_token": "a", "token_type": "bearer", "scope": [] }));
        assert_eq!(scope_strs(&a), Some(vec![]));
        assert_eq!(scope_strs(&b), Some(vec![]));
    }

    #[test]
    fn scope_missing_or_null() {
        let missing = parse(json!({ "access_token": "a", "token_type": "bearer" }));
        let null = parse(json!({ "access_token": "a", "token_type": "bearer", "scope": null }));
        assert!(missing.scopes().is_none());
        assert!(null.scopes().is_none());
    }

    #[test]
    fn scope_invalid_type_is_rejected() {
        let res = serde_json::from_value::<CivitaiTokenResponse>(json!({
            "access_token": "a",
            "token_type": "bearer",
            "scope": { "nope": true }
        }));
        assert!(res.is_err());
    }

    #[test]
    fn token_type_is_case_insensitive() {
        for tt in ["bearer", "Bearer", "BEARER"] {
            let r = parse(json!({ "access_token": "a", "token_type": tt }));
            assert_eq!(*r.token_type(), BasicTokenType::Bearer, "input: {tt}");
        }
        let mac = parse(json!({ "access_token": "a", "token_type": "MAC" }));
        assert_eq!(*mac.token_type(), BasicTokenType::Mac);
    }

    #[test]
    fn token_type_unknown_becomes_extension_preserving_case() {
        let r = parse(json!({ "access_token": "a", "token_type": "DPoP" }));
        assert_eq!(*r.token_type(), BasicTokenType::Extension("DPoP".into()));
    }

    #[test]
    fn token_type_missing_is_rejected() {
        let res = serde_json::from_value::<CivitaiTokenResponse>(json!({ "access_token": "a" }));
        assert!(res.is_err());
    }

    #[test]
    fn full_response_fields() {
        let r = parse(json!({
            "access_token": "access-123",
            "token_type": "Bearer",
            "expires_in": 3600,
            "refresh_token": "refresh-456",
            "scope": ["10701093"]
        }));
        assert_eq!(r.access_token().secret(), "access-123");
        assert_eq!(r.expires_in(), Some(Duration::from_secs(3600)));
        assert_eq!(r.refresh_token().map(|t| t.secret().as_str()), Some("refresh-456"));
    }

    #[test]
    fn optional_fields_absent() {
        let r = parse(json!({ "access_token": "a", "token_type": "bearer" }));
        assert!(r.expires_in().is_none());
        assert!(r.refresh_token().is_none());
        assert!(r.scopes().is_none());
    }

    #[test]
    fn access_token_missing_is_rejected() {
        let res = serde_json::from_value::<CivitaiTokenResponse>(json!({ "token_type": "bearer" }));
        assert!(res.is_err());
    }

    #[test]
    fn unknown_fields_are_ignored() {
        let r = parse(json!({
            "access_token": "a",
            "token_type": "bearer",
            "id_token": "eyJ...",
            "user_id": 12345
        }));
        assert_eq!(r.access_token().secret(), "a");
    }

    #[test]
    fn parses_raw_json_string() {
        let body = r#"{"access_token":"a","token_type":"Bearer","expires_in":2592000,"refresh_token":"r","scope":["10701093"]}"#;
        let r: CivitaiTokenResponse = serde_json::from_str(body).unwrap();
        assert_eq!(scope_strs(&r), Some(vec!["10701093"]));
    }

    #[test]
    fn serializes_to_rfc6749_shape() {
        let r = parse(json!({
            "access_token": "a",
            "token_type": "Bearer",
            "expires_in": 60,
            "refresh_token": "r",
            "scope": ["read", "write"]
        }));
        let v = serde_json::to_value(&r).unwrap();
        assert_eq!(v["scope"], json!("read write"));
        assert_eq!(v["token_type"], json!("bearer"));
        assert_eq!(v["expires_in"], json!(60));
        assert_eq!(v["refresh_token"], json!("r"));
    }

    #[test]
    fn serialization_omits_absent_optionals() {
        let r = parse(json!({ "access_token": "a", "token_type": "bearer" }));
        let v = serde_json::to_value(&r).unwrap();
        let obj = v.as_object().unwrap();
        assert!(!obj.contains_key("scope"));
        assert!(!obj.contains_key("expires_in"));
        assert!(!obj.contains_key("refresh_token"));
    }

    #[test]
    fn round_trip() {
        let original = parse(json!({
            "access_token": "a",
            "token_type": "DPoP",
            "expires_in": 10,
            "refresh_token": "r",
            "scope": [10701093]
        }));
        let s = serde_json::to_string(&original).unwrap();
        let back: CivitaiTokenResponse = serde_json::from_str(&s).unwrap();

        assert_eq!(back.access_token().secret(), original.access_token().secret());
        assert_eq!(back.token_type(), original.token_type());
        assert_eq!(back.expires_in(), original.expires_in());
        assert_eq!(
            back.refresh_token().map(|t| t.secret().clone()),
            original.refresh_token().map(|t| t.secret().clone())
        );
        assert_eq!(scope_strs(&back), scope_strs(&original));
    }
}
