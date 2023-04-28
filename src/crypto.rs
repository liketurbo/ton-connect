use crate::types::{BridgeMessage, WalletEvent};
use crypto_box::{
    aead::{Aead, OsRng},
    PublicKey, SalsaBox, SecretKey,
};
use generic_array::{typenum::U24, GenericArray};
use std::convert::TryInto;

const NONCE_LENGTH: usize = 24;

pub struct ClientKeypair {
    pub public: PublicKey,
    secret: SecretKey,
}

impl ClientKeypair {
    pub fn random() -> Self {
        let secret = SecretKey::generate(&mut OsRng);
        let public = secret.public_key();

        Self { secret, public }
    }

    fn get_box(&self, public: &PublicKey) -> SalsaBox {
        SalsaBox::new(public, &self.secret)
    }

    pub fn decrypt_message(
        &self,
        bridge_msg: BridgeMessage,
    ) -> Result<WalletEvent, Box<dyn std::error::Error>> {
        let payload =
            base64::decode(&bridge_msg.message).expect("invalid base64 message from bridge");
        let sender_pubkey = hex::decode(&bridge_msg.from).expect("invalid hex sender pubkey");
        let sender_pubkey: [u8; 32] = sender_pubkey
            .try_into()
            .map_err(|_| "invalid sender pubkey length")?;
        let sender_pubkey = PublicKey::from(sender_pubkey);
        let my_box: SalsaBox = self.get_box(&sender_pubkey);
        let nonce: &[u8; NONCE_LENGTH] = &payload[..NONCE_LENGTH].try_into().unwrap();
        let nonce: GenericArray<u8, U24> = GenericArray::from(*nonce);
        let ciphertext = &payload[NONCE_LENGTH..];
        let plaintext = my_box
            .decrypt(&nonce, ciphertext)
            .map_err(|_| "decryption failed")?;
        let plaintext = String::from_utf8(plaintext)?;
        let wallet_event: WalletEvent = serde_json::from_str(&plaintext)?;
        Ok(wallet_event)
    }
}
