use num_bigint::BigInt;
use num_traits::ToPrimitive;


use log::LevelFilter;

use crate::zkrypto::chaum_pedersen::ChaumPedersenProver;

pub mod zkrypto;
pub mod utils;

use zkp_auth::auth_client::AuthClient;
use zkp_auth::{RegisterRequest,  
    AuthenticationChallengeRequest,
    AuthenticationAnswerRequest};

pub mod zkp_auth {
    tonic::include_proto!("zkp_auth"); // The string specified here must match the proto package name
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    log::set_max_level(LevelFilter::Debug);

    let username = utils::generate_random_string(32);
    let mut client = AuthClient::connect("https://localhost:50051").await?;

    let mut prover = ChaumPedersenProver::new();


    let y1: i64;
    let y2: i64;

    match prover.kp.y1.to_i64() {
        Some(x) => y1 = x,
        None => panic!("Error casting"),
    }

    match prover.kp.y2.to_i64() {
        Some(x) => y2 = x,
        None => panic!("Error casting"),
    }

    let _ = client.register(tonic::Request::new(
        RegisterRequest{
            user: username.to_string(),
            y1: y1,
            y2: y2,
        })
    ).await?;

    let (br1, br2) = prover.prove_sync_a();

    let r1: i64;
    let r2: i64;

    match br1.to_i64() {
        Some(x) => r1 = x,
        None => panic!("Error casting"),
    }

    match br2.to_i64() {
        Some(x) => r2 = x,
        None => panic!("Error casting"),
    }


    let challenge_response = client.create_authentication_challenge(tonic::Request::new(
        AuthenticationChallengeRequest{
            user: username.to_string(),
            r1: r1,
            r2: r2,
        })
    ).await?;

    let c = challenge_response.get_ref().c;

    
    let bs: BigInt = prover.prove_sync_b(&BigInt::from(c));

    let s: i64;
    match bs.to_i64() {
        Some(x) => s = x,
        None => panic!("Error casting"),
    }
    let authentication_response = client.verify_authentication(tonic::Request::new(
        AuthenticationAnswerRequest{
            auth_id: challenge_response.get_ref().auth_id.to_string(),
            s: s,
        })
    ).await?;

    println!("Authentication Response {}", authentication_response.get_ref().session_id.to_string());
    Ok(())

}

