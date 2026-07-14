//! Compute-budget microbenches for iterating a `Bytes` (host-function metering
//! only).
//!
//! These benches are disabled by default (`#[ignore]`) because:
//! 1. they should be run under `--release` rather than the default test profile.
//! 2. their output is only meaningful with `--nocapture`.
//!
//! Run with the following command:
//! cargo test --release --package soroban-sdk --lib --features testutils \
//!   -- tests::bytes_iter_bench --ignored --nocapture

use crate::{Bytes, Env};

fn report(label: &str, env: &Env) {
    let cpu = env.cost_estimate().budget().cpu_instruction_cost();
    let mem = env.cost_estimate().budget().memory_bytes_cost();
    println!("BENCH {label} cpu={cpu} mem={mem}");
}

macro_rules! bench_iter {
    ($fwd:ident, $rev:ident, $n:literal) => {
        #[test]
        #[ignore]
        fn $fwd() {
            let env = Env::default();
            let b = Bytes::from_slice(&env, &[7u8; $n]);
            env.cost_estimate().budget().reset_unlimited();
            let mut acc: u32 = 0;
            for x in b.iter() {
                acc = acc.wrapping_add(x as u32);
            }
            core::hint::black_box(acc);
            report(stringify!($fwd), &env);
        }

        #[test]
        #[ignore]
        fn $rev() {
            let env = Env::default();
            let b = Bytes::from_slice(&env, &[7u8; $n]);
            env.cost_estimate().budget().reset_unlimited();
            let mut acc: u32 = 0;
            for x in b.iter().rev() {
                acc = acc.wrapping_add(x as u32);
            }
            core::hint::black_box(acc);
            report(stringify!($rev), &env);
        }
    };
}

bench_iter!(bench_iter_32, bench_iter_rev_32, 32);
bench_iter!(bench_iter_48, bench_iter_rev_48, 48);
bench_iter!(bench_iter_96, bench_iter_rev_96, 96);
bench_iter!(bench_iter_192, bench_iter_rev_192, 192);
