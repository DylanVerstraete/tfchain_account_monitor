use sp_core::crypto::Pair;
use tfchain_client::{AccountId32, Client};
use std::env;
use telegram_bot::Api;
extern crate tokio;
use tokio::time;
use std::time::Duration;
use serde_yaml;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Network {
    network_url: String,
    accounts: Vec<Account>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Account {
    name: String,
    address: String,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        panic!("config file not specified, please specify a valid config file.")
    }

    let config_file = std::fs::read(args[1].clone()).unwrap();
    let networks: Vec<Network> = serde_yaml::from_str(String::from_utf8(config_file).unwrap().as_str()).unwrap();

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);
    let chat_id = -501732733;
    let chat = telegram_bot::ChatId::new(chat_id);

    let req = telegram_bot::requests::SendMessage::new(chat, String::from("Bot started"));
    let _ = api.send(req).await;

    for network in networks.iter().cloned() {
        let api = api.clone();
        tokio::task::spawn(async move {
            let key: (sp_core::sr25519::Pair, _) = Pair::generate();
            let client = Client::new(network.network_url.clone(), key.0.clone());

            let mut interval = time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                for acc in &network.accounts {
                    let b = client
                        .get_account_free_balance(&acc.address.parse::<AccountId32>().unwrap());

                    let balance = b.unwrap();
                    let msg = format!(
                        "\nnetwork: {} \nbalance of account {}: {} \nAddress: {}",
                        network.network_url, acc.name, balance.free, acc.address
                    );
                    println!("{}", msg);

                    if balance.free < 100000000 {
                        println!("should notify telegram");
                        let req = telegram_bot::requests::SendMessage::new(chat, msg);
                        let res = api.send(req).await;
                        println!("message pushed to telegram: {:?}", res);
                    }
                }
            }
        });
    }

    tokio::signal::ctrl_c().await.unwrap();
}
