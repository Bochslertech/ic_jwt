mod env;
mod init;
mod service;
mod types;

use candid::Principal;

use crate::service::JWTService;
use crate::types::JWTServiceStorage;
use ic_cdk_macros::{query, update};
use std::cell::RefCell;

thread_local! {
    static SERVICE: RefCell<JWTService> = RefCell::default();
}

#[update]
#[candid::candid_method(update)]
fn generate_jwt() -> String {
    SERVICE.with(|service| service.borrow_mut().generate_jwt())
}

#[update]
#[candid::candid_method(update)]
fn set_jwt_secret(new_secret: String) -> () {
    SERVICE.with(|service| service.borrow_mut().set_jwt_secret(new_secret))
}

#[query]
#[candid::candid_method(query)]
fn get_user_jwt(user: Principal) -> Result<String, String> {
    SERVICE.with(|service| service.borrow().get_user_jwt(user))
}

#[update]
#[candid::candid_method(update)]
fn set_owner(new_owner: Principal) -> () {
    SERVICE.with(|service| service.borrow_mut().set_owner(new_owner))
}

#[query]
#[candid::candid_method(query)]
fn get_owner() -> Principal {
    SERVICE.with(|service| service.borrow().get_owner())
}

#[query]
#[candid::candid_method(query)]
fn get_jwt_secret() -> Result<String, String> {
    SERVICE.with(|service| service.borrow().get_jwt_secret())
}

candid::export_service!();
#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
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
