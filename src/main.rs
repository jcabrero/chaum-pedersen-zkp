use num_bigint::BigInt; // Import the BigInt type for arbitrary precision integers

use log::LevelFilter; // Import log level filters for logging

use crate::zkrypto::chaum_pedersen::{ChaumPedersenProver, ChaumPedersenVerifier}; // Import necessary components from the chaum_pedersen module

pub mod zkrypto; // Import the zkrypto module

fn main() {
    env_logger::init(); // Initialize the logger for logging messages
    log::set_max_level(LevelFilter::Debug); // Set the maximum logging level to Debug

    // Generate a 100-bit prime number
    let p = zkrypto::prime::n_bit_prime(100);

    // Generate prime p1, prime q, and a list of generator values
    let (p1, q, g_list) = zkrypto::generator::get_generator_prime(5, 2);

    // Print the generated prime and generator values
    println!("Prime: {}", p);
    println!("p: {}, q: {}, g: {}, q: {}", p1, q, g_list[0], g_list[1]);

    // Create a new instance of ChaumPedersenProver
    let mut prover = ChaumPedersenProver::new();

    // Create a new instance of ChaumPedersenVerifier using prover's public keys
    let mut verifier: ChaumPedersenVerifier = ChaumPedersenVerifier::new(prover.kp.y1.clone(), prover.kp.y2.clone());

    // Prover: Prove part A of the protocol and receive response values r1 and r2
    let (r1, r2) = prover.prove_sync_a();

    // Verifier: Verify part A of the protocol using received response values r1 and r2, and get challenge value c
    let c: BigInt = verifier.verify_sync_a(r1, r2);

    // Prover: Prove part B of the protocol using the challenge value c, and receive solution value s
    let s: BigInt = prover.prove_sync_b(&c);

    // Verifier: Verify part B of the protocol using the received solution value s, and get verification result
    let verification = verifier.verify_sync_b(s.clone());

    // Print the authentication result
    println!("VERIFICATION: {}", verification);
}
