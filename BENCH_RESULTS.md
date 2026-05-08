# U256 / I256 <-> u128 / i128 conversion bench

Branch: `dc/U258_u128` (off `origin/main`, commit `e1bf74ba`).
Run: `cargo test --release --package soroban-sdk --lib --features testutils -- tests::num_bench --ignored --nocapture`

| Bench                   | Before CPU | After CPU |    Δ % | Before Mem | After Mem |    Δ % |
|-------------------------|-----------:|----------:|-------:|-----------:|----------:|-------:|
| `u256_from_u128_max`    |      3,942 |           |        |        432 |           |        |
| `u256_to_u128_fits`     |     52,358 |           |        |      3,664 |           |        |
| `u256_to_u128_overflow` |     52,358 |           |        |      3,664 |           |        |
| `i256_from_i128_min`    |      3,942 |           |        |        432 |           |        |
| `i256_to_i128_pos`      |     52,358 |           |        |      3,664 |           |        |
| `i256_to_i128_neg`      |     52,358 |           |        |      3,664 |           |        |
