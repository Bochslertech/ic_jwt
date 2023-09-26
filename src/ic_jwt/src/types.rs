use std::collections::HashMap;

use candid::Principal;
use candid::{CandidType, Deserialize};

use crate::env::CanisterEnvironment;
use crate::service::JWTService;

#[derive(CandidType, Deserialize)]
pub struct StableService {
    jwt_users: HashMap<Principal, UserJWT>,
    owner: Principal,
    jwt_secret: String,
}

impl From<JWTService> for StableService {
    fn from(input: JWTService) -> Self {
        StableService {
            jwt_users: input.jwt_users,
            owner: input.owner,
            jwt_secret: input.jwt_secret,
        }
    }
}

impl From<StableService> for JWTService {
    fn from(input: StableService) -> Self {
        JWTService {
            env: Box::new(CanisterEnvironment {}),
            owner: input.owner,
            jwt_secret: input.jwt_secret,
            jwt_users: input.jwt_users,
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct JWTServiceStorage {
    pub owner: Principal,
    pub jwt_secret: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Account {
    pub owner: Principal,
    pub jwt_token: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct UserJWT {
    pub token: String,
    pub token_exp: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct WalletReceiveResult {
    pub accepted: u64,
}
