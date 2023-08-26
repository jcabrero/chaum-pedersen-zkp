use num_bigint::BigInt;

use log::{LevelFilter};

use crate::zkrypto::chaum_pedersen::{ChaumPedersenProver, ChaumPedersenVerifier};

pub mod zkrypto;

fn main() {
    env_logger::init();
    log::set_max_level(LevelFilter::Debug);

    let p  = zkrypto::prime::n_bit_prime(100);
    let (p1, q, g_list) = zkrypto::generator::get_generator_prime(5, 2);
    println!("Prime: {}", p);
    println!("p: {}, q: {}, g: {}, q: {}", p1, q, g_list[0], g_list[1]);
    let mut prover = ChaumPedersenProver::new();
    let mut verifier: ChaumPedersenVerifier = ChaumPedersenVerifier::new(prover.kp.y1.clone(), prover.kp.y2.clone());

    let (r1, r2) = prover.prove_sync_a();

    let c: BigInt = verifier.verify_sync_a(r1, r2);

    let s: BigInt = prover.prove_sync_b(&c);

    let verification = verifier.verify_sync_b(s.clone());

    println!("VERIFICATION: {}", verification);

}

