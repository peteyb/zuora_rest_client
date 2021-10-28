use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client, ClientBuilder,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::future::ready;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    access_token: String,
    token_type: String,
    expires_in: usize,
    scope: String,
    jti: String,
}

#[derive(Debug)]
pub struct Zuora {
    client: reqwest::Client,
    client_id: String,
    client_secret: String,
    domain: String,
    version: String,
    retry_attempts: usize,
}

impl Zuora {
    pub fn new(
        client_id: String,
        client_secret: String,
        domain: String,
        version: String,
        retry_attempts: usize,
    ) -> Self {
        Self {
            client: reqwest::Client::new(),
            client_id,
            client_secret,
            domain,
            version,
            retry_attempts,
        }
    }
    fn endpoint(&self) -> String {
        format!("{}{}", self.domain, self.version)
    }
    #[tokio::main]
    pub async fn generate_token(&self) -> Result<AccessToken, reqwest::Error> {
        let request_url = format!("{}/oauth/token", self.domain);

        let data = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("grant_type", "client_credentials"),
        ];

        let resp = self.client.post(&request_url).form(&data).send().await;
        match resp {
            Ok(x) => {
                let data = x.text().await.unwrap();
                let token: AccessToken = serde_json::from_str(&data[..]).unwrap();
                Ok(token)
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn client_endpoint() {
        let client = Zuora::new(
            String::from("client_id"),
            String::from("client_secret"),
            String::from("https://rest.sandbox.eu.zuora.com"),
            String::from("/v1"),
            3,
        );
        assert_eq!(client.endpoint(), "https://rest.sandbox.eu.zuora.com/v1");
    }
}
