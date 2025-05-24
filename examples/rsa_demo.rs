use cryptomorph::asymmetric::rsa::{decrypt_rsa, encrypt_rsa, generate_rsa_keypair};
use num_bigint::BigUint;
use std::str;

fn main() {
    println!("🔐 RSA-Demo: Schlüsselpaar wird generiert...");
    let (pub_key, priv_key) = generate_rsa_keypair(512); // für Demo klein gehalten

    let message = "Cryptomorph rockt!";
    let message_int = BigUint::from_bytes_be(message.as_bytes());

    println!("➡️  Klartext als Zahl: {}", &message_int);

    let ciphertext = encrypt_rsa(&message_int, &pub_key);
    println!("🔒 Verschlüsselt: {}", &ciphertext);

    let decrypted = decrypt_rsa(&ciphertext, &priv_key);
    let decrypted_bytes = decrypted.to_bytes_be();
    let decrypted_str = str::from_utf8(&decrypted_bytes).expect("Ungültiges UTF-8");

    println!("✅ Entschlüsselt: {}", decrypted_str);
}
