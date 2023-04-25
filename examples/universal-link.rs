extern crate ton_connect;

use ton_connect::{ConnectItem, ConnectRequest, TonAddressItem, Tonconnect};

fn main() {
    let tonconnect = Tonconnect::new();
    let init_request = ConnectRequest::new(
        "https://raw.githubusercontent.com/XaBbl4/pytonconnect/main/pytonconnect-manifest.json"
            .to_string(),
        vec![ConnectItem::TonAddress(TonAddressItem::new())],
    );
    let universal_link = tonconnect
        .create_universal_link(
            "https://app.tonkeeper.com/ton-connect".to_string(),
            init_request,
        )
        .unwrap();

    println!("{}", universal_link);
}
