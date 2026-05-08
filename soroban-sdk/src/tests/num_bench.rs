//! Compute-budget microbenches for U256 / I256 <-> u128 / i128 conversions.
//!
//! These benches read the host's modeled CPU and memory budget after invoking
//! a conversion, to track the cost-model impact of changes to those paths.
//! They should be run with the release profile, i.e. `cargo test --release`,
//! to mirror the configuration used to publish results.
//!
//! These benches are disabled by default (`#[ignore]`) because:
//! 1. they should be run under `--release` rather than the default test profile.
//! 2. their output is only meaningful with `--nocapture`.
//!
//! Run with the following command:
//! cargo test --release --package soroban-sdk --lib --features testutils \
//!   -- tests::num_bench --ignored --nocapture

use crate::{Env, I256, U256};

fn report(label: &str, env: &Env) {
    let cpu = env.cost_estimate().budget().cpu_instruction_cost();
    let mem = env.cost_estimate().budget().memory_bytes_cost();
    println!("BENCH {label} cpu={cpu} mem={mem}");
}

#[test]
#[ignore]
fn bench_u256_from_u128_max() {
    let env = Env::default();
    env.cost_estimate().budget().reset_unlimited();
    let _ = U256::from_u128(&env, u128::MAX);
    report("u256_from_u128_max", &env);
}

#[test]
#[ignore]
fn bench_u256_to_u128_fits() {
    let env = Env::default();
    let v = U256::from_u128(&env, u128::MAX);
    env.cost_estimate().budget().reset_unlimited();
    let _ = v.to_u128();
    report("u256_to_u128_fits", &env);
}

#[test]
#[ignore]
fn bench_u256_to_u128_overflow() {
    let env = Env::default();
    let v = U256::from_u128(&env, u128::MAX).mul(&U256::from_u32(&env, 8));
    env.cost_estimate().budget().reset_unlimited();
    let _ = v.to_u128();
    report("u256_to_u128_overflow", &env);
}

#[test]
#[ignore]
fn bench_i256_from_i128_min() {
    let env = Env::default();
    env.cost_estimate().budget().reset_unlimited();
    let _ = I256::from_i128(&env, i128::MIN);
    report("i256_from_i128_min", &env);
}

#[test]
#[ignore]
fn bench_i256_to_i128_pos() {
    let env = Env::default();
    let v = I256::from_i128(&env, i128::MAX);
    env.cost_estimate().budget().reset_unlimited();
    let _ = v.to_i128();
    report("i256_to_i128_pos", &env);
}

#[test]
#[ignore]
fn bench_i256_to_i128_neg() {
    let env = Env::default();
    let v = I256::from_i128(&env, i128::MIN);
    env.cost_estimate().budget().reset_unlimited();
    let _ = v.to_i128();
    report("i256_to_i128_neg", &env);
}
