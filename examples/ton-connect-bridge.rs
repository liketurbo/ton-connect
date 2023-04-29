extern crate ton_connect;

use ton_connect::{
    crypto::ClientKeypair,
    types::{BridgeMessage, ConnectItem, ConnectRequest, Topic},
    HttpBridge, TonConnect,
};

fn main() {
    let tonconnect = TonConnect::new("https://app.tonkeeper.com/ton-connect".to_string());
    let init_request = ConnectRequest {
        manifest_url:
            "https://raw.githubusercontent.com/XaBbl4/pytonconnect/main/pytonconnect-manifest.json"
                .to_string(),
        items: vec![ConnectItem::TonAddressItem],
    };
    let client_a = ClientKeypair::random();
    let client_a_pub = client_a.public.clone();
    let universal_link = tonconnect
        .create_connect_link(&client_a_pub, &init_request)
        .unwrap();

    println!("{}", universal_link);

    let mut bridge = HttpBridge::new("https://bridge.tonapi.io/bridge");
    let client_ids = vec![&client_a_pub];
    let topics = Some(vec![Topic::SendTransaction]);
    let _ = bridge.listen(&client_ids, &topics, |bridge_msg: BridgeMessage| {
        let res = client_a.decrypt_message(bridge_msg).unwrap();
        println!("{:?}", res);
    });
}
