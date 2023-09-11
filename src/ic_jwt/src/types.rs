use candid::Principal;
use candid::{CandidType, Deserialize};

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