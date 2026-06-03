//! WASM compute-budget benches for BytesN<N> -> [u8; N] conversions.
//!
//! These benches are disabled by default (`#[ignore]`) because:
//! 1. they should be run under `--release` rather than the default test profile.
//! 2. their output is only meaningful with `--nocapture`.
//!
//! Run with:
//! make build-test-wasms
//! cargo test --release --package soroban-sdk --lib --features testutils \
//!   -- tests::bytesn_bench_wasm --ignored --nocapture

use crate::{BytesN, Env};

mod bytesn_bench {
    use crate as soroban_sdk;
    soroban_sdk::contractimport!(file = "../target/wasm32v1-none/release/test_bytesn_bench.wasm");
}

fn report(label: &str, env: &Env) {
    let cpu = env.cost_estimate().budget().cpu_instruction_cost();
    let mem = env.cost_estimate().budget().memory_bytes_cost();
    println!("BENCH {label} cpu={cpu} mem={mem}");
}

macro_rules! bench_to_array {
    ($bench:ident, $fn_:ident, $n:literal) => {
        #[test]
        #[ignore]
        fn $bench() {
            let env = Env::default();
            let id = env.register(bytesn_bench::WASM, ());
            let client = bytesn_bench::Client::new(&env, &id);
            let b: BytesN<$n> = BytesN::from_array(&env, &[7u8; $n]);
            env.cost_estimate().budget().reset_unlimited();
            let _ = client.$fn_(&b);
            report(stringify!($bench), &env);
        }
    };
}

bench_to_array!(bench_baseline_32, baseline_32, 32);
bench_to_array!(bench_baseline_48, baseline_48, 48);
bench_to_array!(bench_baseline_96, baseline_96, 96);
bench_to_array!(bench_baseline_192, baseline_192, 192);

bench_to_array!(bench_to_array_32, to_array_32, 32);
bench_to_array!(bench_to_array_48, to_array_48, 48);
bench_to_array!(bench_to_array_96, to_array_96, 96);
bench_to_array!(bench_to_array_192, to_array_192, 192);

bench_to_array!(bench_to_array_ref_32, to_array_ref_32, 32);
bench_to_array!(bench_to_array_ref_48, to_array_ref_48, 48);
bench_to_array!(bench_to_array_ref_96, to_array_ref_96, 96);
bench_to_array!(bench_to_array_ref_192, to_array_ref_192, 192);
