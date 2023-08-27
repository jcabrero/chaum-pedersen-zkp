// Import required libraries and modules
use std::thread;
use std::time::Duration;
use std::env;

use num_bigint::BigInt;
use num_traits::ToPrimitive;

use log::{debug, LevelFilter};

// Import modules from the current crate
use crate::zkrypto::chaum_pedersen::ChaumPedersenProver;

// Define the module structure for the generated proto files
pub mod zkp_auth {
    tonic::include_proto!("zkp_auth"); // The string specified here must match the proto package name
}

// Import the required modules from the generated proto files
use zkp_auth::auth_client::AuthClient;
use zkp_auth::{
    RegisterRequest,
    AuthenticationChallengeRequest,
    AuthenticationAnswerRequest,
};

// Import other modules from the crate
pub mod zkrypto;
pub mod utils;

// Main async function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logging system
    env_logger::init();
    log::set_max_level(LevelFilter::Debug);

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let host: String;
    match args.len() {
        2 => host = format!("https://{}:50051", &args[1]),
        3 => host = format!("https://{}:{}", &args[1], &args[2]),
        _ => host = "https://localhost:50051".to_string(),
    }
    debug!("[C] Waiting for server to come online");
    thread::sleep(Duration::from_secs(3));
    debug!("[C] Starting connection to host {}", host.to_string());

    // Connect to the gRPC server
    let mut client = AuthClient::connect(host).await?;

    // Continuous loop for registration and authentication
    loop {
        thread::sleep(Duration::from_secs(2));

        // Generate a random username
        let username = utils::generate_random_string(32);
        debug!("[C] Starting Registration for {}", username);

        // Initialize the Chaum-Pedersen prover
        let mut prover = ChaumPedersenProver::new();

        // Convert BigInt values to i64 for serialization
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

        // Send registration request to the server
        let _ = client.register(tonic::Request::new(
            RegisterRequest {
                user: username.to_string(),
                y1: y1,
                y2: y2,
            },
        )).await?;

        // Prove the challenge for synchronization A
        let (br1, br2) = prover.prove_sync_a();

        // Convert BigInt values to i64 for serialization
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

        debug!("[C] Requesting Challenge for {}", username);

        // Request challenge from the server
        let challenge_response = client.create_authentication_challenge(tonic::Request::new(
            AuthenticationChallengeRequest {
                user: username.to_string(),
                r1: r1,
                r2: r2,
            },
        )).await?;

        let c = challenge_response.get_ref().c;

        // Prove the challenge for synchronization B
        let bs: BigInt = prover.prove_sync_b(&BigInt::from(c));

        // Convert BigInt value to i64 for serialization
        let s: i64;
        match bs.to_i64() {
            Some(x) => s = x,
            None => panic!("Error casting"),
        }

        debug!("[C] Requesting Authentication for {}", username);

        // Verify authentication response from the server
        let authentication_response = client.verify_authentication(tonic::Request::new(
            AuthenticationAnswerRequest {
                auth_id: challenge_response.get_ref().auth_id.to_string(),
                s: s,
            },
        )).await?;

        // Print authentication response
        println!("Authentication Response {}", authentication_response.get_ref().session_id.to_string());
    }
}
