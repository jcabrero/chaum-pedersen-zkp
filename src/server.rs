// Import required libraries and modules
use std::collections::HashMap;
use std::env;

use num_bigint::BigInt;
use num_traits::ToPrimitive;

use log::{debug, LevelFilter};

// Import modules from the current crate
use crate::zkrypto::chaum_pedersen::ChaumPedersenVerifier;

// Define the module structure for the generated proto files
pub mod zkp_auth {
    tonic::include_proto!("zkp_auth"); // The string specified here must match the proto package name
}

// Import other modules from the crate
pub mod zkrypto;
pub mod utils;

// Import required modules from the Tonic crate
use tonic::{transport::Server, Request, Response, Status};
use std::sync::{Arc, Mutex};

// Import generated proto modules
use zkp_auth::auth_server::{Auth, AuthServer};
use zkp_auth::{
    RegisterRequest,
    RegisterResponse,
    AuthenticationChallengeRequest,
    AuthenticationChallengeResponse,
    AuthenticationAnswerRequest,
    AuthenticationAnswerResponse,
};

// Main state struct to hold verifier, auth ID, and session data
#[derive(Debug, Default)]
pub struct MyAuthState {
    verifiers: Mutex<HashMap<String, ChaumPedersenVerifier>>,
    auth_ids: Mutex<HashMap<String, String>>,
    sessions: Mutex<HashMap<String, String>>,
}

// Main implementation struct for the server
#[derive(Debug, Default)]
pub struct MyAuth {
    state: Arc<MyAuthState>,
}

impl MyAuth {
    // Constructor for the MyAuth struct
    fn new() -> MyAuth {
        MyAuth {
            state: Arc::new(MyAuthState {
                verifiers: std::sync::Mutex::new(HashMap::new()),
                auth_ids: std::sync::Mutex::new(HashMap::new()),
                sessions: std::sync::Mutex::new(HashMap::new()),
            }),
        }
    }
}

// Implementation of the Auth trait for the server
#[tonic::async_trait]
impl Auth for MyAuth {
    // Implementation of the register function
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let reply = RegisterResponse {};
        let mut verifiers: std::sync::MutexGuard<'_, HashMap<String, ChaumPedersenVerifier>> =
            self.state.verifiers.lock().unwrap();
        let username = request.get_ref().user.to_string();
        debug!("[S] Requesting Registration for {}", username.to_string());
        if let Some(_) = verifiers.get(&username) {
            // User exists
            return Err(Status::already_exists(format!(
                "User Already Exists {}",
                username.to_string()
            )));
        } else {
            // User doesn't exist
            verifiers.insert(
                username.clone(),
                ChaumPedersenVerifier::new(
                    BigInt::from(request.get_ref().y1),
                    BigInt::from(request.get_ref().y2),
                ),
            );
        }
        return Ok(Response::new(reply));
    }

    // Implementation of the create_authentication_challenge function
    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        let username = request.get_ref().user.to_string();
        debug!("[S] Requesting Challenge for {}", username.to_string());
        let mut verifiers: std::sync::MutexGuard<'_, HashMap<String, ChaumPedersenVerifier>> =
            self.state.verifiers.lock().unwrap();
        if let Some(verifier) = verifiers.get_mut(&username) {
            // User exists

            let auth_id = utils::generate_random_string(32);
            let c: BigInt = verifier.verify_sync_a(
                BigInt::from(request.get_ref().r1),
                BigInt::from(request.get_ref().r2),
            );
            let mut auth_ids: std::sync::MutexGuard<'_, HashMap<String, String>> =
                self.state.auth_ids.lock().unwrap();
            auth_ids.insert(auth_id.to_string(), username.to_string());

            match c.to_i64() {
                Some(x) => return Ok(Response::new(AuthenticationChallengeResponse { auth_id, c: x })),
                None => {
                    return Err(Status::aborted(format!(
                        "Wrong format for number {}",
                        username.to_string()
                    )))
                }
            }
        } else {
            return Err(Status::not_found(format!(
                "User Already Exists {}",
                username.to_string()
            )));
        }
    }

    // Implementation of the verify_authentication function
    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status>{
        let auth_id =  request.get_ref().auth_id.to_string();
        let mut auth_ids: std::sync::MutexGuard<'_, HashMap<String, String>> = self.state.auth_ids.lock().unwrap();
        if let Some(username) = auth_ids.clone().get_mut(&auth_id) {
            debug!("[S] Requesting Authentication for {}", username.to_string());
            auth_ids.remove(&auth_id);
            let mut verifiers: std::sync::MutexGuard<'_, HashMap<String, ChaumPedersenVerifier>> = self.state.verifiers.lock().unwrap();
            if let Some(verifier) = verifiers.get_mut(&username.to_string()){
                // User exists 
            
                let verification_result: bool = verifier.verify_sync_b(
                    BigInt::from(request.get_ref().s)
                );
                if verification_result {
                    let session_id = utils::generate_random_string(32);
                    let reply: AuthenticationAnswerResponse = AuthenticationAnswerResponse {
                        session_id: session_id.to_string()
                    };
                    let mut sessions: std::sync::MutexGuard<'_, HashMap<String, String>> = self.state.sessions.lock().unwrap();
                    sessions.insert(session_id.to_string(), username.to_string());
                    return Ok(Response::new(reply));
                } else {
                    return Err(
                        Status::permission_denied(
                            format!("Verification Unsuccesful {}", username.to_string())
                        )
                    );
                }
            } else {
                return Err(Status::not_found(
                    format!("User for Auth_ID Not Found {} - {}", 
                    auth_id.to_string(), 
                    username.to_string()
                ))
            );
            }
        } else {
            return Err(Status::not_found(format!("Auth_ID Not Found {}", auth_id.to_string())));
        }

        
        
    }
}
// Main async function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logging system
    env_logger::init();
    log::set_max_level(LevelFilter::Debug);

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let host: String;

    // Determine the host
    match args.len() {
        2 => host = format!("0.0.0.0:{}", &args[1]),
        _ => host = "0.0.0.0:50051".to_string(),
    }

    // Parse the address
    let addr = host.parse()?;

    // Create a new instance of MyAuth
    let auth = MyAuth::new();
    debug!("[S] Serving on host: {}", host);

    // Start the gRPC server
    Server::builder()
        .add_service(AuthServer::new(auth))
        .serve(addr)
        .await?;

    Ok(())
}