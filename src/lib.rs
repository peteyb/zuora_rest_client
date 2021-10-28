#[derive(Debug)]
pub struct Zuora {
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
    ) -> Zuora {
        Zuora {
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
