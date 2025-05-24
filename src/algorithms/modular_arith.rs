use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Schnelle modulare Exponentiation (Square-and-Multiply)
/// für beliebig große Zahlen.
///
/// Gibt (base^exponent mod modulus) zurück.
pub fn mod_exp(mut base: BigUint, mut exponent: BigUint, modulus: &BigUint) -> BigUint {
    let one = BigUint::one();
    let mut result = one.clone();
    base %= modulus;

    while !exponent.is_zero() {
        if &exponent % 2u32 == one {
            result = (result * &base) % modulus;
        }
        exponent >>= 1;
        base = (&base * &base) % modulus;
    }

    result
}

/// ======= Tests =======

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;
    use num_traits::FromPrimitive;

    #[test]
    fn test_small_numbers() {
        let base = BigUint::from_u64(4).unwrap();
        let exp = BigUint::from_u64(13).unwrap();
        let modulus = BigUint::from_u64(497).unwrap();
        assert_eq!(
            mod_exp(base, exp, &modulus),
            BigUint::from_u64(445).unwrap()
        );
    }

    #[test]
    fn test_large_exponent() {
        let base = BigUint::from_u64(2).unwrap();
        let exp = BigUint::from_u64(1024).unwrap();
        let modulus = BigUint::from_u64(97).unwrap();
        let result = mod_exp(base, exp, &modulus);
        assert!(result < modulus); // mod-Ergebnis muss < modulus sein
    }
}
