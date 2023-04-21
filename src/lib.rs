extern crate chacha20poly1305;
extern crate hex;
extern crate percent_encoding;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate x25519_dalek;

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    ChaCha20Poly1305, Nonce, XChaCha20Poly1305,
};
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use x25519_dalek::{EphemeralSecret, PublicKey};

pub mod types;
pub use types::{TonAddressItem, TonProofItem};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConnectItem {
    TonAddress(TonAddressItem),
    TonProof(TonProofItem),
}

#[derive(Serialize, Deserialize)]
pub struct InitRequest {
    #[serde(rename = "manifestUrl")]
    pub manifest_url: String,
    pub items: Vec<ConnectItem>,
}

impl InitRequest {
    pub fn new(manifest_url: String, items: Vec<ConnectItem>) -> Self {
        Self {
            manifest_url,
            items,
        }
    }
}

pub struct Tonconnect {
    secret: EphemeralSecret,
    public: PublicKey,
}

impl Tonconnect {
    pub fn new() -> Self {
        let secret = EphemeralSecret::new(OsRng);
        let public = PublicKey::from(&secret);

        Self { secret, public }
    }

    pub fn create_universal_link(
        &self,
        universal_url: String,
        init_request: InitRequest,
    ) -> Result<String, ()> {
        let hex_public = hex::encode(self.public.as_bytes());
        let init_request = serde_json::to_string(&init_request).unwrap();
        let init_request = utf8_percent_encode(&init_request, NON_ALPHANUMERIC);
        let link = format!("{}?v=2&id={}&r={}", universal_url, hex_public, init_request);
        Ok(link)
    }
}
