use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

/// App needs to have its manifest to pass meta information to the wallet.
#[derive(Debug, Serialize, Deserialize)]
pub struct AppManifest {
    /// App URL. Will be used as the dapp identifier. Will be used to open the dapp after click to its icon in the wallet. It is recommended to pass url without closing slash, e.g. 'https://mydapp.com' instead of 'https://mydapp.com/'.
    pub url: String,
    /// App name. Might be simple, will not be used as identifier.
    pub name: String,
    /// Url to the app icon. Must be PNG, ICO, ... format. SVG icons are not supported. Perfectly pass url to a 180x180px PNG icon.
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    /// Url to the Terms Of Use document. Optional for usual apps, but required for the apps which is placed in the Tonkeeper recommended apps list.
    #[serde(rename = "termsOfUseUrl")]
    pub terms_of_use_url: Option<String>,
    /// Url to the Privacy Policy document. Optional for usual apps, but required for the apps which is placed in the Tonkeeper recommended apps list.
    #[serde(rename = "privacyPolicyUrl")]
    pub privacy_policy_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectRequest {
    /// Link to the app's tonconnect-manifest.json
    #[serde(rename = "manifestUrl")]
    pub manifest_url: String,
    /// Data items to share with the app.
    pub items: Vec<ConnectItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum ConnectItem {
    #[serde(rename = "ton_addr")]
    TonAddressItem,
    /// Arbitrary payload, e.g. nonce + expiration timestamp.
    #[serde(rename = "ton_proof")]
    TonProofItem { payload: String },
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

/// Wallet responds with ConnectEvent message if the user approves the request.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum WalletEvent {
    #[serde(rename = "connect")]
    Connect {
        /// Increasing event counter.
        id: u32,
        payload: ConnectPayload,
    },
    #[serde(rename = "connect_error")]
    ConnectError {
        /// Increasing event counter.
        id: u32,
        payload: ConnectErrorPayload,
    },
    #[serde(rename = "disconnect")]
    Disconnect {
        /// Increasing event counter.
        id: u32,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectPayload {
    pub items: Vec<ConnectItemReply>,
    pub device: DeviceInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectErrorPayload {
    /// | Code | Description                   |
    /// |------|-------------------------------|
    /// | 0    | Unknown error                 |
    /// | 1    | Bad request                   |
    /// | 2    | App manifest not found        |
    /// | 3    | App manifest content error    |
    /// | 100  | Unknown app                   |
    /// | 300  | User declined the connection  |
    pub code: u32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum ConnectItemReply {
    /// Untrusted data returned by the wallet.
    /// If you need a guarantee that the user owns this address and public key, you need to additionally request a ton_proof.
    #[serde(rename = "ton_addr")]
    TonAddress {
        /// TON address raw (`0:<hex>`).
        address: String,
        /// Network `global_id`.
        network: NETWORK,
        /// HEX string without 0x.
        #[serde(rename = "publicKey")]
        public_key: String,
        /// Base64 (not url safe) encoded stateinit cell for the wallet contract.
        #[serde(rename = "walletStateInit")]
        wallet_state_init: String,
    },
    #[serde(rename = "ton_proof")]
    TonProofItemReplySuccess {
        proof: TonProofItemReplySuccessData,
    },
    TonProofItemReplyError {
        error: TonProofItemReplyErrorData,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NETWORK {
    #[serde(rename = "-239")]
    MAINNET,
    #[serde(rename = "-3")]
    TESTNET,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TonProofItemReplySuccessData {
    /// 64-bit unix epoch time of the signing operation (seconds).
    #[serde(with = "timestamp")]
    pub timestamp: timestamp::Timestamp,
    pub domain: TonProofDomain,
    /// Base64-encoded signature.
    pub signature: String,
    /// Payload from the request.
    pub payload: String,
}

mod timestamp {
    use serde::{Deserialize, Deserializer, Serializer};

    #[derive(Debug)]
    pub enum Timestamp {
        StringValue(String),
        NumberValue(u64),
    }

    pub fn serialize<S>(value: &Timestamp, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Timestamp::StringValue(string_value) => serializer.serialize_str(string_value),
            Timestamp::NumberValue(number_value) => serializer.serialize_u64(*number_value),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Timestamp, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer).and_then(|value| {
            if let Some(string_value) = value.as_str() {
                Ok(Timestamp::StringValue(string_value.to_owned()))
            } else if let Some(number_value) = value.as_u64() {
                Ok(Timestamp::NumberValue(number_value))
            } else {
                Err(serde::de::Error::custom("Invalid timestamp format"))
            }
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TonProofItemReplyErrorData {
    /// | Code | Description                   |
    /// |------|-------------------------------|
    /// | 0    | Unknown error                 |
    /// | 400  | Method is not supported       |
    pub code: u32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TonProofDomain {
    /// AppDomain length.
    #[serde(rename = "lengthBytes")]
    pub length_bytes: u32,
    /// App domain name (as url part, without encoding).
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub platform: Platform,
    /// E.g. "Tonkeeper".
    #[serde(rename = "appName")]
    pub app_name: String,
    /// E.g. "2.3.367".
    #[serde(rename = "appVersion")]
    pub app_version: String,
    #[serde(rename = "maxProtocolVersion")]
    pub max_protocol_version: u32,
    /// List of supported features and methods in RPC.
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
    fn test_serialize_link() {
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
    fn test_deserialize_event() {
        let input = "{\"id\":65,\"event\":\"connect\",\"payload\":{\"items\":[{\"name\":\"ton_addr\",\"address\":\"0:dc69be3a989b1513f14e9e4ecc46ff6753770a2b199138708ff66238c2719e99\",\"network\":\"-239\",\"publicKey\":\"321b717a3a455096b6c3d20ad3f66207125557dacbb888fca9e00023aa89bfff\",\"walletStateInit\":\"te6cckECFgEAAwQAAgE0ARUBFP8A9KQT9LzyyAsCAgEgAxACAUgEBwLm0AHQ0wMhcbCSXwTgItdJwSCSXwTgAtMfIYIQcGx1Z70ighBkc3RyvbCSXwXgA/pAMCD6RAHIygfL/8nQ7UTQgQFA1yH0BDBcgQEI9ApvoTGzkl8H4AXTP8glghBwbHVnupI4MOMNA4IQZHN0crqSXwbjDQUGAHgB+gD0BDD4J28iMFAKoSG+8uBQghBwbHVngx6xcIAYUATLBSbPFlj6Ahn0AMtpF8sfUmDLPyDJgED7AAYAilAEgQEI9Fkw7UTQgQFA1yDIAc8W9ADJ7VQBcrCOI4IQZHN0coMesXCAGFAFywVQA88WI/oCE8tqyx/LP8mAQPsAkl8D4gIBIAgPAgEgCQ4CAVgKCwA9sp37UTQgQFA1yH0BDACyMoHy//J0AGBAQj0Cm+hMYAIBIAwNABmtznaiaEAga5Drhf/AABmvHfaiaEAQa5DrhY/AABG4yX7UTQ1wsfgAWb0kK29qJoQICga5D6AhhHDUCAhHpJN9KZEM5pA+n/mDeBKAG3gQFImHFZ8xhAT48oMI1xgg0x/TH9MfAvgju/Jk7UTQ0x/TH9P/9ATRUUO68qFRUbryogX5AVQQZPkQ8qP4ACSkyMsfUkDLH1Iwy/9SEPQAye1U+A8B0wchwACfbFGTINdKltMH1AL7AOgw4CHAAeMAIcAC4wABwAORMOMNA6TIyx8Syx/L/xESExQAbtIH+gDU1CL5AAXIygcVy//J0Hd0gBjIywXLAiLPFlAF+gIUy2sSzMzJc/sAyEAUgQEI9FHypwIAcIEBCNcY+gDTP8hUIEeBAQj0UfKnghBub3RlcHSAGMjLBcsCUAbPFlAE+gIUy2oSyx/LP8lz+wACAGyBAQjXGPoA0z8wUiSBAQj0WfKnghBkc3RycHSAGMjLBcsCUAXPFlAD+gITy2rLHxLLP8lz+wAACvQAye1UAFEAAAAAKamjFzIbcXo6RVCWtsPSCtP2YgcSVVfay7iI/KngACOqib//QMgBKq8=\"}],\"device\":{\"platform\":\"iphone\",\"appName\":\"Tonkeeper\",\"appVersion\":\"3.0.304\",\"maxProtocolVersion\":2,\"features\":[\"SendTransaction\",{\"name\":\"SendTransaction\",\"maxMessages\":4}]}}}";
        let deserialized: WalletEvent = serde_json::from_str(input).unwrap();
        match deserialized {
            WalletEvent::Connect { .. } => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_custom_serde() {
        let device_info_1 = DeviceInfo {
            platform: Platform::IPhone,
            app_name: "Cool App".to_owned(),
            app_version: "2.0.0".to_owned(),
            max_protocol_version: 2,
            features: vec![
                Feature::SendTransactionDeprecated,
                Feature::SignData,
                Feature::SendTransaction { max_messages: 10 },
            ],
        };
        let device_info_2: DeviceInfo = serde_json::from_str("{\"platform\":\"iphone\",\"appName\":\"Cool App\",\"appVersion\":\"2.0.0\",\"maxProtocolVersion\":2,\"features\":[\"SendTransaction\",{\"name\":\"SignData\"},{\"name\":\"SendTransaction\",\"maxMessages\":10}]}").unwrap();

        let serialized_1 = serde_json::to_string(&device_info_1).unwrap();
        let serialized_2 = serde_json::to_string(&device_info_2).unwrap();
        assert_eq!(serialized_1, serialized_2);
    }
}
