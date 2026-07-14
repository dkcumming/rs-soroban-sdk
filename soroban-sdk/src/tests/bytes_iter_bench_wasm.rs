//! WASM compute-budget benches for iterating a `Bytes`.
//!
//! These benches are disabled by default (`#[ignore]`) because:
//! 1. they should be run under `--release` rather than the default test profile.
//! 2. their output is only meaningful with `--nocapture`.
//!
//! Run with:
//! make build-test-wasms
//! cargo test --release --package soroban-sdk --lib --features testutils \
//!   -- tests::bytes_iter_bench_wasm --ignored --nocapture

use crate::{Bytes, Env};

mod bytes_iter_bench {
    use crate as soroban_sdk;
    soroban_sdk::contractimport!(
        file = "../target/wasm32v1-none/release/test_bytes_iter_bench.wasm"
    );
}

fn report(label: &str, env: &Env) {
    let cpu = env.cost_estimate().budget().cpu_instruction_cost();
    let mem = env.cost_estimate().budget().memory_bytes_cost();
    println!("BENCH {label} cpu={cpu} mem={mem}");
}

macro_rules! bench {
    ($bench:ident, $fn_:ident, $n:literal) => {
        #[test]
        #[ignore]
        fn $bench() {
            let env = Env::default();
            let id = env.register(bytes_iter_bench::WASM, ());
            let client = bytes_iter_bench::Client::new(&env, &id);
            let b = Bytes::from_slice(&env, &[7u8; $n]);
            env.cost_estimate().budget().reset_unlimited();
            let _ = client.$fn_(&b);
            report(stringify!($bench), &env);
        }
    };
}

bench!(bench_baseline_32, baseline, 32);
bench!(bench_baseline_48, baseline, 48);
bench!(bench_baseline_96, baseline, 96);
bench!(bench_baseline_192, baseline, 192);

bench!(bench_iter_32, sum_iter, 32);
bench!(bench_iter_48, sum_iter, 48);
bench!(bench_iter_96, sum_iter, 96);
bench!(bench_iter_192, sum_iter, 192);

bench!(bench_iter_rev_32, sum_iter_rev, 32);
bench!(bench_iter_rev_48, sum_iter_rev, 48);
bench!(bench_iter_rev_96, sum_iter_rev, 96);
bench!(bench_iter_rev_192, sum_iter_rev, 192);
