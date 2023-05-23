extern crate ton_connect;

use ton_connect::{
    base64,
    crypto::{ClientKeypair, NONCE_LENGTH},
    types::{BridgeMessage, Topic, WalletEvent},
    HttpBridge,
};

fn main() {
    let client_a = ClientKeypair::generate_random_keypair();
    let client_a_pub = client_a.public.clone();

    let mut bridge = HttpBridge::new("https://bridge.tonapi.io/bridge");
    let client_ids = vec![&client_a_pub];
    let topics = Some(vec![Topic::SendTransaction]);
    let _ = bridge.listen(&client_ids, &topics, |bridge_msg: BridgeMessage| {
        let msg = base64::decode(bridge_msg.message).expect("invalid base64 message from bridge");
        let (nonce, ciphertext) = msg.split_at(NONCE_LENGTH);
        let plaintext = client_a
            .decrypt_message(&ciphertext, &nonce, &bridge_msg.from)
            .unwrap();
        let wallet_event: WalletEvent = serde_json::from_str(&plaintext).unwrap();
        println!("{:?}", wallet_event);
    });
}
