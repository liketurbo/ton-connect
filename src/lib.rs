pub extern crate base64;
extern crate crypto_box;
extern crate hex;
extern crate mime;
extern crate percent_encoding;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use crypto_box::PublicKey;
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Url,
};
use std::io::{BufRead, BufReader};

pub mod crypto;
pub mod link;
pub mod types;

use types::{BridgeMessage, ConnectRequest, Topic};

pub struct HttpBridge {
    url: Url,
    client: reqwest::blocking::Client,
    last_event_id: Option<String>,
}

#[derive(Debug, Default)]
pub struct Event {
    pub id: Option<String>,
    pub kind: Option<String>,
    pub data: String,
}

impl HttpBridge {
    pub fn new(url: &str) -> Self {
        let url = Url::parse(&url).unwrap();
        let client = reqwest::blocking::Client::new();
        let last_event_id = None;
        Self {
            url,
            client,
            last_event_id,
        }
    }

    pub fn listen(
        &mut self,
        client_ids: &Vec<&PublicKey>,
        topics: &Option<Vec<Topic>>,
        handler: impl Fn(BridgeMessage) -> (),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut headers: HeaderMap<HeaderValue> = HeaderMap::with_capacity(2);
        headers.insert("Accept", "text/event-stream".parse()?);
        if let Some(ref last_event_id) = &self.last_event_id {
            headers.insert("Last-Event-ID", last_event_id.parse()?);
        }

        let mut url = self.url.clone();

        {
            let mut path_segments = url.path_segments_mut().map_err(|_| "cannot be base")?;
            path_segments.push("events");
        }

        if client_ids.is_empty() {
            return Err("client_ids is empty".into());
        }

        {
            let values = client_ids
                .iter()
                .map(|client_id| hex::encode(client_id.as_bytes()))
                .collect::<Vec<String>>()
                .join(",");
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

        let res = self.client.get(url).headers(headers).send()?;
        if !res.status().is_success() {
            return Err(format!("request failed with status: {}", res.status()).into());
        }
        if let Some(content_type_value) = res.headers().get(CONTENT_TYPE) {
            let mime_type = content_type_value
                .to_str()?
                .to_string()
                .parse::<mime::Mime>()?;
            if mime_type.type_() != mime::TEXT || mime_type.subtype() != mime::EVENT_STREAM {
                return Err(
                    format!("expected content type text/event-stream, got {}", mime_type).into(),
                );
            }
        } else {
            return Err("expected content type text/event-stream, got none".into());
        }

        let mut stream = BufReader::new(res);
        let mut line = String::new();
        let mut event = Event::default();
        // Event stream format:
        // \r\n\
        // body: heartbeat
        // \r\n\
        // body: heartbeat
        // ... and maybe an event
        // we are assuming that the first line is always \r\n\
        let mut ignore_next_blank = true;
        loop {
            line.clear();
            stream.read_line(&mut line)?;

            if line.is_empty() {
                return Err("unexpected end of stream".into());
            }

            let line = line.trim_end_matches(|c| c == '\r' || c == '\n');

            if line == "body: heartbeat" {
                ignore_next_blank = true;
                continue;
            } else if ignore_next_blank {
                ignore_next_blank = false;
                continue;
            }

            if line.is_empty() {
                let bridge_msg: BridgeMessage = serde_json::from_str(&event.data).unwrap();
                handler(bridge_msg);
                event = Event::default();
                continue;
            }

            let (field, value) = if let Some(pos) = line.find(':') {
                let (f, v) = line.split_at(pos);
                // Strip ":" and space
                let v = &v[1..];
                let v = if v.starts_with(' ') { &v[1..] } else { v };
                (f, v)
            } else {
                (line, "")
            };

            match field {
                "id" => {
                    event.id = Some(value.to_string());
                }
                "event" => {
                    event.kind = Some(value.to_string());
                }
                "data" => {
                    event.data = value.to_string();
                }
                _ => {}
            }
        }
    }

    pub fn send_transaction() {
        unimplemented!()
    }
}
