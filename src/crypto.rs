use crypto_box::{
    aead::{
        generic_array::{typenum::U24, GenericArray},
        Aead, OsRng,
    },
    PublicKey, SalsaBox, SecretKey,
};
use std::convert::TryInto;

/// The length of the nonce used for encryption and decryption operations.
pub const NONCE_LENGTH: usize = 24;

/// Generates a random nonce of length `NONCE_LENGTH`.
///
/// # Example
///
/// ```
/// use ton_connect::crypto::generate_random_nonce;
///
/// let nonce = generate_random_nonce();
/// println!("Generated Nonce: {:?}", nonce);
/// ```
pub fn generate_random_nonce() -> [u8; NONCE_LENGTH] {
    let nonce: [u8; NONCE_LENGTH] = rand::random();
    nonce
}

/// Encrypts a plaintext message using the key pair and the recipient's public key.
///
/// # Example
///
/// ```
/// use ton_connect::crypto::{ClientKeypair, NONCE_LENGTH, encrypt_message};
///
/// let sender_keypair = ClientKeypair::from_hex_secret(
///     "e9f5b8703c101f2e8abd79789e8996a119d6aa4730eab85751e9ab03861c48a3",
/// )
/// .unwrap();
/// let receiver_public = "ac02417ce252b719c9fea60404a5b34edddb3b5dd5b1d6a45764b34ce92eb33b";
/// let nonce = [0u8; NONCE_LENGTH];
/// let plaintext = b"Hello, World!";
/// let ciphertext = encrypt_message(
///     &sender_keypair.secret,
///     plaintext,
///     &nonce,
///     &receiver_public,
/// ).unwrap();
/// println!("Encrypted data: {:?}", ciphertext);
/// ```
pub fn encrypt_message(
    secret: &SecretKey,
    plaintext: &[u8],
    nonce: &[u8],
    receiver_pubkey: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let pubkey = parse_hex_pubkey(receiver_pubkey)?;
    let my_box: SalsaBox = create_box(&secret, &pubkey);
    if nonce.len() != NONCE_LENGTH {
        return Err("incorrect nonce size".into());
    }
    let nonce = GenericArray::<u8, U24>::from_slice(&nonce);
    let ciphertext = my_box
        .encrypt(nonce, plaintext)
        .map_err(|_| "encryption failed")?;
    Ok(ciphertext)
}

/// Decrypts a ciphertext message using the key pair and the sender's public key.
///
/// # Example
/// ```
/// use ton_connect::crypto::{encrypt_message, decrypt_message, ClientKeypair, NONCE_LENGTH};
///
/// let sender_keypair = ClientKeypair::from_hex_secret(
///     "e9f5b8703c101f2e8abd79789e8996a119d6aa4730eab85751e9ab03861c48a3",
/// )
/// .unwrap();
/// let receiver_keypair = ClientKeypair::from_hex_secret(
///     "6a17d2188b8ad1b6729c7eb9fbb2984641db90e12321f3f3c2f600473fc1cd37",
/// ).unwrap();
/// let nonce = [0u8; NONCE_LENGTH];
/// let plaintext = b"Hello, World!";
/// let ciphertext = encrypt_message(
///     &sender_keypair.secret,
///     plaintext,
///     &nonce,
///     &receiver_keypair.get_hex_public()
/// ).unwrap();
/// let decrypted = decrypt_message(
///     &receiver_keypair.secret,
///     &ciphertext,
///     &nonce,
///     &sender_keypair.get_hex_public()
/// ).unwrap();
/// println!("Hello, World! = {}", decrypted);
/// ```
pub fn decrypt_message(
    secret: &SecretKey,
    ciphertext: &[u8],
    nonce: &[u8],
    sender_pubkey: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let pubkey = parse_hex_pubkey(sender_pubkey)?;
    let my_box: SalsaBox = create_box(&secret, &pubkey);
    if nonce.len() != NONCE_LENGTH {
        return Err("incorrect nonce size".into());
    }
    let nonce = GenericArray::<u8, U24>::from_slice(&nonce);
    let plaintext = my_box
        .decrypt(nonce, ciphertext)
        .map_err(|_| "decryption failed")
        .map(String::from_utf8)??;
    Ok(plaintext)
}

pub struct ClientKeypair {
    pub public: PublicKey,
    pub secret: SecretKey,
}

impl ClientKeypair {
    /// Generates a random `ClientKeypair`.
    ///
    /// # Example
    ///
    /// ```
    /// use ton_connect::crypto::ClientKeypair;
    ///
    /// let keypair = ClientKeypair::generate_random();
    /// println!("Public Key: {:?}", keypair.public);
    /// println!("Secret Key: {:?}", keypair.secret);
    /// ```
    pub fn generate_random() -> Self {
        let secret = SecretKey::generate(&mut OsRng);
        let public = secret.public_key();

        Self { secret, public }
    }

