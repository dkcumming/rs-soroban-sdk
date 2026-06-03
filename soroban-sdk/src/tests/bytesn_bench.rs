//! Compute-budget microbenches for BytesN<N> -> [u8; N] conversions.
//!
//! These benches are disabled by default (`#[ignore]`) because:
//! 1. they should be run under `--release` rather than the default test profile.
//! 2. their output is only meaningful with `--nocapture`.
//!
//! Run with the following command:
//! cargo test --release --package soroban-sdk --lib --features testutils \
//!   -- tests::bytesn_bench --ignored --nocapture

use crate::{BytesN, Env};

fn report(label: &str, env: &Env) {
    let cpu = env.cost_estimate().budget().cpu_instruction_cost();
    let mem = env.cost_estimate().budget().memory_bytes_cost();
    println!("BENCH {label} cpu={cpu} mem={mem}");
}

macro_rules! bench_into_array {
    ($owned:ident, $ref_:ident, $n:literal) => {
        #[test]
        #[ignore]
        fn $owned() {
            let env = Env::default();
            let b: BytesN<$n> = BytesN::from_array(&env, &[7u8; $n]);
            env.cost_estimate().budget().reset_unlimited();
            let _: [u8; $n] = b.into();
            report(stringify!($owned), &env);
        }

        #[test]
        #[ignore]
        fn $ref_() {
            let env = Env::default();
            let b: BytesN<$n> = BytesN::from_array(&env, &[7u8; $n]);
            env.cost_estimate().budget().reset_unlimited();
            let _: [u8; $n] = (&b).into();
            report(stringify!($ref_), &env);
        }
    };
}

bench_into_array!(
    bench_bytesn_32_into_array,
    bench_bytesn_32_ref_into_array,
    32
);
bench_into_array!(
    bench_bytesn_48_into_array,
    bench_bytesn_48_ref_into_array,
    48
);
bench_into_array!(
    bench_bytesn_96_into_array,
    bench_bytesn_96_ref_into_array,
    96
);
bench_into_array!(
    bench_bytesn_192_into_array,
    bench_bytesn_192_ref_into_array,
    192
);
