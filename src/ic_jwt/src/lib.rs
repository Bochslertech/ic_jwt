mod env;
mod init;
mod service;
mod types;

use crate::service::JWTService;
use crate::types::{JWTServiceStorage, UserJWT};
use candid::Principal;
use ic_cdk::{caller, storage};
use ic_cdk_macros::{post_upgrade, pre_upgrade, query, update};
use std::cell::RefCell;
use std::mem;

thread_local! {
    static SERVICE: RefCell<JWTService> = RefCell::default();
}

#[update]
#[candid::candid_method(update)]
fn generate_jwt() -> String {
    SERVICE.with(|service| service.borrow_mut().generate_jwt())
}

#[query]
#[candid::candid_method(query)]
fn get_my_jwt() -> Result<UserJWT, String> {
    SERVICE.with(|service| service.borrow().get_my_jwt())
}

#[update(guard = "caller_is_owner")]
#[candid::candid_method(update)]
fn set_jwt_secret(new_secret: String) -> Result<String, String> {
    SERVICE.with(|service| service.borrow_mut().set_jwt_secret(new_secret))
}

#[query(guard = "caller_is_owner")]
#[candid::candid_method(query)]
fn get_user_jwt(user: Principal) -> Result<String, String> {
    SERVICE.with(|service| service.borrow().get_user_jwt(user))
}

#[update(guard = "caller_is_owner")]
#[candid::candid_method(update)]
fn set_owner(new_owner: Principal) -> Result<String, String> {
    SERVICE.with(|service| service.borrow_mut().set_owner(new_owner))
}

#[query]
#[candid::candid_method(query)]
fn get_owner() -> Principal {
    SERVICE.with(|service| service.borrow().get_owner())
}

#[query(guard = "caller_is_owner")]
#[candid::candid_method(query)]
fn get_jwt_secret() -> Result<String, String> {
    SERVICE.with(|service| service.borrow().get_jwt_secret())
}

#[pre_upgrade]
fn pre_upgrade() {
    let service: JWTService = SERVICE.with(|service| mem::take(&mut *service.borrow_mut()));
    // Transform into stable service
    let stable_service: types::StableService = service.into();
    storage::stable_save((stable_service,)).unwrap();
}
#[post_upgrade]
fn post_upgrade() {
    let (stable_service,): (types::StableService,) =
        ic_cdk::storage::stable_restore().expect("failed to restore stable service");

    // Transform from stable service
    let service = stable_service.into();
    SERVICE.with(|s| {
        s.replace(service);
    });
}

candid::export_service!();
#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

pub fn caller_is_owner() -> Result<(), String> {
    let caller: Principal = caller();
    let owner: Principal = SERVICE.with(|service| service.borrow().get_owner());
    if caller == owner {
        Ok(())
    } else {
        Err(format!("Caller ({}) is not a owner of the system.", caller))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;
        let dir = PathBuf::from(env::current_dir().unwrap());
        write(dir.join("ic_jwt.did"), export_candid()).expect("Write failed.");
    }
}
