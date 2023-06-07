use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde_json;
use types::{ConnectRequest, Topic};
use url::Url;

/// Creates a universal link for TON Connect using the provided wallet universal URL, client ID, and ConnectRequest.
///
/// # Example
///
/// ```
/// use ton_connect::types::{ConnectRequest, ConnectItem};
/// use ton_connect::helpers::create_universal_link;
///
/// let wallet_universal_url = "https://app.tonkeeper.com/ton-connect";
/// let client_id = "a3baaa66a1eee1dbe79058aca5980a2222fcc418508635f9317e9dc8c3108201";
/// let connect_request = ConnectRequest {
///     manifest_url: "https://raw.githubusercontent.com/XaBbl4/pytonconnect/main/pytonconnect-manifest.json".to_string(),
///     items: vec![
///         ConnectItem::TonAddressItem,
///         ConnectItem::TonProofItem {
///             payload: "some_payload".to_string(),
///         },
///     ],
/// };
///
/// let universal_link = create_universal_link(&wallet_universal_url, &client_id, &connect_request).unwrap();
/// println!("Universal link: {}", universal_link);
/// ```
pub fn create_universal_link(
    wallet_universal_url: &str,
    client_id: &str,
    connect_request: &ConnectRequest,
) -> Result<String, Box<dyn std::error::Error>> {
    let connect_request = serde_json::to_string(&connect_request)?;
    let connect_request = utf8_percent_encode(&connect_request, NON_ALPHANUMERIC);
    let universal_link = format!(
        "{}?v=2&id={}&r={}",
        wallet_universal_url, client_id, connect_request
    );
    let url = Url::parse(&universal_link)?;
    Ok(url.to_string())
}

/// Creates a listen URL for subscribing to events using the provided bridge URL, client public keys, and optional topics.
///
/// # Example
///
/// ```
/// use ton_connect::types::Topic;
/// use ton_connect::helpers::create_listen_url;
///
/// let bridge_url = "https://bridge.tonapi.io/bridge";
/// let clients_ids = vec![
///     "a3baaa66a1eee1dbe79058aca5980a2222fcc418508635f9317e9dc8c3108201".to_string(),
/// ];
/// let topics = Some(vec![Topic::SendTransaction]);
///
/// let url = create_listen_url(&bridge_url, &clients_ids, &topics).unwrap();
/// println!("Listen URL: {}", url);
/// ```
pub fn create_listen_url(
    bridge_url: &str,
    clients_ids: &Vec<String>,
    topics: &Option<Vec<Topic>>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut url = Url::parse(bridge_url)?;

    {
        let mut path_segments = url.path_segments_mut().map_err(|_| "cannot be base")?;
        path_segments.push("events");
    }

    if clients_ids.is_empty() {
        return Err("client_ids is empty".into());
    }

    {
        let values = clients_ids.join(",");
        url.query_pairs_mut().append_pair("client_id", &values);
    }

    if let Some(topics) = topics.as_ref() {
        let values = topics
            .iter()
            .map(|topic| serde_json::to_string(topic).expect("cannot serialize topic"))
            .map(|topic| topic.trim_matches('"').to_string())
            .collect::<Vec<String>>()
            .join(",");

        url.query_pairs_mut().append_pair("topic", &values);
    }

    Ok(url.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ConnectItem;

    #[test]
    fn test_universal_link_create() {
        let wallet_universal_url = "https://app.tonkeeper.com/ton-connect";
        let client_public = "a3baaa66a1eee1dbe79058aca5980a2222fcc418508635f9317e9dc8c3108201";
        let connect_request = ConnectRequest {
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
        let universal_link =
            create_universal_link(&wallet_universal_url, &client_public, &connect_request).unwrap();
        assert_eq!(universal_link.to_string(), "https://app.tonkeeper.com/ton-connect?v=2&id=a3baaa66a1eee1dbe79058aca5980a2222fcc418508635f9317e9dc8c3108201&r=%7B%22manifestUrl%22%3A%22https%3A%2F%2Fraw%2Egithubusercontent%2Ecom%2FXaBbl4%2Fpytonconnect%2Fmain%2Fpytonconnect%2Dmanifest%2Ejson%22%2C%22items%22%3A%5B%7B%22name%22%3A%22ton%5Faddr%22%7D%2C%7B%22name%22%3A%22ton%5Fproof%22%2C%22payload%22%3A%22some%5Fpayload%22%7D%5D%7D");
    }

    #[test]
    fn test_listen_url_create() {
        let bridge_url = "https://bridge.tonapi.io/bridge";
        let clients_ids =
            vec!["a3baaa66a1eee1dbe79058aca5980a2222fcc418508635f9317e9dc8c3108201".to_string()];
        let topics = Some(vec![Topic::SendTransaction]);

        let url = create_listen_url(&bridge_url, &clients_ids, &topics).unwrap();
        assert_eq!(url.to_string(), "https://bridge.tonapi.io/bridge/events?client_id=a3baaa66a1eee1dbe79058aca5980a2222fcc418508635f9317e9dc8c3108201&topic=sendTransaction");

        let url = create_listen_url(&bridge_url, &clients_ids, &None).unwrap();
        assert_eq!(url.to_string(), "https://bridge.tonapi.io/bridge/events?client_id=a3baaa66a1eee1dbe79058aca5980a2222fcc418508635f9317e9dc8c3108201");
    }
}
