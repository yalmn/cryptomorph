use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{One, Zero};

/// Berechnet den größten gemeinsamen Teiler (ggT) von a und b
/// mithilfe des euklidischen Algorithmus.
pub fn gcd(mut a: BigUint, mut b: BigUint) -> BigUint {
    while !b.is_zero() {
        let tmp = b.clone();
        b = a % &tmp;
        a = tmp;
    }
    a
}

/// Erweiterter euklidischer Algorithmus:
/// Liefert (g, x, y) mit a*x + b*y = g = gcd(a, b)
pub fn extended_gcd(a: &BigUint, b: &BigUint) -> (BigUint, BigInt, BigInt) {
    let zero = BigInt::zero();
    let one = BigInt::one();

    let (mut old_r, mut r) = (a.to_bigint().unwrap(), b.to_bigint().unwrap());
    let (mut old_s, mut s) = (one.clone(), zero.clone());
    let (mut old_t, mut t) = (zero.clone(), one.clone());

    while !r.is_zero() {
        let q = &old_r / &r;

        let tmp_r = old_r.clone();
        old_r = r.clone();
        r = tmp_r - &q * &r;

        let tmp_s = old_s.clone();
        old_s = s.clone();
        s = tmp_s - &q * &s;

        let tmp_t = old_t.clone();
        old_t = t.clone();
        t = tmp_t - &q * &t;
    }

    (old_r.to_biguint().unwrap(), old_s, old_t)
}

/// Berechnet das modulare Inverse von a modulo m, falls existent.
/// Gibt Some(invers) zurück, sonst None.
pub fn mod_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    let (g, x, _) = extended_gcd(a, m);
    if g != BigUint::one() {
        return None;
    }
    let mut x = x % &m.to_bigint().unwrap();
    if x < BigInt::zero() {
        x += &m.to_bigint().unwrap();
    }
    Some(x.to_biguint().unwrap())
}

/// Kleinstes gemeinsames Vielfaches (kgV) von a und b:
/// lcm(a, b) = a * b / gcd(a, b)
pub fn lcm(a: &BigUint, b: &BigUint) -> BigUint {
    let g = gcd(a.clone(), b.clone());
    (a * b) / g
}

/// Prüft, ob a und b teilerfremd sind (gcd == 1).
pub fn is_coprime(a: &BigUint, b: &BigUint) -> bool {
    gcd(a.clone(), b.clone()) == BigUint::one()
}

/// Euler’sche Phi-Funktion φ(n): Anzahl der 1 ≤ k < n mit gcd(k, n) = 1.
/// Einfache Faktorisierung für Demonstrationszwecke.
pub fn totient(n: &BigUint) -> BigUint {
    let one = BigUint::one();
    let mut result = n.clone();
    let mut nn = n.clone();
    let mut p = BigUint::from(2u32);

    while &p * &p <= nn {
        if (&nn % &p).is_zero() {
            // p ist Faktor
            result = (&result / &p) * (&p - &one);
            while (&nn % &p).is_zero() {
                nn /= &p;
            }
        }
        p += &one;
    }
    if nn > one {
        // Restfaktor
        result = (&result / &nn) * (&nn - &one);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigUint;

    #[test]
    fn test_gcd_small() {
        assert_eq!(
            gcd(12u32.to_biguint().unwrap(), 8u32.to_biguint().unwrap()),
            4u32.to_biguint().unwrap()
        );
        assert_eq!(
            gcd(17u32.to_biguint().unwrap(), 31u32.to_biguint().unwrap()),
            1u32.to_biguint().unwrap()
        );
    }

    #[test]
    fn test_extended_gcd() {
        let a = 240u32.to_biguint().unwrap();
        let b = 46u32.to_biguint().unwrap();
        let (g, x, y) = extended_gcd(&a, &b);
        let lhs = a.to_bigint().unwrap() * x + b.to_bigint().unwrap() * y;
        assert_eq!(lhs, g.to_bigint().unwrap());
        assert_eq!(g, 2u32.to_biguint().unwrap());
    }

    #[test]
    fn test_mod_inverse() {
        let a = 3u32.to_biguint().unwrap();
        let m = 11u32.to_biguint().unwrap();
        let inv = mod_inverse(&a, &m).unwrap();
        assert_eq!((a.clone() * inv.clone()) % m.clone(), BigUint::one());

        // kein Inverses
        let a = 6u32.to_biguint().unwrap();
        let m = 9u32.to_biguint().unwrap();
        assert!(mod_inverse(&a, &m).is_none());
    }

    #[test]
    fn test_lcm_and_coprime() {
        let a = 12u32.to_biguint().unwrap();
        let b = 15u32.to_biguint().unwrap();
        assert!(is_coprime(
            &7u32.to_biguint().unwrap(),
            &20u32.to_biguint().unwrap()
        ));
        assert_eq!(lcm(&a, &b), 60u32.to_biguint().unwrap());
    }

    #[test]
    fn test_totient() {
        assert_eq!(
            totient(&1u32.to_biguint().unwrap()),
            1u32.to_biguint().unwrap()
        );
        assert_eq!(
            totient(&9u32.to_biguint().unwrap()),
            6u32.to_biguint().unwrap()
        );
        assert_eq!(
            totient(&10u32.to_biguint().unwrap()),
            4u32.to_biguint().unwrap()
        );
    }
}
