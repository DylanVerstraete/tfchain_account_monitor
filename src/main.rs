use sp_core::crypto::{AccountId32, Pair};
use std::env;
use telegram_bot::Api;
use tfchain_client::client;
extern crate tokio;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::time::Duration;
use tokio::time;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Network {
    network_url: String,
    accounts: Vec<Account>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Account {
    name: String,
    address: String,
    threshold: i64,
}

const PRECISION: i64 = 10000000;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        panic!("config file not specified, please specify a valid config file.")
    }

    let config_file = std::fs::read(args[1].clone()).unwrap();
    let networks: Vec<Network> =
        serde_yaml::from_str(String::from_utf8(config_file).unwrap().as_str()).unwrap();

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);
    // let chat_id = -501732733;
    let chat_id = env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_BOT_TOKEN not set");
    let id: i64 = chat_id.parse().unwrap();
    let chat = telegram_bot::ChatId::new(id);

    let req = telegram_bot::requests::SendMessage::new(chat, String::from("Bot started"));
    let _ = api.send(req).await;

    for network in networks.iter().cloned() {
        let api = api.clone();
        tokio::task::spawn(async move {
            let key: (sp_core::sr25519::Pair, _) = Pair::generate();
            let mut interval = time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                for acc in &network.accounts {
                    let client = client::TfchainClient::new(
                        network.network_url.clone(),
                        key.0.clone(),
                        "mainnet",
                    )
                    .await
                    .unwrap();
                    // let client = Client::new(network.network_url.clone(), key.0.clone());
                    let b = client
                        .get_balance(acc.address.parse::<AccountId32>().unwrap())
                        .await
                        .unwrap();

                    match b {
                        Some(balance) => {
                            let msg = format!(
                                "\nnetwork: {} \nbalance of account {}: {} TFT \nAddress: {}",
                                network.network_url,
                                acc.name,
                                balance.data.free / PRECISION as u128,
                                acc.address
                            );
                            println!("{}", msg);

                            if balance.data.free < (acc.threshold * PRECISION) as u128 {
                                let req = telegram_bot::requests::SendMessage::new(chat, msg);
                                let _ = api.send(req).await;
                                println!("message pushed to telegram bot");
                            }
                        }
                        None => {
                            println!(
                                "something went wrong requestin balance from: {:}",
                                acc.address
                            );
                        }
                    }
                }
            }
        });
    }

    tokio::signal::ctrl_c().await.unwrap();
}
