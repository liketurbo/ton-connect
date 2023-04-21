use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TonAddressItem {
    name: String,
}

impl Default for TonAddressItem {
    fn default() -> Self {
        Self {
            name: "ton_addr".to_string(),
        }
    }
}

impl TonAddressItem {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize)]
pub struct TonProofItem {
    name: String,
    payload: String,
}

impl Default for TonProofItem {
    fn default() -> Self {
        Self {
            name: "ton_proof".to_string(),
            payload: "".to_string(),
        }
    }
}

impl TonProofItem {
    pub fn new(payload: String) -> Self {
        Self {
            name: "ton_proof".to_string(),
            payload,
        }
    }
}
