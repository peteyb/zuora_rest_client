# zuora_rest_client

[![crates.io](https://img.shields.io/crates/v/zuora_rest_client.svg)](https://crates.io/crates/zuora_rest_client)
[![Documentation](https://docs.rs/zuora_rest_client/badge.svg)](https://docs.rs/zuora_rest_client)
[![MIT licensed](https://img.shields.io/crates/l/zuora_rest_client.svg)](./LICENSE)

A HTTP Client built on top of the [reqwest](https://crates.io/crates/reqwest) package for accessing the [Zuora Billing REST API](https://www.zuora.com/developer/api-reference/)

## Example

This example uses [serde_json](https://crates.io/crates/serde_json) to prepare the data for a GET request, so your `Cargo.toml` could look like this:

```toml
[dependencies]
zuora_rest_client = "0.1.0"
serde_json = "1"
```

And then the code:

```rust
use std::env;
use std::process;
use zuora_rest_client::Zuora;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments");
        process::exit(1);
    }

    let mut client = Zuora::new(
        env::var("ZUORA_CLIENT_ID").unwrap_or_default(),
        env::var("ZUORA_CLIENT_SECRET").unwrap_or_default(),
        args[1].clone(),
        args[2].clone(),
        3,
    );

    let token = client.generate_token();
    println!("{:?}", token);

    let get = client.get("/catalog/products", serde_json::from_str("{}").unwrap());
    println!("{:?}", get);
    Ok(())
}
```

## License

Licensed under

- MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)
