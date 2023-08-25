use rand::Rng;
use num_bigint::{BigInt, ToBigInt, RandBigInt};
use num_traits::One;

pub fn miller_rabin_primality_test(n: i64, k: i64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    
    // Write n - 1 as (2^s) * d where d is odd
    let mut s: i64 = 0;
    let mut d: i64 = n-1;
    while d & 1 == 0{
        s += 1;
        d >>= 1;
    }

    // Witness loop
    let mut rng = rand::thread_rng();
    for _ in 0..k {
        // Choose a random number

        let a: BigInt = rng.gen_bigint_range(&2.to_bigint().unwrap(), &(n-1).to_bigint().unwrap());

        let mut x: BigInt = a.modpow(&BigInt::from(d), &BigInt::from(n));  // Compute a^d % n
        if x.eq(&BigInt::one()) || x.eq(&BigInt::from(n-1)) {
            continue;
        }

        // Check whether it is a squared base.
        for _ in 0..(s - 1) {
            x = x.modpow(&BigInt::from(2), &BigInt::from(n));  // Square x and take the modulo
            if x.eq(&BigInt::from(n-1)) {
                break;
            }
        }
        if x.ne(&BigInt::from(n-1)){
            return false;  // n is composite
        }
    }

    return true;  // n is probably prime
}

pub fn n_bit_prime() -> Result<i64, ()> {
    let mut rng = rand::thread_rng();
    let mut candidate: i64;
    loop { // Instead of while true {}
        candidate = rng.gen();
        candidate = candidate | 1;
        if miller_rabin_primality_test(candidate, 5) {
            return Result::Ok(candidate);
        }
    }
}