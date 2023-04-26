extern crate ton_connect;

use ton_connect::{ClientKeypair, Event, HttpBridge};
use ton_connect::{ConnectItem, ConnectRequest, TonAddressItem, TonConnect};

fn main() {
    let tonconnect = TonConnect::new("https://app.tonkeeper.com/ton-connect".to_string());
    let init_request = ConnectRequest::new(
        "https://raw.githubusercontent.com/XaBbl4/pytonconnect/main/pytonconnect-manifest.json"
            .to_string(),
        vec![ConnectItem::TonAddress(TonAddressItem::new())],
    );
    let client_a = ClientKeypair::random();
    let universal_link = tonconnect
        .create_connect_link(client_a.public, init_request)
        .unwrap();

    println!("{}", universal_link);

    let mut bridge = HttpBridge::new("https://bridge.tonapi.io/bridge");
    let client_ids = vec![client_a.public];
    let topics = None;
    bridge.subscribe(
        &client_ids,
        topics,
        |res: Result<Event, Box<dyn std::error::Error>>| {
            println!("{:?}", res.unwrap());
        },
    );
}
