//! # Zuora REST Client
//!
//! `zuora_rest_client` is a HTTP Client built on top of the `reqwest` package  for accessing the
//! [Zuora Billing REST API](https://www.zuora.com/developer/api-reference/)
//!
//! This package currently only provides an interface for performing OAuth authenticated GET
//! requests
//!
//! # TODO
//!
//! - Add missing HTTP methods
//! - Add retry logic
//! - Hook up to a CI server
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct AccessToken {
    access_token: String,
    token_type: String,
    expires_in: usize,
    scope: String,
    jti: String,
}

#[derive(Debug)]
pub struct Zuora {
    client_id: String,
    client_secret: String,
    domain: String,
    version: String,
    retry_attempts: usize,
    client: reqwest::Client,
    token: Option<AccessToken>,
}

/// Create an instance of a Zuora client
impl Zuora {
    pub fn new(
        client_id: String,
        client_secret: String,
        domain: String,
        version: String,
        retry_attempts: usize,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            domain,
            version,
            retry_attempts,
            client: reqwest::Client::new(),
            token: None,
        }
    }
    fn endpoint(&self) -> String {
        format!("{}{}", self.domain, self.version)
    }

    fn construct_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        if let Some(token) = &self.token {
            let bearer = format!("Bearer {}", { &token.access_token });
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(bearer.as_str()).unwrap(),
            );
        }

        headers
    }
    /// Generate and set an OAuth token against this instance of the Zuora client to allow bearer
    /// token in subsequent HTTP requests
    ///
    /// # Example
    ///
    /// ```
    /// use std::env;
    /// use zuora_rest_client::Zuora;
    ///
    /// let mut client = Zuora::new(
    ///     env::var("ZUORA_CLIENT_ID").unwrap_or_default(),
    ///     env::var("ZUORA_CLIENT_SECRET").unwrap_or_default(),
    ///     String::from("https://rest.sandbox.eu.zuora.com"),
    ///     String::from("/v1"),
    ///     3,
    /// );
    /// let result = client.generate_token();
    /// assert_eq!(result.unwrap(), ());
    /// ```
    ///
    /// # Errors
    ///
    /// This method may return a `reqwest::Error` where the call to Zuora was not successful
    ///
    #[tokio::main]
    pub async fn generate_token(&mut self) -> Result<(), reqwest::Error> {
        let request_url = format!("{}/oauth/token", self.domain);

        let data = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("grant_type", "client_credentials"),
        ];

        let resp = self.client.post(&request_url).form(&data).send().await;
        match resp {
            Ok(x) if x.status().is_success() => {
                let data = x.text().await.unwrap();
                self.token = Some(serde_json::from_str(&data[..]).unwrap());
                Ok(())
            }
            Ok(_x) => Ok(()),
            Err(err) => Err(err),
        }
    }
    /// Perform GET request on Zuora API
    ///
    /// # Example
    ///
    /// ```
    /// use std::env;
    /// use zuora_rest_client::Zuora;
    ///
    /// let mut client = Zuora::new(
    ///     env::var("ZUORA_CLIENT_ID").unwrap_or_default(),
    ///     env::var("ZUORA_CLIENT_SECRET").unwrap_or_default(),
    ///     String::from("https://rest.sandbox.eu.zuora.com"),
    ///     String::from("/v1"),
    ///     3,
    /// );
    /// let get = client.get("/catalog/products", serde_json::from_str("{}").unwrap());
    /// println!("{:?}", get);
    /// ```
    ///
    /// # Errors
    ///
    /// This method may return a `reqwest::Error` where the call to Zuora was not successful
    ///
    #[tokio::main]
    pub async fn get(
        &self,
        path: &str,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let resp = self
            .client
            .get(self.endpoint() + path)
            .headers(self.construct_headers())
            .form(&payload)
            .send()
            .await;
        match resp {
            Ok(x) => {
                let data = x.text().await.unwrap();
                let value = serde_json::from_str(&data[..]).unwrap();
                Ok(value)
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;
    use mockito::mock;
    fn init() -> Zuora {
        let host = mockito::server_url();
        Zuora::new(
            String::from("client_id"),
            String::from("client_secret"),
            host,
            String::from("/v1"),
            3,
        )
    }
    #[test]
    fn endpoint() {
        let client = init();
        assert_eq!(client.endpoint(), format!("{}/v1", client.domain));
    }

    #[test]
    fn construct_headers_default() {
        let client = init();
        let headers = client.construct_headers();
        assert_eq!(headers.len(), 0);
    }

    #[test]
    fn construct_headers_auth() {
        let mut client = init();
        let token = AccessToken {
            access_token: String::from("access"),
            token_type: String::from("bearer"),
            expires_in: 100,
            scope: String::from("scope"),
            jti: String::from("jti"),
        };
        client.token = Some(token);
        let headers = client.construct_headers();
        assert!(headers.contains_key(AUTHORIZATION));
        assert_eq!(headers[AUTHORIZATION], "Bearer access");
    }

    #[test]
    fn generate_token() {
        let mock_request = mock("POST", "/oauth/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{ 
                "access_token": "access", 
                "token_type": "bearer", 
                "expires_in": 100, 
                "scope": "scope", 
                "jti": "jti" 
            }"#,
            )
            .create();

        let mut client = init();
        let token = AccessToken {
            access_token: String::from("access"),
            token_type: String::from("bearer"),
            expires_in: 100,
            scope: String::from("scope"),
            jti: String::from("jti"),
        };
        let result = client.generate_token();
        assert_eq!(result.unwrap(), ());
        assert_eq!(client.token, Some(token));
        mock_request.assert();
    }
    #[test]
    fn get_success() {
        let client = init();
        let body = r#"{ 
            "products": [], 
            "success": true 
        }"#;
        let mock_request = mock("GET", "/v1/catalog/products")
            .with_status(200)
            .with_body(&body)
            .create();

        let result = client.get("/catalog/products", serde_json::from_str("{}").unwrap());
        let expected: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(result.unwrap(), expected);
        mock_request.assert();
    }
}
