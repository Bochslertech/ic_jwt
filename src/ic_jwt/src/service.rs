use crate::env::{EmptyEnvironment, Environment};
use crate::types::{JWTServiceStorage, UserJWT};

use candid::Principal;
use hmac::{Hmac, Mac};
use ic_cdk::caller;
use jwt::{Claims, SignWithKey};
use sha2::Sha256;
use std::collections::HashMap;

const VALIDITY_PERIOD: u64 = 60 * 60 * 24 * 7;
const CACHE_PERIOD: u64 = 60 * 60 * 24;

/// Implements the JWTService interface
pub struct JWTService {
    pub env: Box<dyn Environment>,
    pub jwt_users: HashMap<Principal, UserJWT>,
    pub owner: Principal,
    pub jwt_secret: String,
}

impl Default for JWTService {
    fn default() -> Self {
        JWTService {
            env: Box::new(EmptyEnvironment {}),
            jwt_users: HashMap::new(),
            owner: Principal::anonymous(),
            jwt_secret: String::from("some-secret"),
        }
    }
}

impl From<JWTServiceStorage> for JWTService {
    fn from(stable: JWTServiceStorage) -> JWTService {
        JWTService {
            env: Box::new(EmptyEnvironment {}),
            jwt_users: HashMap::new(),
            owner: stable.owner,
            jwt_secret: stable.jwt_secret,
        }
    }
}

/// Implements the JWTService interface
impl JWTService {
    pub fn generate_jwt(&mut self) -> String {
        let caller_user: String = caller().to_text();

        // find old jwt token
        if let Some(jwt_token) = self.jwt_users.get(&caller()) {
            if jwt_token.token_exp > self.env.now_secs() + CACHE_PERIOD {
                return jwt_token.token.clone();
            }
        }

        let key: Hmac<Sha256> = Hmac::new_from_slice(self.jwt_secret.as_bytes()).unwrap();
        let exp_at: u64 = self.env.now_secs() + VALIDITY_PERIOD;
        let mut claims: Claims = Default::default();
        claims.registered.issuer = Some(caller_user);
        claims.registered.expiration = Some(exp_at);
        claims.registered.subject = Some("canister login".into());
        let token_str = claims.sign_with_key(&key).unwrap();
        let user_jwt: UserJWT = UserJWT {
            token: token_str.clone(),
            token_exp: exp_at,
        };
        self.jwt_users.insert(caller(), user_jwt);
        return token_str;
    }

    /// Return the user JWT, if one exists
    pub fn get_user_jwt(&self, user_principal: Principal) -> Result<String, String> {
        if self.owner == caller() {
            let jwt_token = self
                .jwt_users
                .get(&user_principal)
                .ok_or_else(|| format!("No jwt with principal {} exists", user_principal))?;
            Ok(jwt_token.token.clone())
        } else {
            Err(String::from("caller error"))
        }
    }

    /// Set the jwt_secret
    pub fn set_jwt_secret(&mut self, new_secret: String) -> Result<String, String> {
        if self.owner == caller() {
            self.jwt_secret = new_secret.clone();
            Ok(new_secret)
        } else {
            Err(String::from("caller error"))
        }
    }

    /// Set the canister owner
    pub fn set_owner(&mut self, new_owner: Principal) -> Result<String, String> {
        if self.owner == caller() {
            self.owner = new_owner;
            Ok(new_owner.to_string())
        } else {
            Err(String::from("caller error"))
        }
    }

    /// Return the canister owner
    pub fn get_owner(&self) -> Principal {
        self.owner
    }

    pub fn get_jwt_secret(&self) -> Result<String, String> {
        if self.owner == caller() {
            Ok(self.jwt_secret.clone())
        } else {
            Err(String::from("caller error"))
        }
    }
}
