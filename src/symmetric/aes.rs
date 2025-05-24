use aes::Aes256;
use block_padding::Pkcs7;
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use cbc::{Decryptor, Encryptor};
use rand::Rng;

/// Verschlüsselt Daten mit AES-256 im CBC-Modus mit PKCS7-Padding.
/// Gibt IV und Ciphertext zurück.
pub fn encrypt_aes256(key: &[u8; 32], plaintext: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let iv: [u8; 16] = rand::thread_rng().gen();
    let mut buffer = vec![0u8; plaintext.len() + 16]; // +Blockgröße für Padding
    buffer[..plaintext.len()].copy_from_slice(plaintext);

    let cipher = Encryptor::<Aes256>::new(key.into(), &iv.into());
    let ct = cipher
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, plaintext.len())
        .unwrap();
    (iv.to_vec(), ct.to_vec())
}

/// Entschlüsselt AES-256-CBC mit PKCS7.
/// Gibt den Klartext zurück.
pub fn decrypt_aes256(key: &[u8; 32], iv: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let cipher = Decryptor::<Aes256>::new(key.into(), iv.into());
    let mut buf = ciphertext.to_vec();
    let pt = cipher.decrypt_padded_mut::<Pkcs7>(&mut buf).unwrap();
    pt.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_encrypt_decrypt() {
        let key: [u8; 32] = rand::thread_rng().gen();
        let plaintext = b"Das ist eine geheime Nachricht.";
        let (iv, ct) = encrypt_aes256(&key, plaintext);
        let pt = decrypt_aes256(&key, &iv, &ct);
        assert_eq!(pt, plaintext);
    }
}
