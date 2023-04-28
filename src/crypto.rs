use chacha20poly1305;
use crypto_box::{
    aead::{Aead, AeadCore, OsRng},
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
        payload: &[u8],
        sender_pubkey: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sender_pubkey: [u8; 32] = sender_pubkey.try_into()?;
        let sender_pubkey = PublicKey::from(sender_pubkey);
        let my_box: SalsaBox = self.get_box(&sender_pubkey);
        let nonce: &[u8; NONCE_LENGTH] = &payload[..NONCE_LENGTH].try_into().unwrap();
        let nonce: GenericArray<u8, U24> = GenericArray::from(*nonce);
        let ciphertext = &payload[NONCE_LENGTH..];
        let plaintext = my_box
            .decrypt(&nonce, ciphertext)
            .map_err(|_| "decryption failed")?;
        Ok(())
    }
}
