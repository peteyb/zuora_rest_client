# zuora_rest_client

[![crates.io](https://img.shields.io/crates/v/zuora_rest_client.svg)](https://crates.io/crates/zuora_rest_client)
[![Documentation](https://docs.rs/zuora_rest_client/badge.svg)](https://docs.rs/zuora_rest_client)
[![MIT licensed](https://img.shields.io/crates/l/zuora_rest_client.svg)](./LICENSE)

A HTTP Client built on top of the [reqwest](https://crates.io/crates/reqwest) package for accessing the [Zuora Billing REST API](https://www.zuora.com/developer/api-reference/)

## TODOs

This package currently only provides an interface for performing OAuth authenticated GET requests

- Add missing HTTP methods
- Add retry logic
- Hook up to a CI server

## Example

This example uses [serde_json](https://crates.io/crates/serde_json) to prepare the data for a GET request. Your `Cargo.toml` could look like this:

```toml
[dependencies]
zuora_rest_client = "0.1"
serde_json = "1"
```

And then the code:

```rust
use std::env;
use zuora_rest_client::Zuora;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Zuora::new(
        env::var("ZUORA_CLIENT_ID").unwrap_or_default(),
        env::var("ZUORA_CLIENT_SECRET").unwrap_or_default(),
        String::from("https://rest.sandbox.eu.zuora.com"),
        String::from("/v1"),
        3,
    );

    let result = client.generate_token();
    println!("{:?}", result);

    let get = client.get("/catalog/products", serde_json::from_str("{}").unwrap());
    println!("{:?}", get);
    Ok(())
}
```

## License

Licensed under

- MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)
