use crate::asymmetric::rsa::{decrypt_rsa, encrypt_rsa, PrivateKey, PublicKey};
use crate::symmetric::aes::{decrypt_aes256, encrypt_aes256};
use base64::Engine;
use hex::decode;
use num_bigint::BigUint;
use sha2::{Digest, Sha256};
use std::fs;
use std::fs::{read, write};
use std::path::Path;

// ---------- PEM Helper -----------
fn to_pem(label: &str, data: &[u8]) -> String {
    let b64 = base64::engine::general_purpose::STANDARD.encode(data);
    let mut pem = format!("-----BEGIN {}-----\n", label);
    for chunk in b64.as_bytes().chunks(64) {
        pem.push_str(&String::from_utf8_lossy(chunk));
        pem.push('\n');
    }
    pem.push_str(&format!("-----END {}-----\n", label));
    pem
}

fn from_pem(pem: &str) -> Vec<u8> {
    let data: String = pem
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .collect();
    base64::engine::general_purpose::STANDARD
        .decode(&data)
        .expect("Kein gültiges Base64")
}

// ----- Key Writing (PEM) ----------
pub fn write_rsa_keys_pem(pub_key: &PublicKey, priv_key: &PrivateKey, out_dir: &Path) {
    let n_bytes = pub_key.n.to_bytes_be();
    let d_bytes = priv_key.d.to_bytes_be();

    let pub_pem = to_pem("RSA PUBLIC KEY", &n_bytes);
    let priv_pem = to_pem("RSA PRIVATE KEY", &d_bytes);

    write(out_dir.join("rsa_public.key"), pub_pem).unwrap();
    write(out_dir.join("rsa_private.key"), priv_pem).unwrap();
}

// ----- Key Reading (PEM) ----------
pub fn load_rsa_public_key(path: &Path) -> PublicKey {
    let content = fs::read_to_string(path).expect("Fehler beim Lesen des Public Keys");
    let n = from_pem(&content);

    // Standard Public Exponent 65537
    let e = BigUint::from(65537u32);

    PublicKey {
        n: BigUint::from_bytes_be(&n),
        e,
    }
}

pub fn load_rsa_private_key(path: &Path) -> PrivateKey {
    let content = fs::read_to_string(path).expect("Fehler beim Lesen des Private Keys");
    let d = from_pem(&content);

    let pub_content = fs::read_to_string(path.with_file_name("rsa_public.key"))
        .expect("Fehler beim Lesen des Public Keys");
    let n = from_pem(&pub_content);

    PrivateKey {
        n: BigUint::from_bytes_be(&n),
        d: BigUint::from_bytes_be(&d),
    }
}

// ---------- Datei-Verschlüsselung: RSA + AES -----------
pub fn rsa_encrypt_file(input_path: &Path, pub_key_path: &Path, output_path: &Path) {
    let plaintext = read(input_path).expect("Fehler beim Lesen der Eingabedatei");
    let pub_key = load_rsa_public_key(pub_key_path);
    let aes_key: [u8; 32] = rand::random();
    let (iv, ciphertext) = encrypt_aes256(&aes_key, &plaintext);

    let enc_key = encrypt_rsa(&BigUint::from_bytes_be(&aes_key), &pub_key).to_bytes_be();

    let mut out = vec![];
    out.extend_from_slice(&(enc_key.len() as u16).to_be_bytes());
    out.extend_from_slice(&enc_key);
    out.extend_from_slice(&iv);
    out.extend_from_slice(&ciphertext);

    write(output_path, out).expect("Fehler beim Schreiben der Ausgabedatei");
    println!(
        "Datei verschlüsselt gespeichert in: {}",
        output_path.display()
    );
}

