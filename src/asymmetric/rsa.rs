use crate::algorithms::number_theory::{is_coprime, mod_inverse};
use crate::algorithms::primality::is_probably_prime;
use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use rand::rngs::OsRng;

#[derive(Debug)]
pub struct PublicKey {
    pub e: BigUint,
    pub n: BigUint,
}

#[derive(Debug)]
pub struct PrivateKey {
    pub d: BigUint,
    pub n: BigUint,
}

/// Generiert ein RSA-Schlüsselpaar mit der gegebenen Bitlänge.
pub fn generate_rsa_keypair(bits: usize) -> (PublicKey, PrivateKey) {
    let one = BigUint::one();
    let (p, q) = loop {
        let p = gen_prime(bits / 2);
        let q = gen_prime(bits / 2);
        if p != q {
            break (p, q);
        }
    };

    let n = &p * &q;
    let phi = (&p - &one) * (&q - &one);

    let e = BigUint::from(65537u32);
    assert!(is_coprime(&e, &phi), "e muss teilerfremd zu φ(n) sein");

    let d = mod_inverse(&e, &phi).expect("Kein modulares Inverses für e gefunden");

    (
        PublicKey {
            e: e.clone(),
            n: n.clone(),
        },
        PrivateKey { d, n },
    )
}

/// RSA-Verschlüsselung: c = m^e mod n
pub fn encrypt_rsa(message: &BigUint, pub_key: &PublicKey) -> BigUint {
    message.modpow(&pub_key.e, &pub_key.n)
}

/// RSA-Entschlüsselung: m = c^d mod n
pub fn decrypt_rsa(ciphertext: &BigUint, priv_key: &PrivateKey) -> BigUint {
    ciphertext.modpow(&priv_key.d, &priv_key.n)
}

/// Generiert eine zufällige Primzahl mit der gewünschten Bitlänge.
fn gen_prime(bits: usize) -> BigUint {
    let mut rng = OsRng;
    loop {
        let mut candidate = rng.gen_biguint(bits.try_into().unwrap());
        // stelle sicher, dass die Zahl ungerade ist
        candidate.set_bit(0, true);
        if is_probably_prime(&candidate, 10) {
            return candidate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigUint;

    #[test]
    fn test_rsa_encrypt_decrypt() {
        let (pub_key, priv_key) = generate_rsa_keypair(512); // klein für Test
        let message = 12345u32.to_biguint().unwrap();

        let ciphertext = encrypt_rsa(&message, &pub_key);
        let decrypted = decrypt_rsa(&ciphertext, &priv_key);

        assert_eq!(message, decrypted);
    }
}
