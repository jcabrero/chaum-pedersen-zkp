use std::collections::HashMap;

use num_bigint::BigInt;
use num_traits::ToPrimitive;


use log::LevelFilter;

use crate::zkrypto::chaum_pedersen::ChaumPedersenVerifier;

pub mod zkrypto;
pub mod utils;

use tonic::{transport::Server, Request, Response, Status};
use std::sync::{Arc, Mutex};

use zkp_auth::auth_server::{Auth, AuthServer };
use zkp_auth::{RegisterRequest, 
    RegisterResponse, 
    AuthenticationChallengeRequest,
    AuthenticationChallengeResponse,
    AuthenticationAnswerRequest, 
    AuthenticationAnswerResponse};

pub mod zkp_auth {
    tonic::include_proto!("zkp_auth"); // The string specified here must match the proto package name
}
#[derive(Debug, Default)]
pub struct MyAuthState{
    verifiers: Mutex<HashMap<String, ChaumPedersenVerifier>>,
    auth_ids: Mutex<HashMap<String, String>>,
    sessions: Mutex<HashMap<String, String>>,
}

#[derive(Debug, Default)]
pub struct MyAuth {
    state: Arc<MyAuthState>,
}

impl MyAuth{
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

#[tonic::async_trait]
impl Auth for MyAuth{

    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status>{
        let reply = RegisterResponse {};
        let mut verifiers: std::sync::MutexGuard<'_, HashMap<String, ChaumPedersenVerifier>> = self.state.verifiers.lock().unwrap();
        let username =  request.get_ref().user.to_string();
        if let Some(_) = verifiers.get(&username) {
            // User exists 
            return Err(Status::already_exists(format!("User Already Exists {}", username.to_string())));
        } else {
            // User doesn't exist
            verifiers.insert(username, 
                ChaumPedersenVerifier::new(
                    BigInt::from(request.get_ref().y1), 
                    BigInt::from(request.get_ref().y2))
                );
        }
        return Ok(Response::new(reply));
    }

    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status>{
        let username =  request.get_ref().user.to_string();
        let mut verifiers: std::sync::MutexGuard<'_, HashMap<String, ChaumPedersenVerifier>> = self.state.verifiers.lock().unwrap();
        if let Some(verifier) = verifiers.get_mut(&username) {
            // User exists 

            let auth_id = utils::generate_random_string(32);
            let c: BigInt = verifier.verify_sync_a(
                BigInt::from(request.get_ref().r1),
                BigInt::from(request.get_ref().r2)
            );
            let mut auth_ids: std::sync::MutexGuard<'_, HashMap<String, String>> = self.state.auth_ids.lock().unwrap();
            auth_ids.insert(auth_id.to_string(), username.to_string());

            match c.to_i64(){
                Some(x) => return Ok(Response::new(AuthenticationChallengeResponse {auth_id:auth_id, c:x})),
                None => return Err(Status::aborted(format!("Wrong format for number {}", username.to_string()))),
            }
        } else {
            return Err(Status::not_found(format!("User Already Exists {}", username.to_string())));
        }
        
    }

    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status>{
        let auth_id =  request.get_ref().auth_id.to_string();
        let mut auth_ids: std::sync::MutexGuard<'_, HashMap<String, String>> = self.state.auth_ids.lock().unwrap();
        if let Some(username) = auth_ids.clone().get_mut(&auth_id) {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    log::set_max_level(LevelFilter::Debug);
    let addr = "0.0.0.0:50051".parse()?;
    let auth = MyAuth::new();

    Server::builder()
        .add_service(AuthServer::new(auth))
        .serve(addr)
        .await?;

    Ok(())
}
