#![no_std]
use soroban_sdk::{contract, contractimpl, BytesN, Env};

#[contract]
pub struct Contract;

// Each conversion returns a value derived from the array so the conversion
// cannot be optimized out of the guest Wasm.
//
// Each `to_array_N` has a matching `baseline_N` with the same signature, so
// that subtracting it accounts for dispatch overhead.

#[contractimpl]
impl Contract {
    pub fn baseline_32(_env: Env, _b: BytesN<32>) -> u32 {
        0
    }

    pub fn baseline_48(_env: Env, _b: BytesN<48>) -> u32 {
        0
    }

    pub fn baseline_96(_env: Env, _b: BytesN<96>) -> u32 {
        0
    }

    pub fn baseline_192(_env: Env, _b: BytesN<192>) -> u32 {
        0
    }

    pub fn to_array_32(_env: Env, b: BytesN<32>) -> u32 {
        let a: [u8; 32] = b.into();
        a[0] as u32 + a[31] as u32
    }

    pub fn to_array_48(_env: Env, b: BytesN<48>) -> u32 {
        let a: [u8; 48] = b.into();
        a[0] as u32 + a[47] as u32
    }

    pub fn to_array_96(_env: Env, b: BytesN<96>) -> u32 {
        let a: [u8; 96] = b.into();
        a[0] as u32 + a[95] as u32
    }

    pub fn to_array_192(_env: Env, b: BytesN<192>) -> u32 {
        let a: [u8; 192] = b.into();
        a[0] as u32 + a[191] as u32
    }

    pub fn to_array_ref_32(_env: Env, b: BytesN<32>) -> u32 {
        let a: [u8; 32] = (&b).into();
        a[0] as u32 + a[31] as u32
    }

    pub fn to_array_ref_48(_env: Env, b: BytesN<48>) -> u32 {
        let a: [u8; 48] = (&b).into();
        a[0] as u32 + a[47] as u32
    }

    pub fn to_array_ref_96(_env: Env, b: BytesN<96>) -> u32 {
        let a: [u8; 96] = (&b).into();
        a[0] as u32 + a[95] as u32
    }

    pub fn to_array_ref_192(_env: Env, b: BytesN<192>) -> u32 {
        let a: [u8; 192] = (&b).into();
        a[0] as u32 + a[191] as u32
    }
}
