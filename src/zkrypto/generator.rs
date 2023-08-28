
use num_bigint::{BigInt, RandBigInt};
use num_traits::{Zero, One};

use crate::zkrypto::prime::{miller_rabin_primality_test, n_bit_prime};


//use prime::{miller_rabin_primality_test, n_bit_prime};


#[derive(Clone, Default)]
pub struct Generator {
    pub p: BigInt,
    pub q: BigInt,
    pub g: BigInt,
    pub h: BigInt
}

pub fn get_default() -> Generator {
    return Generator{
        p: BigInt::from(421360559 as i64), 
        q: BigInt::from(455033 as i64), 
        g: BigInt::from(103117051 as i64), 
        h: BigInt::from(322482758 as i64)
    };// Default params
}

pub fn get_debug() -> Generator {
    return Generator{
        p: BigInt::from(367), 
        q: BigInt::from(61), 
        g: BigInt::from(137), 
        h: BigInt::from(199)
    };// Default params
}

pub fn get_generator_prime(bits: u64, num: i32) -> (BigInt, BigInt, Vec<BigInt>) {
    let mut g_list: Vec<BigInt> = vec!(); // Initialize an empty vector to store generator values
    let n: BigInt = BigInt::one() << bits; // Calculate 2 raised to the power of bits
    let q: BigInt = n_bit_prime(bits - 1); // Generate a prime number q with (bits - 1) bits
    
    let mut p: BigInt = BigInt::zero(); // Initialize p with zero
    let mut rng = rand::thread_rng(); // Initialize a random number generator

    // Generate a prime number p such that p = k * q + 1 using a loop
    while !miller_rabin_primality_test(&p, 5) {
        let k = rng.gen_bigint_range(&BigInt::one(), &n); // Generate a random integer k in the range [1, n)
        p = k * &q + 1; // Calculate p = k * q + 1
    }
    
    // Loop to find generator values
    loop {
        let h: BigInt = rng.gen_bigint_range(&BigInt::one(), &n); // Generate a random integer h in the range [1, n)
        let g: BigInt = h.modpow(&((p.clone() - 1) / q.clone()), &p); // Calculate g = h^((p - 1) / q) mod p
        
        if g != BigInt::one() { // Check if g is not equal to 1
            g_list.push(g.clone()); // Add g to the list of generator values
            if g_list.len() as u32 == num as u32 { // Check if the desired number of generators has been found
                return (p, q, g_list); // Return the prime p, prime q, and the list of generator values
            }
        }
    }
}


#[cfg(test)]
mod tests{
    use num_bigint::BigInt;
    use crate::zkrypto::generator::{get_default, get_debug, get_generator_prime}; // Adjust the paths accordingly
    use crate::zkrypto::prime::miller_rabin_primality_test;
    
    #[test]
    fn test_get_default() {
        let generator = get_default();
        assert_eq!(generator.p, BigInt::from(421360559 as i64));
        // Add assertions for other fields
    }
    
    #[test]
    fn test_get_debug() {
        let generator = get_debug();
        assert_eq!(generator.p, BigInt::from(367));
        // Add assertions for other fields
    }
    
    #[test]
    fn test_get_generator_prime() {
        let bits = 64;
        let num = 3;
        let (p, q, g_list) = get_generator_prime(bits, num);
        
        // Add assertions to validate the generated values
        assert!(miller_rabin_primality_test(&p, 5));
        assert!(miller_rabin_primality_test(&q, 5));
        assert_eq!(g_list.len(), num as usize);
    }
    
}