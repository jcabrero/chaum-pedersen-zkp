
use num_bigint::{BigInt, RandBigInt};
use num_traits::{Zero, One};
use num_iter;

pub fn miller_rabin_primality_test(n: &BigInt, k: i64) -> bool {
    if n <= &BigInt::one() {
        return false;
    }
    if n <= &BigInt::from(3) {
        return true;
    }
    
    let n_ : BigInt = n.clone() - 1;
    // Write n - 1 as (2^s) * d where d is odd
    let mut s: BigInt = BigInt::zero();
    let mut d: BigInt = n_.clone();
    while (&d & BigInt::one()) == BigInt::zero(){ // d & 1 == 0
        s += 1;
        d >>= 1;
    }

    
    // Witness loop
    let mut rng = rand::thread_rng();
    for _ in 0..k {
        // Choose a random number

        let a: BigInt = rng.gen_bigint_range(&BigInt::from(2), &n_);

        let mut x: BigInt = a.modpow(&d, &n);  // Compute a^d % n
        if x.eq(&BigInt::one()) || x.eq(&n_) {
            continue;
        }

        // Check whether it is a squared base.
        for _ in num_iter::range(BigInt::zero(), &s - 1) {
            x = x.modpow(&BigInt::from(2), &n.clone());  // Square x and take the modulo
            if x.eq(&n_) {
                break;
            }
        }
        if x.ne(&BigInt::from(n-1)){
            return false;  // n is composite
        }
    }

    return true;  // n is probably prime
}

pub fn n_bit_prime(bits: u64) -> BigInt {
    let mut rng = rand::thread_rng();
    let mut candidate: BigInt;
    loop { // Instead of while true {}
        candidate = rng.gen_bigint(bits);
        candidate = candidate | BigInt::one();
        if miller_rabin_primality_test(&candidate, 5) {
            return candidate;
        }
    }
}

#[cfg(test)]
mod test{
    use num_bigint::BigInt;
    use super::{miller_rabin_primality_test, n_bit_prime};

    #[test]
    fn test_miller_rabin_primality_test() {
        let prime = BigInt::from(29); // A prime number
        let non_prime = BigInt::from(30); // A non-prime number

        assert!(miller_rabin_primality_test(&prime, 5));
        assert!(!miller_rabin_primality_test(&non_prime, 5));
    }

    #[test]
    fn test_n_bit_prime() {
        let bits = 32;
        let prime = n_bit_prime(bits);

        assert!(miller_rabin_primality_test(&prime, 5));
        //assert!(prime.bits() > bits);
    }
}