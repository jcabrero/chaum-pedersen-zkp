
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

pub fn get_generator_prime(bits: u64, num: i32) -> (BigInt, BigInt, Vec<BigInt>){
    let mut g_list: Vec<BigInt> = vec!();
    let n : BigInt = BigInt::one() << bits;
    let q : BigInt = n_bit_prime(bits - 1);
    
    let mut p: BigInt = BigInt::zero();
    let mut rng = rand::thread_rng();

    while !miller_rabin_primality_test(&p, 5) {
        let k = rng.gen_bigint_range(&BigInt::one(), &n);
        p = k * &q + 1; // Find aprime such that k * q + 1
    }
    loop {
        let h: BigInt = rng.gen_bigint_range(&BigInt::one(), &n);
        let g: BigInt = h.modpow(&((p.clone() - 1)/q.clone()), &p);
        if g != BigInt::one() {
            g_list.push(g.clone());
            if g_list.len() as u32 == num as u32 {
                return (p, q, g_list);
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