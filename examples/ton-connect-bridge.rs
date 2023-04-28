extern crate base64;
extern crate crypto_box;
extern crate generic_array;
extern crate salsa20;
extern crate serde_json;
extern crate ton_connect;

use crypto_box::{
    aead::{Aead, AeadCore, OsRng},
    PublicKey, SalsaBox, SecretKey,
};
use generic_array::{typenum::U24, GenericArray};
use std::convert::TryInto;
use ton_connect::{BridgeMessage, ClientKeypair, HttpBridge};
use ton_connect::{ConnectItem, ConnectRequest, TonAddressItem, TonConnect};

const NONCE_LENGTH: usize = 24;

fn main() {
    let tonconnect = TonConnect::new("https://app.tonkeeper.com/ton-connect".to_string());
    let init_request = ConnectRequest::new(
        "https://raw.githubusercontent.com/XaBbl4/pytonconnect/main/pytonconnect-manifest.json"
            .to_string(),
        vec![ConnectItem::TonAddress(TonAddressItem::new())],
    );
    let client_a = ClientKeypair::random();
    let client_a_pub = client_a.public.clone();
    let universal_link = tonconnect
        .create_connect_link(&client_a_pub, init_request)
        .unwrap();

    println!("{}", universal_link);

    let mut bridge = HttpBridge::new("https://bridge.tonapi.io/bridge");
    let client_ids = vec![&client_a_pub];
    let topics = None;
    let res = bridge.subscribe(&client_ids, &topics, |bridge_msg: BridgeMessage| {
        let bytes = base64::decode(&bridge_msg.message).unwrap();
        let wallet_public_key_vec = hex::decode(&bridge_msg.from).unwrap();
        let wallet_public_key_array: [u8; 32] =
            wallet_public_key_vec.as_slice().try_into().unwrap();
        let wallet_public_key = PublicKey::from(wallet_public_key_array);
        let my_box: SalsaBox = client_a.get_box(&wallet_public_key);
        let nonce_bytes: &[u8; 24] = &bytes[..NONCE_LENGTH].try_into().unwrap();
        let nonce: GenericArray<u8, U24> = GenericArray::clone_from_slice(nonce_bytes);
        let ciphertext = &bytes[NONCE_LENGTH..];
        let decrypted = my_box.decrypt(&nonce, ciphertext).unwrap();
        let decrypted_str = String::from_utf8(decrypted).unwrap();
        println!("{}", decrypted_str);
    });

    println!("{:?}", res);
}
