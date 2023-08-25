
pub mod zkrypto;

fn main() {
    let p = zkrypto::prime::n_bit_prime().expect("msg");
    println!("Prime: {}", p);
}

