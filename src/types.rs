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

#[derive(Debug, Serialize, Deserialize)]
pub enum Topic {
    #[serde(rename = "sendTransaction")]
    SendTransaction,
    #[serde(rename = "signData")]
    SignData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BridgeMessage {
    pub from: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum WalletEvent {
    #[serde(rename = "connect")]
    Connect { id: u32, payload: ConnectPayload },
    #[serde(rename = "connect_error")]
    ConnectError {
        id: u32,
        payload: ConnectErrorPayload,
    },
    #[serde(rename = "disconnect")]
    Disconnect { id: u32 },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectPayload {
    items: Vec<ConnectItemReply>,
    device: DeviceInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectErrorPayload {
    code: u32,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "name")]
enum ConnectItemReply {
    #[serde(rename = "ton_addr")]
    TonAddress {
        address: String,
        network: NETWORK,
        #[serde(rename = "publicKey")]
        public_key: String,
        #[serde(rename = "walletStateInit")]
        wallet_state_init: String,
    },
    #[serde(rename = "ton_proof")]
    TonProof { proof: TonProofItemReply },
}

#[derive(Debug, Serialize, Deserialize)]
enum NETWORK {
    #[serde(rename = "-239")]
    MAINNET,
    #[serde(rename = "-3")]
    TESTNET,
}

#[derive(Debug, Serialize, Deserialize)]
struct TonProofItemReply {
    timestamp: String,
    domain: TonProofDomain,
    signature: String,
    payload: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TonProofDomain {
    #[serde(rename = "lengthBytes")]
    length_bytes: u32,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceInfo {
    platform: Platform,
    #[serde(rename = "appName")]
    app_name: String,
    #[serde(rename = "appVersion")]
    app_version: String,
    #[serde(rename = "maxProtocolVersion")]
    max_protocol_version: u32,
    features: Vec<Feature>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "name")]
enum Feature {
    SendTransaction {
        #[serde(rename = "maxMessages")]
        max_messages: u32,
    },
    SignData,
}

#[derive(Debug, Serialize, Deserialize)]
enum Platform {
    #[serde(rename = "iphone")]
    IPhone,
    #[serde(rename = "ipad")]
    IPad,
    #[serde(rename = "android")]
    Android,
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "mac")]
    Mac,
    #[serde(rename = "linux")]
    Linux,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_link() {
        let init_request = ConnectRequest {
            manifest_url:
                "https://raw.githubusercontent.com/XaBbl4/pytonconnect/main/pytonconnect-manifest.json"
                    .to_string(),
            items: vec![
                ConnectItem::TonAddressItem,
                ConnectItem::TonProofItem {
                    payload: "some_payload".to_string(),
                },
            ],
        };
        let serialized = serde_json::to_string(&init_request).unwrap();
        let expected_output = "{\"manifestUrl\":\"https://raw.githubusercontent.com/XaBbl4/pytonconnect/main/pytonconnect-manifest.json\",\"items\":[{\"name\":\"ton_addr\"},{\"name\":\"ton_proof\",\"payload\":\"some_payload\"}]}";
        assert_eq!(serialized, expected_output);
    }

    #[test]
    fn deserialize_event() {
        let input = "{\"id\":65,\"event\":\"connect\",\"payload\":{\"items\":[{\"name\":\"ton_addr\",\"address\":\"0:dc69be3a989b1513f14e9e4ecc46ff6753770a2b199138708ff66238c2719e99\",\"network\":\"-239\",\"publicKey\":\"321b717a3a455096b6c3d20ad3f66207125557dacbb888fca9e00023aa89bfff\",\"walletStateInit\":\"te6cckECFgEAAwQAAgE0ARUBFP8A9KQT9LzyyAsCAgEgAxACAUgEBwLm0AHQ0wMhcbCSXwTgItdJwSCSXwTgAtMfIYIQcGx1Z70ighBkc3RyvbCSXwXgA/pAMCD6RAHIygfL/8nQ7UTQgQFA1yH0BDBcgQEI9ApvoTGzkl8H4AXTP8glghBwbHVnupI4MOMNA4IQZHN0crqSXwbjDQUGAHgB+gD0BDD4J28iMFAKoSG+8uBQghBwbHVngx6xcIAYUATLBSbPFlj6Ahn0AMtpF8sfUmDLPyDJgED7AAYAilAEgQEI9Fkw7UTQgQFA1yDIAc8W9ADJ7VQBcrCOI4IQZHN0coMesXCAGFAFywVQA88WI/oCE8tqyx/LP8mAQPsAkl8D4gIBIAgPAgEgCQ4CAVgKCwA9sp37UTQgQFA1yH0BDACyMoHy//J0AGBAQj0Cm+hMYAIBIAwNABmtznaiaEAga5Drhf/AABmvHfaiaEAQa5DrhY/AABG4yX7UTQ1wsfgAWb0kK29qJoQICga5D6AhhHDUCAhHpJN9KZEM5pA+n/mDeBKAG3gQFImHFZ8xhAT48oMI1xgg0x/TH9MfAvgju/Jk7UTQ0x/TH9P/9ATRUUO68qFRUbryogX5AVQQZPkQ8qP4ACSkyMsfUkDLH1Iwy/9SEPQAye1U+A8B0wchwACfbFGTINdKltMH1AL7AOgw4CHAAeMAIcAC4wABwAORMOMNA6TIyx8Syx/L/xESExQAbtIH+gDU1CL5AAXIygcVy//J0Hd0gBjIywXLAiLPFlAF+gIUy2sSzMzJc/sAyEAUgQEI9FHypwIAcIEBCNcY+gDTP8hUIEeBAQj0UfKnghBub3RlcHSAGMjLBcsCUAbPFlAE+gIUy2oSyx/LP8lz+wACAGyBAQjXGPoA0z8wUiSBAQj0WfKnghBkc3RycHSAGMjLBcsCUAXPFlAD+gITy2rLHxLLP8lz+wAACvQAye1UAFEAAAAAKamjFzIbcXo6RVCWtsPSCtP2YgcSVVfay7iI/KngACOqib//QMgBKq8=\"}],\"device\":{\"platform\":\"iphone\",\"appName\":\"Tonkeeper\",\"appVersion\":\"3.0.304\",\"maxProtocolVersion\":2,\"features\":[{\"name\":\"SendTransaction\",\"maxMessages\":4}]}}}";
        let deserialized: WalletEvent = serde_json::from_str(input).unwrap();
        match deserialized {
            WalletEvent::Connect { .. } => assert!(true),
            _ => assert!(false),
        }
    }
}
