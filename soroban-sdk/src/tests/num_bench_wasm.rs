//! WASM compute-budget benches for U256 / I256 <-> u128 / i128.
//!
//! Run with:
//! make build-test-wasms
//! cargo test --release --package soroban-sdk --lib --features testutils \
//!   -- tests::num_bench_wasm --ignored --nocapture

use crate::{Env, I256, U256};

mod num_bench {
    use crate as soroban_sdk;
    soroban_sdk::contractimport!(file = "../target/wasm32v1-none/release/test_num_bench.wasm");
}

#[test]
#[ignore]
fn bench_baseline() {
    let env = Env::default();
    let id = env.register(num_bench::WASM, ());
    let client = num_bench::Client::new(&env, &id);
    env.cost_estimate().budget().reset_unlimited();
    client.baseline();
    println!("=== baseline ===");
    env.cost_estimate().budget().print();
}

#[test]
#[ignore]
fn bench_u256_from_u128_max() {
    let env = Env::default();
    let id = env.register(num_bench::WASM, ());
    let client = num_bench::Client::new(&env, &id);
    env.cost_estimate().budget().reset_unlimited();
    let _ = client.u256_from_u128(&u128::MAX);
    println!("=== u256_from_u128_max ===");
    env.cost_estimate().budget().print();
}

#[test]
#[ignore]
fn bench_u256_to_u128_fits() {
    let env = Env::default();
    let id = env.register(num_bench::WASM, ());
    let client = num_bench::Client::new(&env, &id);
    let v = U256::from_u128(&env, u128::MAX);
    env.cost_estimate().budget().reset_unlimited();
    let _ = client.u256_to_u128(&v);
    println!("=== u256_to_u128_fits ===");
    env.cost_estimate().budget().print();
}

#[test]
#[ignore]
fn bench_u256_to_u128_overflow() {
    let env = Env::default();
    let id = env.register(num_bench::WASM, ());
    let client = num_bench::Client::new(&env, &id);
    let v = U256::from_u128(&env, u128::MAX).mul(&U256::from_u32(&env, 8));
    env.cost_estimate().budget().reset_unlimited();
    let _ = client.u256_to_u128(&v);
    println!("=== u256_to_u128_overflow ===");
    env.cost_estimate().budget().print();
}

#[test]
#[ignore]
fn bench_i256_from_i128_min() {
    let env = Env::default();
    let id = env.register(num_bench::WASM, ());
    let client = num_bench::Client::new(&env, &id);
    env.cost_estimate().budget().reset_unlimited();
    let _ = client.i256_from_i128(&i128::MIN);
    println!("=== i256_from_i128_min ===");
    env.cost_estimate().budget().print();
}

#[test]
#[ignore]
fn bench_i256_to_i128_pos() {
    let env = Env::default();
    let id = env.register(num_bench::WASM, ());
    let client = num_bench::Client::new(&env, &id);
    let v = I256::from_i128(&env, i128::MAX);
    env.cost_estimate().budget().reset_unlimited();
    let _ = client.i256_to_i128(&v);
    println!("=== i256_to_i128_pos ===");
    env.cost_estimate().budget().print();
}

#[test]
#[ignore]
fn bench_i256_to_i128_neg() {
    let env = Env::default();
    let id = env.register(num_bench::WASM, ());
    let client = num_bench::Client::new(&env, &id);
    let v = I256::from_i128(&env, i128::MIN);
    env.cost_estimate().budget().reset_unlimited();
    let _ = client.i256_to_i128(&v);
    println!("=== i256_to_i128_neg ===");
    env.cost_estimate().budget().print();
}
