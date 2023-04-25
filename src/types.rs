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

#[derive(Debug, Serialize, Deserialize)]
enum Platform {
    #[serde(rename = "iphone")]
    Iphone,
    #[serde(rename = "ipad")]
    Ipad,
    #[serde(rename = "android")]
    Android,
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "mac")]
    Mac,
    #[serde(rename = "linux")]
    Linux,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    platform: Platform,
    app_name: String,
    app_version: String,
    max_protocol_version: u32,
    features: Vec<Feature>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "name")]
enum Feature {
    #[serde(rename = "SendTransaction")]
    SendTransaction { max_messages: u32 },
    #[serde(rename = "SignData")]
    SignData,
}

pub struct WalletInfo {
    pub name: String,
    pub image: String,
    pub tondns: Option<String>,
    pub about_url: String,
}