// ---------- Datei-Entschlüsselung: RSA + AES -----------
pub fn rsa_decrypt_file(input_path: &Path, priv_key_path: &Path, output_path: &Path) {
    let data = read(input_path).expect("Fehler beim Lesen der Eingabedatei");
    let priv_key = load_rsa_private_key(priv_key_path);

    let key_len = u16::from_be_bytes([data[0], data[1]]) as usize;
    let enc_key = &data[2..2 + key_len];
    let iv = &data[2 + key_len..2 + key_len + 16];
    let ciphertext = &data[2 + key_len + 16..];

    let aes_key_big = decrypt_rsa(&BigUint::from_bytes_be(enc_key), &priv_key);
    let mut aes_key = aes_key_big.to_bytes_be();

    if aes_key.len() < 32 {
        let mut padded = vec![0u8; 32 - aes_key.len()];
        padded.append(&mut aes_key);
        aes_key = padded;
    }

    let aes_key: [u8; 32] = aes_key.try_into().expect("Ungültige AES-Key-Länge");
    let plaintext = decrypt_aes256(&aes_key, iv, ciphertext);

    write(output_path, &plaintext).expect("Fehler beim Schreiben der Ausgabedatei");
    println!(
        "Datei entschlüsselt gespeichert in: {}",
        output_path.display()
    );
}

// ---------- AES File-Only -----------
pub fn aes_encrypt_file(input_path: &Path, key_hex: &str, output_path: &Path) {
    let plaintext = read(input_path).expect("Fehler beim Lesen der Eingabedatei");
    let key = decode(key_hex).expect("Ungültiger Hex-Schlüssel");
    assert_eq!(key.len(), 32, "AES-Schlüssel muss 32 Byte lang sein");
    let key_array: [u8; 32] = key.try_into().unwrap();

    let (iv, ciphertext) = encrypt_aes256(&key_array, &plaintext);

    let mut out = vec![];
    out.extend_from_slice(&iv);
    out.extend_from_slice(&ciphertext);

    write(output_path, out).expect("Fehler beim Schreiben der Ausgabedatei");
    println!("AES-Datei gespeichert in: {}", output_path.display());
}

pub fn aes_decrypt_file(input_path: &Path, key_hex: &str, output_path: &Path) {
    let data = read(input_path).expect("Fehler beim Lesen der Eingabedatei");
    let key = decode(key_hex).expect("Ungültiger Hex-Schlüssel");
    assert_eq!(key.len(), 32, "AES-Schlüssel muss 32 Byte lang sein");
    let key_array: [u8; 32] = key.try_into().unwrap();

    let iv = &data[..16];
    let ciphertext = &data[16..];

    let plaintext = decrypt_aes256(&key_array, iv, ciphertext);
    write(output_path, plaintext).expect("Fehler beim Schreiben der Ausgabedatei");
    println!(
        "Datei erfolgreich entschlüsselt in: {}",
        output_path.display()
    );
}

// ---------- Signieren & Verifizieren -----------
pub fn rsa_sign_file(input_path: &Path, priv_key_path: &Path, sig_path: &Path) {
    let data = read(input_path).expect("Fehler beim Lesen der Datei");
    let hash = Sha256::digest(&data);
    let hash_int = BigUint::from_bytes_be(&hash);

    let priv_key = load_rsa_private_key(priv_key_path);
    let signature = decrypt_rsa(&hash_int, &priv_key);
    let sig_bytes = signature.to_bytes_be();

    write(sig_path, sig_bytes).expect("Fehler beim Schreiben der Signatur");
    println!("Datei signiert: {}", sig_path.display());
}

pub fn rsa_verify_file(input_path: &Path, pub_key_path: &Path, sig_path: &Path) {
    let data = read(input_path).expect("Fehler beim Lesen der Datei");
    let signature = read(sig_path).expect("Fehler beim Lesen der Signatur");
    let hash = Sha256::digest(&data);

    let sig_int = BigUint::from_bytes_be(&signature);
    let pub_key = load_rsa_public_key(pub_key_path);
    let decrypted = encrypt_rsa(&sig_int, &pub_key);
    let hash_int = BigUint::from_bytes_be(&hash);

    if decrypted == hash_int {
        println!("Signatur gültig.");
    } else {
        println!("Signatur ungültig!");
    }
}
