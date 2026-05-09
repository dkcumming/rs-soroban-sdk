# U256 / I256 <-> u128 / i128 conversion bench

Branch: `dc/U258_u128` (off `origin/main`, commit `e1bf74ba`).

## Host-side (host-function metering only)

Run: `cargo test --release --package soroban-sdk --lib --features testutils -- tests::num_bench --ignored --nocapture`

| Bench                   | Before CPU | After CPU |    Δ % | Before Mem | After Mem |    Δ % |
|-------------------------|-----------:|----------:|-------:|-----------:|----------:|-------:|
| `u256_from_u128_max`    |      3,942 |           |        |        432 |           |        |
| `u256_to_u128_fits`     |     52,358 |           |        |      3,664 |           |        |
| `u256_to_u128_overflow` |     52,358 |           |        |      3,664 |           |        |
| `i256_from_i128_min`    |      3,942 |           |        |        432 |           |        |
| `i256_to_i128_pos`      |     52,358 |           |        |      3,664 |           |        |
| `i256_to_i128_neg`      |     52,358 |           |        |      3,664 |           |        |

## WASM-metered (guest WASM + host-function metering)

Run: `make build-test-wasms && cargo test --release --package soroban-sdk --lib --features testutils -- tests::num_bench_wasm --ignored --nocapture`

| Bench                   | Before CPU | After CPU | Δ % | Before Mem | After Mem | Δ % |
|-------------------------|-----------:|----------:|----:|-----------:|----------:|----:|
| `baseline`              |    329,285 |           |     |  1,219,606 |           |     |
| `u256_from_u128_max`    |    340,811 |           |     |  1,220,190 |           |     |
| `u256_to_u128_fits`     |    485,986 |           |     |  1,224,270 |           |     |
| `u256_to_u128_overflow` |    484,433 |           |     |  1,224,166 |           |     |
| `i256_from_i128_min`    |    340,839 |           |     |  1,220,190 |           |     |
| `i256_to_i128_pos`      |    486,082 |           |     |  1,224,270 |           |     |
| `i256_to_i128_neg`      |    486,082 |           |     |  1,224,270 |           |     |

Each row is the budget of a single contract invocation that calls the
named conversion exactly once. The contract is registered and any operand
is constructed *before* the budget is reset, so the measurement excludes
Wasm parsing, instantiation, and on-host operand construction. What
remains is the contract dispatch plus the conversion's host-function and
Wasm instruction costs.

- `baseline` is an empty contract function under the same harness,
  capturing the fixed dispatch overhead. Subtract it from any other row
  to estimate the conversion-only delta.
