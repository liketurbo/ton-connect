use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectRequest {
    #[serde(rename = "manifestUrl")]
    pub manifest_url: String,
    pub items: Vec<ConnectItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum ConnectItem {
    #[serde(rename = "ton_addr")]
    TonAddressItem,
    #[serde(rename = "ton_proof")]
    TonProofItem { payload: String },
}

#[derive(Debug, Serialize, Deserialize)]
struct TonAddressItem {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TonProofItem {
    name: String,
    payload: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BridgeMessage {
    pub from: String,
    pub message: String,
}
