use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Context};
use axum::Extension;
use jwt::Token;
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::random_string;

pub type GoogleOauthExtension = Extension<Arc<GoogleOauthClient>>;

pub struct GoogleOauthClient {
    http: reqwest::Client,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    scopes: String,
    states: Mutex<Vec<String>>,
    user_agent: String,
}

impl GoogleOauthClient {
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        scopes: &[&str],
        user_agent: String,
    ) -> GoogleOauthClient {
        GoogleOauthClient {
            http: reqwest::Client::new(),
            client_id,
            client_secret,
            redirect_uri,
            user_agent,
            scopes: urlencoding::encode(&scopes.join(" ")).to_string(),
            states: Default::default(),
        }
    }

    pub fn get_login_redirect_url(&self) -> String {
        let state = random_string(40);
        let redirect_url = format!(
            "https://accounts.google.com/o/oauth2/v2/auth?response_type=code&state={state}&client_id={}&redirect_uri={}&scope={}",
            self.client_id, self.redirect_uri, self.scopes
        );
        {
            let mut states = self.states.lock().unwrap();
            states.push(state);
            // println!("{states:#?}");
        }
        redirect_url
    }

    pub fn check_state(&self, state: &str) -> bool {
        let mut contained = false;
        self.states.lock().unwrap().retain(|s| {
            if s.as_str() == state {
                contained = true;
                false
            } else {
                true
            }
        });
        contained
    }

    pub async fn get_access_token(&self, code: &str) -> anyhow::Result<AccessTokenResponse> {
        self.post::<(), _>(
            "https://oauth2.googleapis.com/token",
            &[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("redirect_uri", self.redirect_uri.as_str()),
                ("code", code),
                ("grant_type", "authorization_code"),
            ],
            None,
        )
        .await
        .context("failed to get google access token using code")
    }

    pub fn get_google_user(&self, id_token: &str) -> anyhow::Result<GoogleUser> {
        let t: Token<Value, GoogleUser, jwt::Unverified> =
            Token::parse_unverified(id_token).context("failed to parse id_token")?;
        Ok(t.claims().to_owned())
    }

    async fn post<B: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &[(&str, &str)],
        bearer_token: Option<&str>,
    ) -> anyhow::Result<R> {
        let mut req = self
            .http
            .post(endpoint)
            .form(body)
            .header("Accept", "application/json")
            .header("User-Agent", &self.user_agent);

        if let Some(bearer_token) = bearer_token {
            req = req.header("Authorization", format!("Bearer {bearer_token}"));
        }

        let res = req.send().await.context("failed to reach google")?;

        let status = res.status();

        if status == StatusCode::OK {
            let body = res
                .json()
                .await
                .context("failed to parse POST body into expected type")?;
            Ok(body)
        } else {
            let text = res.text().await.context(format!(
                "method: POST | status: {status} | failed to get response text"
            ))?;
            Err(anyhow!("method: POST | status: {status} | text: {text}"))
        }
    }
}

#[derive(Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub id_token: String,
    pub scope: String,
    pub token_type: String,
}

#[derive(Deserialize, Clone)]
pub struct GoogleUser {
    #[serde(rename = "sub")]
    pub id: String,
    pub email: String,
    pub picture: String,
}
