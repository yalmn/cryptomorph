use cryptomorph::asymmetric::rsa::generate_rsa_keypair;
use cryptomorph::cryptomorph_cli;
use cryptomorph_cli::{
    aes_decrypt_file, aes_encrypt_file, rsa_decrypt_file, rsa_encrypt_file, rsa_sign_file,
    rsa_verify_file, write_rsa_keys_pem,
};
use rand::RngCore;
use std::env;
use std::fs::create_dir_all;
use std::path::Path;

fn print_help() {
    println!("\nCryptomorph CLI – Hybrides Verschlüsselungstool\n");
    println!("Verwendung:");
    println!("  Rsa_Key_Gen <bitlänge> <output-pfad>");
    println!("      → Generiert ein RSA-Schlüsselpaar mit gegebener Bitlänge (z. B. 4096)");
    println!("  rsa_encrypt <input.txt> <rsa_public.key> <output.bin>");
    println!("      → Verschlüsselt Datei mit AES + RSA (hybrid)");
    println!("  rsa_decrypt <input.bin> <rsa_private.key> <output.txt>");
    println!("      → Entschlüsselt Datei mit RSA → AES");
    println!("  aes_encrypt <input.txt> <key.hex> <output.bin>");
    println!("      → Verschlüsselt Datei direkt mit AES-256");
    println!("  aes_decrypt <input.bin> <key.hex> <output.txt>");
    println!("      → Entschlüsselt AES-Datei");
    println!("  rsa_sign <input.txt> <rsa_private.key> <signature.sig>");
    println!("      → Signiert Datei mit privatem Schlüssel");
    println!("  rsa_verify <input.txt> <rsa_public.key> <signature.sig>");
    println!("      → Verifiziert Signatur mit öffentlichem Schlüssel");
    println!("  gen_aes_key");
    println!("      → Generiert zufälligen 256-Bit AES-Schlüssel (Hex)\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        print_help();
        return;
    }

    match args[1].as_str() {
        "Rsa_Key_Gen" if args.len() == 4 => {
            let bits: usize = args[2].parse().expect("Ungültige Bitlänge");
            let out_dir = Path::new(&args[3]);
            create_dir_all(&out_dir).expect("Konnte Ausgabeordner nicht erstellen");

            let (pub_key, priv_key) = generate_rsa_keypair(bits);
            write_rsa_keys_pem(&pub_key, &priv_key, out_dir);

            println!("RSA-Schlüssel gespeichert in: {}", out_dir.display());
        }
        "rsa_encrypt" if args.len() == 5 => {
            let input = Path::new(&args[2]);
            let pub_key = Path::new(&args[3]);
            let output = Path::new(&args[4]);
            rsa_encrypt_file(input, pub_key, output);
        }
        "rsa_decrypt" if args.len() == 5 => {
            let input = Path::new(&args[2]);
            let priv_key = Path::new(&args[3]);
            let output = Path::new(&args[4]);
            rsa_decrypt_file(input, priv_key, output);
        }
        "aes_encrypt" if args.len() == 5 => {
            let input = Path::new(&args[2]);
            let key_hex = &args[3];
            let output = Path::new(&args[4]);
            aes_encrypt_file(input, key_hex, output);
        }
        "aes_decrypt" if args.len() == 5 => {
            let input = Path::new(&args[2]);
            let key_hex = &args[3];
            let output = Path::new(&args[4]);
            aes_decrypt_file(input, key_hex, output);
        }
        "rsa_sign" if args.len() == 5 => {
            let input = Path::new(&args[2]);
            let priv_key = Path::new(&args[3]);
            let signature = Path::new(&args[4]);
            rsa_sign_file(input, priv_key, signature);
        }
        "rsa_verify" if args.len() == 5 => {
            let input = Path::new(&args[2]);
            let pub_key = Path::new(&args[3]);
            let signature = Path::new(&args[4]);
            rsa_verify_file(input, pub_key, signature);
        }
        "gen_aes_key" => {
            let mut key = [0u8; 32];
            rand::thread_rng().fill_bytes(&mut key);
            println!("🔑 Zufälliger AES-256-Schlüssel (Hex):");
            println!("{}", hex::encode(key));
        }
        _ => {
            eprintln!("Ungültige Argumente. Hilfe mit ./cryptomorph --help");
            std::process::exit(1);
        }
    }
}
