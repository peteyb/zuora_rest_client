// use std::collections::HashMap;
use std::env;
use std::process;
use zuora_rest_client::Zuora;

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments");
        process::exit(1);
    }

    let client = Zuora::new(
        env::var("ZUORA_CLIENT_ID").unwrap_or_default(),
        env::var("ZUORA_CLIENT_SECRET").unwrap_or_default(),
        args[1].clone(),
        args[2].clone(),
        3,
    );
    // println!("{:?}", client);

    let token = client.generate_token();
    println!("{:?}", token);

    // let resp = reqwest::get("https://httpbin.org/ip")
    //     .await?
    //     .json::<HashMap<String, String>>()
    //     .await?;
    // println!("{:#?}", resp);
    Ok(())
}
