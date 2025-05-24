use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_integer::Integer;
use num_traits::{One, Zero};
use rand::thread_rng;

/// Miller-Rabin-Primzahltest für beliebig große `n`.
///
/// Gibt `true` zurück, wenn `n` mit hoher Wahrscheinlichkeit prim ist.
pub fn is_probably_prime(n: &BigUint, k: u32) -> bool {
    let one = BigUint::one();
    let two = &one + &one;

    if n == &two || n == &(3u32.to_biguint().unwrap()) {
        return true;
    }
    if n < &two || n.is_even() {
        return false;
    }

    // Schreibe n-1 als 2^r * d
    let mut d = n - &one;
    let mut r = 0u32;
    while &d % &two == Zero::zero() {
        d /= &two;
        r += 1;
    }

    let mut rng = thread_rng();

    'witness_loop: for _ in 0..k {
        let a = rng.gen_biguint_range(&two, &(n - &two));
        let mut x = a.modpow(&d, n);

        if x == one || x == (n - &one) {
            continue 'witness_loop;
        }

        for _ in 0..r - 1 {
            x = x.modpow(&two, n);
            if x == (n - &one) {
                continue 'witness_loop;
            }
        }

        return false;
    }

    true
}

/// ======= Tests =======

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigUint;

    #[test]
    fn test_small_primes() {
        assert!(is_probably_prime(&3u32.to_biguint().unwrap(), 5));
        assert!(is_probably_prime(&13u32.to_biguint().unwrap(), 5));
    }

    #[test]
    fn test_small_composites() {
        assert!(!is_probably_prime(&15u32.to_biguint().unwrap(), 5));
        assert!(!is_probably_prime(&221u32.to_biguint().unwrap(), 5)); // 13×17
    }

    #[test]
    fn test_large_prime_candidate() {
        let p = BigUint::parse_bytes(b"32416190071", 10).unwrap(); // bekannte 11-stellige Primzahl
        assert!(is_probably_prime(&p, 10));
    }
}
