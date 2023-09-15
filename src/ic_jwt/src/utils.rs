use candid::Principal;

pub fn caller_is_not_anonymous() -> Result<(), String> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return Err("Caller is anonymous".to_string());
    }

    Ok(())
}
