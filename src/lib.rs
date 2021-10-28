use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
    use mockito;
    use mockito::mock;
    #[test]
    fn endpoint() {
        let client = Zuora::new(
            String::from("client_id"),
            String::from("client_secret"),
            String::from("https://rest.sandbox.eu.zuora.com"),
            String::from("/v1"),
            3,
        );
        assert_eq!(client.endpoint(), "https://rest.sandbox.eu.zuora.com/v1");
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

        let host = mockito::server_url();
        let client = Zuora::new(
            String::from("client_id"),
            String::from("client_secret"),
            host,
            String::from("/v1"),
            3,
        );
        let token = AccessToken {
            access_token: String::from("access"),
            token_type: String::from("bearer"),
            expires_in: 100,
            scope: String::from("scope"),
            jti: String::from("jti"),
        };
        assert_eq!(client.generate_token().unwrap(), token);
        mock_request.assert();
    }
}
