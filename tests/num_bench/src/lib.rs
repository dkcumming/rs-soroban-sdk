#![no_std]
use soroban_sdk::{contract, contractimpl, Env, I256, U256};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn baseline(_env: Env) {}

    pub fn u256_from_u128(env: Env, x: u128) -> U256 {
        U256::from_u128(&env, x)
    }

    pub fn u256_to_u128(_env: Env, v: U256) -> Option<u128> {
        v.to_u128()
    }

    pub fn i256_from_i128(env: Env, x: i128) -> I256 {
        I256::from_i128(&env, x)
    }

    pub fn i256_to_i128(_env: Env, v: I256) -> Option<i128> {
        v.to_i128()
    }
}
