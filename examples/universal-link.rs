extern crate ton_connect;

use ton_connect::{
    crypto::ClientKeypair,
    types::{ConnectItem, ConnectRequest},
    TonConnect,
};

fn main() {
    let tonconnect = TonConnect::new("https://app.tonkeeper.com/ton-connect".to_string());

    let init_request = ConnectRequest {
        manifest_url:
            "https://raw.githubusercontent.com/XaBbl4/pytonconnect/main/pytonconnect-manifest.json"
                .to_string(),
        items: vec![
            ConnectItem::TonAddressItem,
            ConnectItem::TonProofItem {
                payload: "some_payload".to_string(),
            },
        ],
    };

    let client_a = ClientKeypair::random();
    let universal_link = tonconnect
        .create_connect_link(&client_a.public, &init_request)
        .unwrap();

    println!("{}", universal_link);
}
