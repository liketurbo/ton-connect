--- Request part ---

1. Generate a key pair
2. Generate InitialRequest
    1. Read the app manifest
        ```json
        {
            "url": "<app-url>",                        // required
            "name": "<app-name>",                      // required
            "iconUrl": "<app-icon-url>",               // required
            "termsOfUseUrl": "<terms-of-use-url>",     // optional
            "privacyPolicyUrl": "<privacy-policy-url>" // optional
        }
        ```
    2. Create the request object
        ```json
        {
            manifestUrl: string;
            items: ConnectItem[], // data items to share with the app
        }
        type ConnectItem = TonAddressItem | TonProofItem | ...;
        type TonAddressItem = {
          name: "ton_addr";
        }
        type TonProofItem = {
          name: "ton_proof";
          // arbitrary payload, e.g. nonce + expiration timestamp.
          payload: string;
        }
        ```

--- Response part ---

1. Handle ConnectionEvent
    ```
    type ConnectEvent = ConnectEventSuccess | ConnectEventError;

    type ConnectEventSuccess = {
      event: "connect";
      id: number; // increasing event counter
      payload: {
          items: ConnectItemReply[];
          device: DeviceInfo;   
      }
    }
    type ConnectEventError = {
      event: "connect_error",
      id: number; // increasing event counter
      payload: {
          code: number;
          message: string;
      }
    }

    type DeviceInfo = {
      platform: "iphone" | "ipad" | "android" | "windows" | "mac" | "linux";
      appName:      string; // e.g. "Tonkeeper"  
      appVersion:  string; // e.g. "2.3.367"
      maxProtocolVersion: number;
      features: Feature[]; // list of supported features and methods in RPC
                                    // Currently there is only one feature -- 'SendTransaction'; 
    }

    type Feature = { name: 'SendTransaction', maxMessages: number } | // `maxMessages` is maximum number of messages in one `SendTransaction` that the wallet supports
            { name: 'SignData' };

    type ConnectItemReply = TonAddressItemReply | TonProofItemReply ...;

    type TonAddressItemReply = {
      name: "ton_addr";
      address: string; // TON address raw (`0:<hex>`)
      network: NETWORK; // network global_id
      publicKey: string; // HEX string without 0x
      walletStateInit: string; // Base64 (not url safe) encoded stateinit cell for the wallet contract
    }

    type TonProofItemReply = TonProofItemReplySuccess | TonProofItemReplyError;

    type TonProofItemReplySuccess = {
      name: "ton_proof";
      proof: {
        timestamp: string; // 64-bit unix epoch time of the signing operation (seconds)
        domain: {
          lengthBytes: number; // AppDomain Length
          value: string;  // app domain name (as url part, without encoding) 
        };
        signature: string; // base64-encoded signature
        payload: string; // payload from the request
      }
    }

    type TonProofItemReplyError = {
      name: "ton_addr";
      error: {
          code: ConnectItemErrorCode;
          message?: string;
      }
    }

    enum NETWORK {
      MAINNET = '-239',
      TESTNET = '-3'
    }
    ```
