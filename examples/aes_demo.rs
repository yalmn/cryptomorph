use cryptomorph::symmetric::aes::{decrypt_aes256, encrypt_aes256};
use rand::Rng;
use std::str;

fn main() {
    let key: [u8; 32] = rand::thread_rng().gen();
    let message = "Hallo von Cryptomorph!";
    println!("🔐 Ursprünglicher Text:\n{}\n", message);

    let (iv, ciphertext) = encrypt_aes256(&key, message.as_bytes());
    println!(
        "🧊 Verschlüsselter Text (hex):\n{}\n",
        hex::encode(&ciphertext)
    );
    println!("IV (hex): {}\n", hex::encode(&iv));

    let decrypted = decrypt_aes256(&key, &iv, &ciphertext);
    let decrypted_str = str::from_utf8(&decrypted).expect("Ungültiges UTF-8");

    println!("✅ Entschlüsselter Text:\n{}", decrypted_str);
}
