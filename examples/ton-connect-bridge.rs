extern crate eventsource;
extern crate reqwest;
extern crate ton_connect;

use eventsource::reqwest::Client;
use reqwest::Url;
use ton_connect::{
    base64::{engine::general_purpose, Engine as _},
    crypto::{decrypt_message, ClientKeypair, NONCE_LENGTH},
    helpers::{create_listen_url, create_universal_link},
    types::{BridgeMessage, ConnectItem, ConnectRequest, Topic, WalletEvent},
};

fn main() {
    let client_a = ClientKeypair::generate_random();
    let connect_request = ConnectRequest {
        manifest_url:
            "https://raw.githubusercontent.com/XaBbl4/pytonconnect/main/pytonconnect-manifest.json"
                .to_string(),
        items: vec![
            ConnectItem::TonAddressItem,
            ConnectItem::TonProofItem {
                payload: "halo".to_string(),
            },
        ],
    };
    let wallet_universal_url = "https://app.tonkeeper.com/ton-connect".to_string();
    let link = create_universal_link(
        &wallet_universal_url,
        &client_a.get_hex_public(),
        &connect_request,
    )
    .unwrap();
    println!("Connect link: {}", link);

    let bridge_url = "https://bridge.tonapi.io/bridge";
    let clients_ids = vec![client_a.get_hex_public()];
    let topics = Some(vec![Topic::SendTransaction]);
    let listen_url = create_listen_url(&bridge_url, &clients_ids, &topics).unwrap();
    let client = Client::new(Url::parse(&listen_url).unwrap());
    for event in client {
        let event = event.unwrap();
        if event.id.is_some() {
            let bridge_msg: BridgeMessage = serde_json::from_str(&event.data).unwrap();
            let msg = &general_purpose::STANDARD
                .decode(bridge_msg.message)
                .expect("invalid base64 message from bridge");
            let (nonce, ciphertext) = msg.split_at(NONCE_LENGTH);
            let plaintext =
                decrypt_message(&client_a.secret, &ciphertext, &nonce, &bridge_msg.from).unwrap();
            let wallet_event: WalletEvent = serde_json::from_str(&plaintext).unwrap();
            println!("{}", serde_json::to_string_pretty(&wallet_event).unwrap());
        }
    }
}
