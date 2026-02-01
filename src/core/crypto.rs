use crate::core::models::Vault;
use aes_gcm::{Aes256Gcm, aead::{Aead, KeyInit}, Nonce};
use argon2::{Argon2, Algorithm, Version, Params};
use rand::{RngCore, thread_rng};
use anyhow::{Result, anyhow};

pub fn generate_random_bytes(len: usize) -> Vec<u8> {

    let mut b = vec![0u8; len];
    thread_rng().fill_bytes(&mut b);

    b
}

pub fn derive_key(password: &str, salt: &[u8]) -> Result<Vec<u8>> {

    let params = Params::new(64000, 3, 1, Some(32)).map_err(|e| anyhow!(e))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut cle = [0u8; 32];

    argon2.hash_password_into(password.as_bytes(), salt, &mut cle).map_err(|e| anyhow!(e))?;

    Ok(cle.to_vec())
}

pub fn encrypt_vault(vault: &Vault, key: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {

    let ser = vault.to_json()?;
    let r = generate_random_bytes(12);

    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| anyhow!("Key error"))?;

    let cipher_text = cipher.encrypt(Nonce::from_slice(&r), ser.as_bytes()).map_err(|e| anyhow!(e))?;
    Ok((cipher_text, r))

}

pub fn decrypt_vault(data: &[u8], key: &[u8], r: &[u8]) -> Result<Vault> {
    
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| anyhow!("Key error"))?;

    let decrypted = cipher.decrypt(Nonce::from_slice(r), data).map_err(|_| anyhow!("Auth error"))?;

    let json = std::str::from_utf8(&decrypted)?;
    
    Ok(Vault::from_json(json)?)
}