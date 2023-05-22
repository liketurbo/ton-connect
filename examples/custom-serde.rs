extern crate serde_json;
extern crate ton_connect;

use ton_connect::types::{DeviceInfo, Feature, Platform};

fn main() {
    let device_info = DeviceInfo {
        platform: Platform::IPhone,
        app_name: "Cool App".to_string(),
        app_version: "2.0".to_string(),
        max_protocol_version: 2,
        features: vec![
            Feature::SendTransactionDeprecated,
            Feature::SignData,
            Feature::SendTransaction { max_messages: 27 },
        ],
    };
    let res = serde_json::to_string(&device_info).unwrap();
    println!("{}", res);
    let res: DeviceInfo = serde_json::from_str("{\"platform\":\"iphone\",\"appName\":\"Cool App\",\"appVersion\":\"2.0\",\"maxProtocolVersion\":2,\"features\":[\"SendTransaction\",{\"name\":\"SignData\"},{\"name\":\"SendTransaction\",\"maxMessages\":27}]}").unwrap();
    println!("{:?}", res);
}
