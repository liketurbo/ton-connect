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
use ton_connect::{crypto::ClientKeypair, BridgeMessage, HttpBridge};
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
        let payload =
            base64::decode(&bridge_msg.message).expect("invalid base64 message from bridge");
        let sender_pubkey = hex::decode(&bridge_msg.from).expect("invalid hex sender pubkey");
        client_a.decrypt_message(&payload, &sender_pubkey);
        /*
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
        */

        // {"id":65,"event":"connect","payload":{"items":[{"name":"ton_addr","address":"0:dc69be3a989b1513f14e9e4ecc46ff6753770a2b199138708ff66238c2719e99","network":"-239","publicKey":"321b717a3a455096b6c3d20ad3f66207125557dacbb888fca9e00023aa89bfff","walletStateInit":"te6cckECFgEAAwQAAgE0ARUBFP8A9KQT9LzyyAsCAgEgAxACAUgEBwLm0AHQ0wMhcbCSXwTgItdJwSCSXwTgAtMfIYIQcGx1Z70ighBkc3RyvbCSXwXgA/pAMCD6RAHIygfL/8nQ7UTQgQFA1yH0BDBcgQEI9ApvoTGzkl8H4AXTP8glghBwbHVnupI4MOMNA4IQZHN0crqSXwbjDQUGAHgB+gD0BDD4J28iMFAKoSG+8uBQghBwbHVngx6xcIAYUATLBSbPFlj6Ahn0AMtpF8sfUmDLPyDJgED7AAYAilAEgQEI9Fkw7UTQgQFA1yDIAc8W9ADJ7VQBcrCOI4IQZHN0coMesXCAGFAFywVQA88WI/oCE8tqyx/LP8mAQPsAkl8D4gIBIAgPAgEgCQ4CAVgKCwA9sp37UTQgQFA1yH0BDACyMoHy//J0AGBAQj0Cm+hMYAIBIAwNABmtznaiaEAga5Drhf/AABmvHfaiaEAQa5DrhY/AABG4yX7UTQ1wsfgAWb0kK29qJoQICga5D6AhhHDUCAhHpJN9KZEM5pA+n/mDeBKAG3gQFImHFZ8xhAT48oMI1xgg0x/TH9MfAvgju/Jk7UTQ0x/TH9P/9ATRUUO68qFRUbryogX5AVQQZPkQ8qP4ACSkyMsfUkDLH1Iwy/9SEPQAye1U+A8B0wchwACfbFGTINdKltMH1AL7AOgw4CHAAeMAIcAC4wABwAORMOMNA6TIyx8Syx/L/xESExQAbtIH+gDU1CL5AAXIygcVy//J0Hd0gBjIywXLAiLPFlAF+gIUy2sSzMzJc/sAyEAUgQEI9FHypwIAcIEBCNcY+gDTP8hUIEeBAQj0UfKnghBub3RlcHSAGMjLBcsCUAbPFlAE+gIUy2oSyx/LP8lz+wACAGyBAQjXGPoA0z8wUiSBAQj0WfKnghBkc3RycHSAGMjLBcsCUAXPFlAD+gITy2rLHxLLP8lz+wAACvQAye1UAFEAAAAAKamjFzIbcXo6RVCWtsPSCtP2YgcSVVfay7iI/KngACOqib//QMgBKq8="}],"device":{"platform":"iphone","appName":"Tonkeeper","appVersion":"3.0.304","maxProtocolVersion":2,"features":["SendTransaction",{"name":"SendTransaction","maxMessages":4}]}}}
    });

    println!("{:?}", res);
}
