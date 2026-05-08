# U256 / I256 <-> u128 / i128 conversion bench

Branch: `dc/U258_u128` (off `origin/main`, commit `e1bf74ba`).

## Host-side (host-function metering only)

Run: `cargo test --release --package soroban-sdk --lib --features testutils -- tests::num_bench --ignored --nocapture`

| Bench                   | Before CPU | After CPU |    Δ % | Before Mem | After Mem |    Δ % |
|-------------------------|-----------:|----------:|-------:|-----------:|----------:|-------:|
| `u256_from_u128_max`    |      3,942 |       503 | -87.2% |        432 |        80 | -81.5% |
| `u256_to_u128_fits`     |     52,358 |       488 | -99.1% |      3,664 |         0 | -100%  |
| `u256_to_u128_overflow` |     52,358 |       244 | -99.5% |      3,664 |         0 | -100%  |
| `i256_from_i128_min`    |      3,942 |       503 | -87.2% |        432 |        80 | -81.5% |
| `i256_to_i128_pos`      |     52,358 |       488 | -99.1% |      3,664 |         0 | -100%  |
| `i256_to_i128_neg`      |     52,358 |       488 | -99.1% |      3,664 |         0 | -100%  |

## WASM-metered (guest WASM + host-function metering)

Run: `make build-test-wasms && cargo test --release --package soroban-sdk --lib --features testutils -- tests::num_bench_wasm --ignored --nocapture`

| Bench                   | Before CPU | After CPU |    Δ % | Before Mem | After Mem |   Δ % |
|-------------------------|-----------:|----------:|-------:|-----------:|----------:|------:|
| `baseline`              |    329,285 |   326,974 |  -0.7% |  1,219,606 | 1,150,203 | -5.7% |
| `u256_from_u128_max`    |    340,811 |   330,954 |  -2.9% |  1,220,190 | 1,150,363 | -5.7% |
| `u256_to_u128_fits`     |    485,986 |   332,682 | -31.5% |  1,224,270 | 1,150,363 | -6.0% |
| `u256_to_u128_overflow` |    484,433 |   330,265 | -31.8% |  1,224,166 | 1,150,259 | -6.0% |
| `i256_from_i128_min`    |    340,839 |   330,974 |  -2.9% |  1,220,190 | 1,150,363 | -5.7% |
| `i256_to_i128_pos`      |    486,082 |   332,806 | -31.5% |  1,224,270 | 1,150,363 | -6.0% |
| `i256_to_i128_neg`      |    486,082 |   332,806 | -31.5% |  1,224,270 | 1,150,363 | -6.0% |

Each row is the budget of a single contract invocation that calls the
named conversion exactly once. The contract is registered and any operand
is constructed *before* the budget is reset, so the measurement excludes
Wasm parsing, instantiation, and on-host operand construction. What
remains is the contract dispatch plus the conversion's host-function and
Wasm instruction costs.

- `baseline` is an empty contract function under the same harness,
  capturing the fixed dispatch overhead. Subtract it from any other row
  to estimate the conversion-only delta (see the next section).

## Conversion-only (WASM, baseline-subtracted)

Each cell is the corresponding WASM-metered cell with the `baseline` row
subtracted, isolating the cost attributable to the conversion itself
rather than to dispatch and instantiation. Removing the fixed overhead
makes the result directly comparable to the host-side table above, and
the numbers are fairly similar.

Before baseline: 329,285 CPU, 1,219,606 mem bytes.
After baseline: 326,974 CPU, 1,150,203 mem bytes.

| Bench                   | Before CPU | After CPU |    Δ % | Before Mem | After Mem |    Δ % |
|-------------------------|-----------:|----------:|-------:|-----------:|----------:|-------:|
| `u256_from_u128_max`    |     11,526 |     3,980 | -65.5% |        584 |       160 | -72.6% |
| `u256_to_u128_fits`     |    156,701 |     5,708 | -96.4% |      4,664 |       160 | -96.6% |
| `u256_to_u128_overflow` |    155,148 |     3,291 | -97.9% |      4,560 |        56 | -98.8% |
| `i256_from_i128_min`    |     11,554 |     4,000 | -65.4% |        584 |       160 | -72.6% |
| `i256_to_i128_pos`      |    156,797 |     5,832 | -96.3% |      4,664 |       160 | -96.6% |
| `i256_to_i128_neg`      |    156,797 |     5,832 | -96.3% |      4,664 |       160 | -96.6% |
