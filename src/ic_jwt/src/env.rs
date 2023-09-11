use candid::types::principal::Principal;
use std::time::Duration;

/// The functions that are provided by the environment that the canister runs in
///
/// This is primarily used to enable mocking out these values in tests
pub trait Environment {
    fn now(&self) -> u64;
    fn now_secs(&self) -> u64;
    fn caller(&self) -> Principal;
    fn canister_id(&self) -> Principal;
}

pub struct CanisterEnvironment {}

impl Environment for CanisterEnvironment {
    fn now(&self) -> u64 {
        ic_cdk::api::time()
    }

    fn now_secs(&self) -> u64 {
        let duration: Duration = Duration::from_nanos(ic_cdk::api::time());
        let seconds_timestamp = duration.as_secs();
        seconds_timestamp
    }

    fn caller(&self) -> Principal {
        ic_cdk::caller()
    }

    fn canister_id(&self) -> Principal {
        ic_cdk::id()
    }
}

pub struct EmptyEnvironment {}

impl Environment for EmptyEnvironment {
    fn now(&self) -> u64 {
        unimplemented!()
    }

    fn now_secs(&self) -> u64 {
        unimplemented!()
    }

    fn caller(&self) -> Principal {
        unimplemented!()
    }

    fn canister_id(&self) -> Principal {
        unimplemented!()
    }
}

#[cfg(test)]
pub struct TestEnvironment {
    pub now: u64,
    pub now_secs: u64,
    pub caller: Principal,
    pub canister_id: Principal,
}

#[cfg(test)]
impl Environment for TestEnvironment {
    fn now(&self) -> u64 {
        self.now
    }

    fn now_secs(&self) -> u64 {
        self.now_secs
    }

    fn caller(&self) -> Principal {
        self.caller
    }

    fn canister_id(&self) -> Principal {
        self.canister_id
    }
}