    /// Creates a `ClientKeypair` from a hexadecimal secret key.
    ///
    /// # Example
    ///
    /// ```
    /// use ton_connect::crypto::ClientKeypair;
    ///
    /// let hex_secret = "e9f5b8703c101f2e8abd79789e8996a119d6aa4730eab85751e9ab03861c48a3";
    /// let keypair = ClientKeypair::from_hex_secret(hex_secret).unwrap();
    /// println!("Public Key: {:?}", keypair.public);
    /// println!("Secret Key: {:?}", keypair.secret);
    /// ```
    pub fn from_hex_secret(hex_secret: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let secret = parse_hex_secret(&hex_secret)?;
        let public = secret.public_key();

        Ok(Self { secret, public })
    }

    /// Retrieves the hexadecimal representation of the secret key.
    ///
    /// # Example
    ///
    /// ```
    /// use ton_connect::crypto::ClientKeypair;
    ///
    /// let keypair = ClientKeypair::generate_random();
    /// let hex_secret = keypair.get_hex_secret();
    /// println!("Hex secret: {}", hex_secret);
    /// ```
    pub fn get_hex_secret(&self) -> String {
        hex::encode(self.secret.as_bytes())
    }

    /// Retrieves the hexadecimal representation of the public key.
    ///
    /// # Example
    ///
    /// ```
    /// use ton_connect::crypto::ClientKeypair;
    ///
    /// let keypair = ClientKeypair::generate_random();
    /// let hex_public = keypair.get_hex_public();
    /// println!("Hex public key: {}", hex_public);
    /// ```
    pub fn get_hex_public(&self) -> String {
        hex::encode(self.public.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_plaintext() {
        let sender_keypair = ClientKeypair::from_hex_secret(
            "e9f5b8703c101f2e8abd79789e8996a119d6aa4730eab85751e9ab03861c48a3",
        )
        .unwrap();
        let receiver_public = "a3baaa66a1eee1dbe79058aca5980a2222fcc418508635f9317e9dc8c3108201";
        let nonce = [0u8; NONCE_LENGTH];
        let plaintext = b"Hello, World!";
        let ciphertext =
            encrypt_message(&sender_keypair.secret, plaintext, &nonce, &receiver_public).unwrap();
        assert_ne!(ciphertext, plaintext); // Ciphertext should be different from plaintext
    }

    #[test]
    fn test_decrypt_ciphertext() {
        let sender_keypair = ClientKeypair::from_hex_secret(
            "e9f5b8703c101f2e8abd79789e8996a119d6aa4730eab85751e9ab03861c48a3",
        )
        .unwrap();
        let receiver_keypair = ClientKeypair::from_hex_secret(
            "6a17d2188b8ad1b6729c7eb9fbb2984641db90e12321f3f3c2f600473fc1cd37",
        )
        .unwrap();
        let nonce = [0u8; NONCE_LENGTH];
        let plaintext = b"Hello, World!";
        let ciphertext = encrypt_message(
            &sender_keypair.secret,
            plaintext,
            &nonce,
            &receiver_keypair.get_hex_public(),
        )
        .unwrap();
        let decrypted = decrypt_message(
            &receiver_keypair.secret,
            &ciphertext,
            &nonce,
            &sender_keypair.get_hex_public(),
        )
        .unwrap();
        assert_eq!(decrypted, String::from_utf8_lossy(plaintext)); // Decrypted plaintext should match the original plaintext
    }
}

fn parse_hex_pubkey(hex_pubkey: &str) -> Result<PublicKey, Box<dyn std::error::Error>> {
    let pubkey = hex::decode(hex_pubkey).expect("invalid hex pubkey");
    let pubkey: [u8; 32] = pubkey.try_into().map_err(|_| "invalid pubkey length")?;
    let pubkey = PublicKey::from(pubkey);
    Ok(pubkey)
}

fn parse_hex_secret(hex_secret: &str) -> Result<SecretKey, Box<dyn std::error::Error>> {
    let secret = hex::decode(hex_secret).expect("invalid hex secret");
    let secret: [u8; 32] = secret.try_into().map_err(|_| "invalid secret length")?;
    let secret = SecretKey::from(secret);
    Ok(secret)
}

fn create_box(secret: &SecretKey, public: &PublicKey) -> SalsaBox {
    SalsaBox::new(public, secret)
}
