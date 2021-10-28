use std::env;
use std::process;
use zuora_rest_client::Zuora;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Not enough arguments");
        process::exit(1);
    }

    let client = Zuora::new(
        args[1].clone(),
        args[2].clone(),
        args[3].clone(),
        args[4].clone(),
        3,
    );
    println!("{:?}", client);
}
