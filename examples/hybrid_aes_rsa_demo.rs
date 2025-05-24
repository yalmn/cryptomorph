use cryptomorph::asymmetric::rsa::{decrypt_rsa, encrypt_rsa, generate_rsa_keypair};
use cryptomorph::symmetric::aes::{decrypt_aes256, encrypt_aes256};
use num_bigint::BigUint;
use rand::Rng;
use std::str;

fn main() {
    println!("🔐 Hybrid-Verschlüsselung (AES + RSA-4096)");

    // 1. Nachricht
    let message = "Dies ist eine sehr geheime Nachricht.";
    println!("📨 Ursprünglicher Klartext:\n{}\n", message);

    // 2. Generiere AES-Schlüssel
    let aes_key: [u8; 32] = rand::thread_rng().gen();

    // 3. AES-Verschlüsselung
    let (iv, ciphertext) = encrypt_aes256(&aes_key, message.as_bytes());
    println!("🧊 AES-ciphertext (hex):\n{}\n", hex::encode(&ciphertext));

    // 4. RSA-Schlüsselpaar (4096 Bit)
    println!("🔐 Generiere RSA-4096-Schlüsselpaar...");
    let (pub_key, priv_key) = generate_rsa_keypair(4096);
    println!("✅ RSA-Schlüssel generiert.\n");

    use base64::{engine::general_purpose, Engine as _};

    // Gib RSA Public & Private Keys als Base64 aus
    let n_b64 = general_purpose::STANDARD.encode(pub_key.n.to_bytes_be());
    let e_b64 = general_purpose::STANDARD.encode(pub_key.e.to_bytes_be());
    let d_b64 = general_purpose::STANDARD.encode(priv_key.d.to_bytes_be());

    println!("🔐 Öffentlicher Schlüssel:");
    println!(
        "  n (modulus): {}\n  e (public exponent): {}\n",
        &n_b64, &e_b64
    );

    println!("🗝️ Privater Schlüssel:");
    println!("  d (private exponent): {}\n", &d_b64);

    // 5. Verschlüssele AES-Key mit RSA
    let aes_key_bigint = BigUint::from_bytes_be(&aes_key);
    let encrypted_key = encrypt_rsa(&aes_key_bigint, &pub_key);
    println!("🔑 AES-Key verschlüsselt mit RSA.");

    // 6. Entschlüsselung des AES-Schlüssels
    let decrypted_key = decrypt_rsa(&encrypted_key, &priv_key);
    let aes_key_restored = decrypted_key
        .to_bytes_be()
        .try_into()
        .expect("Falsche AES-Key-Länge");
    println!("🔓 AES-Key erfolgreich entschlüsselt.");

    // 7. Entschlüsselung der Nachricht mit AES
    let decrypted = decrypt_aes256(&aes_key_restored, &iv, &ciphertext);
    let decrypted_str = str::from_utf8(&decrypted).expect("Ungültiges UTF-8");

    println!("\n✅ Entschlüsselter Text:\n{}", decrypted_str);
}
