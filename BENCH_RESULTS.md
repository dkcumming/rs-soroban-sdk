# `BytesN<N>` -> `[u8; N]` conversion bench

Branch: `dc/bytes-from-zc-bench` (off `origin/main`, commit `cbb02ecd`).

Benches the `From<BytesN<N>> for [u8; N]` and `From<&BytesN<N>> for [u8; N]`
conversions. Owned (`From<BytesN<N>>`) and reference (`From<&BytesN<N>>`)
variants measure identically (in most cases); both rows are listed for
completeness.

## Host-side (host-function metering only)

Run: `cargo test --release --package soroban-sdk --lib --features testutils -- tests::bytesn_bench --ignored --nocapture`

| Bench                       | Before CPU | After CPU |    Δ % | Before Mem | After Mem |   Δ % |
|-----------------------------|-----------:|----------:|-------:|-----------:|----------:|------:|
| `bytesn_32_into_array`      |     48,602 |       107 | -99.8% |      3,568 |         0 | -100% |
| `bytesn_32_ref_into_array`  |     48,602 |       107 | -99.8% |      3,568 |         0 | -100% |
| `bytesn_48_into_array`      |     72,938 |       109 | -99.9% |      5,736 |         0 | -100% |
| `bytesn_48_ref_into_array`  |     72,938 |       109 | -99.9% |      5,736 |         0 | -100% |
| `bytesn_96_into_array`      |    146,330 |       115 | -99.9% |     13,776 |         0 | -100% |
| `bytesn_96_ref_into_array`  |    146,330 |       115 | -99.9% |     13,776 |         0 | -100% |
| `bytesn_192_into_array`     |    294,842 |       127 | -99.9% |     36,768 |         0 | -100% |
| `bytesn_192_ref_into_array` |    294,842 |       127 | -99.9% |     36,768 |         0 | -100% |

## WASM-metered (guest WASM + host-function metering)

Run: `make build-test-wasms && cargo test --release --package soroban-sdk --lib --features testutils -- tests::bytesn_bench_wasm --ignored --nocapture`

| Bench              | Before CPU | After CPU |    Δ % | Before Mem | After Mem |   Δ % |
|--------------------|-----------:|----------:|-------:|-----------:|----------:|------:|
| `baseline_*`       |    263,201 |   256,273 |  -2.6% |  1,158,730 | 1,154,758 | -0.3% |
| `to_array_32`      |    443,381 |   257,511 | -41.9% |  1,163,066 | 1,154,758 | -0.7% |
| `to_array_ref_32`  |    443,381 |   257,511 | -41.9% |  1,163,066 | 1,154,758 | -0.7% |
| `to_array_48`      |    533,025 |   258,573 | -51.5% |  1,165,618 | 1,154,758 | -0.9% |
| `to_array_ref_48`  |    533,025 |   258,573 | -51.5% |  1,165,618 | 1,154,758 | -0.9% |
| `to_array_96`      |    799,665 |   259,107 | -67.6% |  1,174,810 | 1,154,758 | -1.7% |
| `to_array_ref_96`  |    799,665 |   259,107 | -67.6% |  1,174,810 | 1,154,758 | -1.7% |
| `to_array_192`     |  1,334,673 |   260,175 | -80.5% |  1,200,106 | 1,154,758 | -3.8% |
| `to_array_ref_192` |  1,334,675 |   260,177 | -80.5% |  1,200,106 | 1,154,758 | -3.8% |

## Conversion-only (WASM, baseline-subtracted)

Each cell is the corresponding WASM-metered cell with the matching
`baseline_N` row subtracted, isolating the cost attributable to the
conversion itself rather than to dispatch and operand passing.

Before baseline (from above): 263,201 CPU, 1,158,730 mem bytes.
After baseline  (from above): 256,273 CPU, 1,154,758 mem bytes.

| Bench              | Before CPU | After CPU |    Δ % | Before Mem | After Mem |   Δ % |
|--------------------|-----------:|----------:|-------:|-----------:|----------:|------:|
| `to_array_32`      |    180,180 |     1,238 | -99.3% |      4,336 |         0 | -100% |
| `to_array_ref_32`  |    180,180 |     1,238 | -99.3% |      4,336 |         0 | -100% |
| `to_array_48`      |    269,824 |     2,300 | -99.1% |      6,888 |         0 | -100% |
| `to_array_ref_48`  |    269,824 |     2,300 | -99.1% |      6,888 |         0 | -100% |
| `to_array_96`      |    536,464 |     2,834 | -99.5% |     16,080 |         0 | -100% |
| `to_array_ref_96`  |    536,464 |     2,834 | -99.5% |     16,080 |         0 | -100% |
| `to_array_192`     |  1,071,472 |     3,902 | -99.6% |     41,376 |         0 | -100% |
| `to_array_ref_192` |  1,071,474 |     3,904 | -99.6% |     41,376 |         0 | -100% |
