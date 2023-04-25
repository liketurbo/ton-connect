extern crate ton_connect;

use ton_connect::{ClientKeypair, HttpBridge};

fn main() {
    let mut bridge = HttpBridge::new("https://bridge.tonapi.io/bridge");
    let client_a = ClientKeypair::random();
    bridge.set_listen_clients(vec![client_a]);
    bridge.listen(|| {
        println!("New event");
    });
}
