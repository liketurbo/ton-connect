use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

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

#[derive(Debug, Serialize, Deserialize)]
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
pub struct DeviceInfo {
    pub platform: Platform,
    #[serde(rename = "appName")]
    pub app_name: String,
    #[serde(rename = "appVersion")]
    pub app_version: String,
    #[serde(rename = "maxProtocolVersion")]
    pub max_protocol_version: u32,
    pub features: Vec<Feature>,
}

#[derive(Debug)]
pub enum Feature {
    SendTransactionDeprecated,
    SendTransaction { max_messages: u32 },
    SignData,
}

impl Serialize for Feature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Feature::SignData => {
                let mut sign_data = serializer.serialize_struct("SignData", 1)?;
                sign_data.serialize_field("name", "SignData")?;
                sign_data.end()
            }
            Feature::SendTransaction { max_messages } => {
                let mut sign_transaction = serializer.serialize_struct("SendTransaction", 2)?;
                sign_transaction.serialize_field("name", "SendTransaction")?;
                sign_transaction.serialize_field("maxMessages", max_messages)?;
                sign_transaction.end()
            }
            Feature::SendTransactionDeprecated => serializer.serialize_str("SendTransaction"),
        }
    }
}

impl<'de> Deserialize<'de> for Feature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};

        struct FeatureVisitor;

        impl<'de> Visitor<'de> for FeatureVisitor {
            type Value = Feature;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a valid Feature representation")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value == "SendTransaction" {
                    Ok(Feature::SendTransactionDeprecated)
                } else {
                    Err(de::Error::unknown_variant(value, &["SendTransaction"]))
                }
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut name: Option<String> = None;
                let mut max_messages: Option<u32> = None;
                let mut encountered_name_field = false;

                while let Some(key) = map.next_key()? {
                    match key {
                        "name" => {
                            if encountered_name_field {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            encountered_name_field = true;
                            name = Some(map.next_value()?);
                        }
                        "maxMessages" => {
                            max_messages = Some(map.next_value()?);
                        }
                        _ => {
                            // Ignore unknown fields
                            let _: de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                if !encountered_name_field {
                    return Err(de::Error::missing_field("name"));
                }

                match name.as_deref() {
                    Some("SendTransaction") => {
                        let max_messages =
                            max_messages.ok_or_else(|| de::Error::missing_field("maxMessages"))?;
                        Ok(Feature::SendTransaction { max_messages })
                    }
                    Some("SignData") => Ok(Feature::SignData),
                    _ => Err(de::Error::unknown_variant(
                        &name.unwrap(),
                        &["SendTransaction", "SignData"],
                    )),
                }
            }
        }

        deserializer.deserialize_any(FeatureVisitor)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Platform {
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
        let input = "{\"id\":65,\"event\":\"connect\",\"payload\":{\"items\":[{\"name\":\"ton_addr\",\"address\":\"0:dc69be3a989b1513f14e9e4ecc46ff6753770a2b199138708ff66238c2719e99\",\"network\":\"-239\",\"publicKey\":\"321b717a3a455096b6c3d20ad3f66207125557dacbb888fca9e00023aa89bfff\",\"walletStateInit\":\"te6cckECFgEAAwQAAgE0ARUBFP8A9KQT9LzyyAsCAgEgAxACAUgEBwLm0AHQ0wMhcbCSXwTgItdJwSCSXwTgAtMfIYIQcGx1Z70ighBkc3RyvbCSXwXgA/pAMCD6RAHIygfL/8nQ7UTQgQFA1yH0BDBcgQEI9ApvoTGzkl8H4AXTP8glghBwbHVnupI4MOMNA4IQZHN0crqSXwbjDQUGAHgB+gD0BDD4J28iMFAKoSG+8uBQghBwbHVngx6xcIAYUATLBSbPFlj6Ahn0AMtpF8sfUmDLPyDJgED7AAYAilAEgQEI9Fkw7UTQgQFA1yDIAc8W9ADJ7VQBcrCOI4IQZHN0coMesXCAGFAFywVQA88WI/oCE8tqyx/LP8mAQPsAkl8D4gIBIAgPAgEgCQ4CAVgKCwA9sp37UTQgQFA1yH0BDACyMoHy//J0AGBAQj0Cm+hMYAIBIAwNABmtznaiaEAga5Drhf/AABmvHfaiaEAQa5DrhY/AABG4yX7UTQ1wsfgAWb0kK29qJoQICga5D6AhhHDUCAhHpJN9KZEM5pA+n/mDeBKAG3gQFImHFZ8xhAT48oMI1xgg0x/TH9MfAvgju/Jk7UTQ0x/TH9P/9ATRUUO68qFRUbryogX5AVQQZPkQ8qP4ACSkyMsfUkDLH1Iwy/9SEPQAye1U+A8B0wchwACfbFGTINdKltMH1AL7AOgw4CHAAeMAIcAC4wABwAORMOMNA6TIyx8Syx/L/xESExQAbtIH+gDU1CL5AAXIygcVy//J0Hd0gBjIywXLAiLPFlAF+gIUy2sSzMzJc/sAyEAUgQEI9FHypwIAcIEBCNcY+gDTP8hUIEeBAQj0UfKnghBub3RlcHSAGMjLBcsCUAbPFlAE+gIUy2oSyx/LP8lz+wACAGyBAQjXGPoA0z8wUiSBAQj0WfKnghBkc3RycHSAGMjLBcsCUAXPFlAD+gITy2rLHxLLP8lz+wAACvQAye1UAFEAAAAAKamjFzIbcXo6RVCWtsPSCtP2YgcSVVfay7iI/KngACOqib//QMgBKq8=\"}],\"device\":{\"platform\":\"iphone\",\"appName\":\"Tonkeeper\",\"appVersion\":\"3.0.304\",\"maxProtocolVersion\":2,\"features\":[\"SendTransaction\",{\"name\":\"SendTransaction\",\"maxMessages\":4}]}}}";
        let deserialized: WalletEvent = serde_json::from_str(input).unwrap();
        match deserialized {
            WalletEvent::Connect { .. } => assert!(true),
            _ => assert!(false),
        }
    }
}
