use crate::env::CanisterEnvironment;
use crate::service::JWTService;
use crate::types::JWTServiceStorage;
use crate::SERVICE;
use ic_cdk_macros::init;

#[init]
#[candid::candid_method(init)]
fn init(init_state: JWTServiceStorage) {
    ic_cdk::setup();

    let mut init_service: JWTService = JWTService::from(init_state);
    init_service.env = Box::new(CanisterEnvironment {});

    SERVICE.with(|service| *service.borrow_mut() = init_service);
}
