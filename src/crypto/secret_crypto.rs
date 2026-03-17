use orion::hazardous::aead::xchacha20poly1305::{seal, open, Nonce, SecretKey};
use orion::kdf::{derive_key, Password, Salt};
use rand_core::{OsRng, RngCore};
use base64::engine::general_purpose::STANDARD as BASE64_ENGINE;
use base64::Engine as _;

pub fn encrypt(secret: &str, master_password: String) -> String {
    let mut nonce_bytes = [0u8; 24];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes).unwrap();

    let password = Password::from_slice(master_password.as_bytes()).unwrap();
    let salt = Salt::from_slice(&nonce_bytes).unwrap();
    let kdf_key = derive_key(&password, &salt, 3, 65536, 32).unwrap();
    let key = SecretKey::from_slice(kdf_key.unprotected_as_bytes()).unwrap();

    let mut ciphertext = vec![0u8; secret.len() + 16];
    seal(&key, &nonce, secret.as_bytes(), None, &mut ciphertext).unwrap();

    let mut combined = Vec::new();
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    BASE64_ENGINE.encode(&combined)
}

pub fn decrypt(stored: &str, master_password: String) -> Option<String> {
    let combined = BASE64_ENGINE.decode(stored).ok()?;
    if combined.len() < 24 { return None; }

    let (nonce_bytes, ciphertext) = combined.split_at(24);
    let nonce = Nonce::from_slice(nonce_bytes).unwrap();

    let password = Password::from_slice(master_password.as_bytes()).unwrap();
    let salt = Salt::from_slice(nonce_bytes).unwrap();
    let kdf_key = derive_key(&password, &salt, 3, 65536, 32).unwrap();
    let key = SecretKey::from_slice(kdf_key.unprotected_as_bytes()).unwrap();

    let mut plaintext = vec![0u8; ciphertext.len() - 16];
    open(&key, &nonce, ciphertext, None, &mut plaintext).ok()?;
    Some(String::from_utf8(plaintext).unwrap())
}