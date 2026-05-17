use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
};
use base64::{Engine as _, engine::general_purpose};
use sha2::{Digest, Sha256};

pub(crate) fn encrypt_webhook_url(secret: &str, plaintext: &str) -> Result<String, String> {
    let cipher = cipher_from_secret(secret)?;
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|_| "encrypt failed".to_string())?;
    let mut payload = nonce_bytes.to_vec();
    payload.extend(ciphertext);
    Ok(general_purpose::STANDARD.encode(payload))
}

pub(crate) fn decrypt_webhook_url(secret: &str, ciphertext: &str) -> Result<String, String> {
    let payload = general_purpose::STANDARD
        .decode(ciphertext)
        .map_err(|_| "invalid base64".to_string())?;
    if payload.len() <= 12 {
        return Err("invalid ciphertext".to_string());
    }
    let (nonce_bytes, encrypted) = payload.split_at(12);
    let cipher = cipher_from_secret(secret)?;
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce_bytes), encrypted)
        .map_err(|_| "decrypt failed".to_string())?;
    String::from_utf8(plaintext).map_err(|_| "invalid utf8".to_string())
}

fn cipher_from_secret(secret: &str) -> Result<Aes256Gcm, String> {
    if secret.len() < 16 {
        return Err("secret too short".to_string());
    }
    let key = Sha256::digest(secret.as_bytes());
    Aes256Gcm::new_from_slice(&key).map_err(|_| "invalid key".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn webhook_url_roundtrips_through_encryption() {
        let secret = "test-session-secret-long-enough";
        let plaintext =
            "https://discord.com/api/webhooks/123456789012345678/abcdefghijklmnopqrstuvwxyz";

        let encrypted = encrypt_webhook_url(secret, plaintext).expect("encrypt");
        assert_ne!(encrypted, plaintext);
        assert!(!encrypted.contains("abcdefghijklmnopqrstuvwxyz"));

        let decrypted = decrypt_webhook_url(secret, &encrypted).expect("decrypt");
        assert_eq!(decrypted, plaintext);
    }
}
