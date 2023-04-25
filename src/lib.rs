extern crate chacha20poly1305;
extern crate hex;
extern crate percent_encoding;
extern crate rand;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate x25519_dalek;

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    ChaCha20Poly1305, Nonce, XChaCha20Poly1305,
};
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use rand::rngs::OsRng;
use reqwest::{
    blocking::{Request, Response},
    header::{HeaderMap, HeaderValue},
    Url,
};
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader, Read},
    net::{TcpStream, ToSocketAddrs},
};
use types::{DeviceInfo, WalletInfo};
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
pub struct ConnectRequest {
    #[serde(rename = "manifestUrl")]
    pub manifest_url: String,
    pub items: Vec<ConnectItem>,
}

impl ConnectRequest {
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
        connect_request: ConnectRequest,
    ) -> Result<String, ()> {
        let hex_public = hex::encode(self.public.as_bytes());
        let init_request = serde_json::to_string(&connect_request).unwrap();
        let init_request = utf8_percent_encode(&init_request, NON_ALPHANUMERIC);
        let link = format!("{}?v=2&id={}&r={}", universal_url, hex_public, init_request);
        Ok(link)
    }
}

struct TonConnectWallet {
    name: String,
    image: String,
    tondns: Option<String>,
    about_url: String,
    universal_url: String,
    sse_bridge_url: String,
}

pub struct ClientKeypair {
    pub public: PublicKey,
    secret: EphemeralSecret,
}

impl ClientKeypair {
    pub fn random() -> Self {
        let secret = EphemeralSecret::new(OsRng);
        let public = PublicKey::from(&secret);

        Self { secret, public }
    }
}

pub enum Topic {
    SendTransaction,
    SignData,
}

pub struct HttpBridge {
    /*
    device_info: DeviceInfo,         // Can be possible multiple sessions
    wallet_info: Option<WalletInfo>, // Also can be multiple wallets
    protocol_version: u32,           // Exclude cause i'm only planning to use version 2
    is_wallet_browser: bool,         // Probably this is not needed too
    */
    url: Url,
    client: reqwest::blocking::Client,
    keypairs: Vec<ClientKeypair>,
    topics: Vec<Topic>,
    last_event_id: Option<String>,
}

impl HttpBridge {
    pub fn new(url: &str) -> Self {
        let url = Url::parse(&url).unwrap();
        let client = reqwest::blocking::Client::new();
        let keypairs = Vec::new();
        let topics = Vec::new();
        let last_event_id = None;
        Self {
            url,
            client,
            keypairs,
            topics,
            last_event_id,
        }
    }

    pub fn set_listen_clients(&mut self, keypairs: Vec<ClientKeypair>) {
        self.keypairs = keypairs;
    }

    pub fn set_listen_topics(&mut self, topics: Vec<Topic>) {
        self.topics = topics;
    }

    pub fn listen(&mut self, handler: impl Fn() -> ()) {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert("Accept", "text/event-stream".parse().unwrap());
        if let Some(ref last_event_id) = &self.last_event_id {
            headers.insert(
                "Last-Event-ID",
                HeaderValue::from_str(last_event_id).unwrap(),
            );
        }

        let mut url = self.url.clone();

        url.path_segments_mut().unwrap().push("events");

        if !self.keypairs.is_empty() {
            let values = self
                .keypairs
                .iter()
                .map(|keypair| hex::encode(keypair.public.as_bytes()))
                .collect::<Vec<String>>()
                .join(",");
            url.query_pairs_mut().append_pair("client_id", &values);
        }

        if !self.topics.is_empty() {
            let values = self
                .topics
                .iter()
                .map(|topic| match topic {
                    Topic::SendTransaction => "sendTransaction",
                    Topic::SignData => "signData",
                })
                .collect::<Vec<&str>>()
                .join(",");
            url.query_pairs_mut().append_pair("topic", &values);
        }

        println!("{}", url);

        let res = self.client.get(url).headers(headers).send().unwrap();
        println!("{}", res.status());
        let mut stream = BufReader::new(res);
        let mut line = String::new();
        loop {
            line.clear();
            stream.read_line(&mut line).unwrap();
            if line.is_empty() {
                break;
            }
            println!("{}", line);
        }
    }

    /*
    pub fn restore_connection() {}
    pub fn send() {}
    pub fn listen() {}
    */
}
