use soroban_sdk::{contractimpl, Address, Env};

pub struct MockFactory;

#[contractimpl]
impl MockFactory {
    pub fn record_volume(_env: Env, _contract: Address, _token: Address, _amount: i128) {}
    pub fn track_participant(_env: Env, _participant: Address) {}
}
