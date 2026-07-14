#![no_std]
use soroban_sdk::{contract, contractimpl, Bytes, Env};

#[contract]
pub struct Contract;

// Each function derives its result from every byte so the iteration cannot be
// optimized out of the guest Wasm.
//
// `baseline` has the same signature as the iterating functions, so subtracting
// it isolates the iteration cost from dispatch and operand-passing overhead.

#[contractimpl]
impl Contract {
    pub fn baseline(_env: Env, _b: Bytes) -> u32 {
        0
    }

    pub fn sum_iter(_env: Env, b: Bytes) -> u32 {
        let mut acc: u32 = 0;
        for x in b.iter() {
            acc = acc.wrapping_add(x as u32);
        }
        acc
    }

    pub fn sum_iter_rev(_env: Env, b: Bytes) -> u32 {
        let mut acc: u32 = 0;
        for x in b.iter().rev() {
            acc = acc.wrapping_add(x as u32);
        }
        acc
    }
}
