use crate::types::ConnectRequest;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde_json;

/// Creates a universal link for TON Connect using the provided wallet universal URL, client ID, and ConnectRequest.
///
/// # Example
///
/// ```
/// use ton_connect::types::{ConnectRequest, ConnectItem};
/// use ton_connect::link::create_universal_link;
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
///
/// # Arguments
///
/// * `wallet_universal_url` - The wallet universal URL.
/// * `client_id` - The client public key encoded as hex.
/// * `connect_request` - The ConnectRequest containing the manifest URL and items.
///
/// # Returns
///
/// A Result containing the generated universal link as a String, or an error if the serialization fails.
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
    Ok(universal_link)
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
        assert_eq!(universal_link, "https://app.tonkeeper.com/ton-connect?v=2&id=a3baaa66a1eee1dbe79058aca5980a2222fcc418508635f9317e9dc8c3108201&r=%7B%22manifestUrl%22%3A%22https%3A%2F%2Fraw%2Egithubusercontent%2Ecom%2FXaBbl4%2Fpytonconnect%2Fmain%2Fpytonconnect%2Dmanifest%2Ejson%22%2C%22items%22%3A%5B%7B%22name%22%3A%22ton%5Faddr%22%7D%2C%7B%22name%22%3A%22ton%5Fproof%22%2C%22payload%22%3A%22some%5Fpayload%22%7D%5D%7D");
    }
}
