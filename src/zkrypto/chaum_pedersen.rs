use num_bigint::{BigInt, RandBigInt};
use num_traits::{Zero, One};
use log::{debug};
use std::fmt::Debug;

use crate::zkrypto::generator::{Generator, get_default};

#[derive(Default)]
pub struct KeyPair{
    x: BigInt,
    pub y1: BigInt,
    pub y2: BigInt
}



impl KeyPair {
    fn new(g: &BigInt, h: &BigInt, p: &BigInt) -> KeyPair{
        let mut rng = rand::thread_rng();
        let x = rng.gen_bigint_range(&BigInt::one(), p);
        let y1 = g.modpow(&x, p);
        let y2 = h.modpow(&x, p);
        return KeyPair{
            x: x,
            y1: y1,
            y2: y2
        };
    }
}

#[derive(Default)]
pub struct ChaumPedersenProver {
    g: Generator,
    pub kp: KeyPair,
    k: BigInt,
    r1: BigInt,
    r2: BigInt,
    c: BigInt,
    s: BigInt,
}

impl Debug for ChaumPedersenProver {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ChaumPedersenProver <-")
    }
}

impl ChaumPedersenProver {
    pub fn new() -> ChaumPedersenProver {
        let g = get_default();
        //let g = get_debug();
        let kp = KeyPair::new(&g.g, &g.h, &g.p);

        return ChaumPedersenProver{
            g: g,
            kp: kp,
            k: BigInt::zero(),
            r1: BigInt::zero(),
            r2: BigInt::zero(),
            c: BigInt::zero(),
            s: BigInt::zero(),
        }
        
    }
    pub fn prove_sync_a(&mut self) -> (BigInt, BigInt){
        debug!("[P] PROVE SYNC A");
        let mut rng = rand::thread_rng();

        self.k =  rng.gen_bigint_range(&BigInt::one(), &(self.g.p.clone()- 2));
        self.r1 = self.g.g.clone().modpow(&self.k, &self.g.p);
        self.r2 = self.g.h.clone().modpow(&self.k, &self.g.p);
        
        debug!("[P] y1: {}, y2:{}", &self.kp.y1, &self.kp.y2);
        debug!("[P] r1: {}, r2:{}", &self.r1, &self.r2);
        debug!("[P] END PROVE SYNC A");
        return (self.r1.clone(), self.r2.clone());
    }

    pub fn prove_sync_b(&mut self, c: &BigInt) -> BigInt{
        debug!("[P] PROVE SYNC B");
        self.c = c.clone();
        /*let t0 = self.kp.x.clone() * self.c.clone();
        let t1 = t0.clone() % self.g.q.clone();
        let t2 = self.k.clone() - t1.clone(); 
        self.s = t2.clone().modpow(&BigInt::one(), &self.g.q);
        debug!("[P] t0: {}, t1: {}, t2: {}, t3 {}", t0, t1, t2, self.s);*/
        self.s = (self.k.clone() - ((self.kp.x.clone() * self.c.clone()) % self.g.q.clone())).modpow(&BigInt::one(), &self.g.q);
        debug!("[P] c: {}, s:{}", &self.c, &self.s);
        debug!("[P] END PROVE SYNC B");
        return self.s.clone();
    }

}




#[derive(Default)]
pub struct ChaumPedersenVerifier {
    g : Generator,
    y1: BigInt,
    y2: BigInt,
    r1: BigInt,
    r2: BigInt,
    c: BigInt,
    s: BigInt,

}

impl Debug for ChaumPedersenVerifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ChaumPedersenVerifier <-")
    }
}

impl ChaumPedersenVerifier {
    pub fn new(y1: BigInt, y2: BigInt) -> ChaumPedersenVerifier {
        let g = get_default();
        //let g = get_debug();
        return ChaumPedersenVerifier{
            g: g,
            y1: y1.clone(),
            y2: y2.clone(),
            r1: BigInt::zero(),
            r2: BigInt::zero(),
            c: BigInt::zero(),
            s: BigInt::zero(),
        }
        
    }

    pub fn verify_sync_a(&mut self, r1: BigInt, r2: BigInt) -> BigInt{
        debug!("[V] VERIFY SYNC A");
        let mut rng = rand::thread_rng();
        

        self.c =  rng.gen_bigint_range(&BigInt::one(), &(self.g.p.clone()));
        self.r1 = r1;
        self.r2 = r2;
        debug!("[V] y1: {}, y2:{}", &self.y1, &self.y2);
        debug!("[V] r1: {}, r2:{}", &self.r1, &self.r2);
        debug!("[V] END VERIFY SYNC A");
        return self.c.clone();
    }

    pub fn verify_sync_b(&mut self, s:BigInt) -> bool {
        debug!("[V] VERIFY SYNC B");
        self.s = s;
        debug!("[V] c: {} s:{}", &self.c, &self.s);
        let r1_prime: BigInt = (
                                self.g.g.modpow(&self.s, &self.g.p) * 
                                self.y1.modpow(&self.c, &self.g.p)
                            ) % self.g.p.clone();
        let r2_prime: BigInt = (
                                self.g.h.modpow(&self.s, &self.g.p) * 
                                self.y2.modpow(&self.c, &self.g.p)
                            ) % self.g.p.clone();

        debug!("[V] END VERIFY SYNC B");                          
        return r1_prime == self.r1 && r2_prime == self.r2;
    }


}

#[cfg(test)]
mod test{
    use crate::zkrypto::generator::get_default;
    use crate::zkrypto::chaum_pedersen::{KeyPair, ChaumPedersenProver, ChaumPedersenVerifier};

    #[test]
    fn test_keypair_creation() {
        let g = get_default().g;
        let h = get_default().h;
        let p = get_default().p;

        let keypair = KeyPair::new(&g, &h, &p);

        let y1_expected = g.modpow(&keypair.x, &p);
        let y2_expected = h.modpow(&keypair.x, &p);

        assert_eq!(keypair.y1, y1_expected);
        assert_eq!(keypair.y2, y2_expected);
    }

    #[test]
    fn test_chaum_pedersen_protocol() {
        let mut prover = ChaumPedersenProver::new();
        let mut verifier = ChaumPedersenVerifier::new(prover.kp.y1.clone(), prover.kp.y2.clone());

        // Prover's side
        let (r1, r2) = prover.prove_sync_a();
        let c = verifier.verify_sync_a(r1.clone(), r2.clone());
        let s = prover.prove_sync_b(&c);
        let result = verifier.verify_sync_b(s);

        assert_eq!(result, true);
    }
}