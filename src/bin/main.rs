use std::env;
use std::process;
use zuora_rest_client::Zuora;
use zuora_rest_client::ZuoraSubscriptionResponse;
use zuora_rest_client::ZuoraTrait;

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

    // let get = client.get("/catalog/products", serde_json::from_str("{}").unwrap());
    // println!("{:?}", get);

    let account_id = String::from("8adc8f9968a1c7d60168a4a88b4d6e9f");
    let zoql = format!(
        "SELECT Id, Name, Version from Subscription where AccountId = '{}'",
        account_id
    );
    let query: ZuoraSubscriptionResponse = client.query(&zoql).unwrap();
    // println!("{:?}", query);
    // let mut subscriptions: Vec<serde_json::Value> = Vec::new();
    for record in query.records {
        println!("{:?}", record);
    }

    Ok(())
}
