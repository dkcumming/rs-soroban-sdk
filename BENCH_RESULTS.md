# `BytesN<N>` -> `[u8; N]` conversion bench

Branch: `dc/bytes-from-zc-bench` (off `origin/main`, commit `cbb02ecd`).

Benches the `From<BytesN<N>> for [u8; N]` and `From<&BytesN<N>> for [u8; N]`
conversions. Owned (`From<BytesN<N>>`) and reference (`From<&BytesN<N>>`)
variants measure identically (in most cases); both rows are listed for
completeness.

## Host-side (host-function metering only)

Run: `cargo test --release --package soroban-sdk --lib --features testutils -- tests::bytesn_bench --ignored --nocapture`

| Bench                       | Before CPU | After CPU | Δ % | Before Mem | After Mem | Δ % |
|-----------------------------|-----------:|----------:|----:|-----------:|----------:|----:|
| `bytesn_32_into_array`      |     48,602 |           |     |      3,568 |           |     |
| `bytesn_32_ref_into_array`  |     48,602 |           |     |      3,568 |           |     |
| `bytesn_48_into_array`      |     72,938 |           |     |      5,736 |           |     |
| `bytesn_48_ref_into_array`  |     72,938 |           |     |      5,736 |           |     |
| `bytesn_96_into_array`      |    146,330 |           |     |     13,776 |           |     |
| `bytesn_96_ref_into_array`  |    146,330 |           |     |     13,776 |           |     |
| `bytesn_192_into_array`     |    294,842 |           |     |     36,768 |           |     |
| `bytesn_192_ref_into_array` |    294,842 |           |     |     36,768 |           |     |

## WASM-metered (guest WASM + host-function metering)

Run: `make build-test-wasms && cargo test --release --package soroban-sdk --lib --features testutils -- tests::bytesn_bench_wasm --ignored --nocapture`

| Bench              | Before CPU | After CPU | Δ % | Before Mem | After Mem | Δ % |
|--------------------|-----------:|----------:|----:|-----------:|----------:|----:|
| `baseline_*`       |    263,201 |           |     |  1,158,730 |           |     |
| `to_array_32`      |    443,381 |           |     |  1,163,066 |           |     |
| `to_array_ref_32`  |    443,381 |           |     |  1,163,066 |           |     |
| `to_array_48`      |    533,025 |           |     |  1,165,618 |           |     |
| `to_array_ref_48`  |    533,025 |           |     |  1,165,618 |           |     |
| `to_array_96`      |    799,665 |           |     |  1,174,810 |           |     |
| `to_array_ref_96`  |    799,665 |           |     |  1,174,810 |           |     |
| `to_array_192`     |  1,334,673 |           |     |  1,200,106 |           |     |
| `to_array_ref_192` |  1,334,675 |           |     |  1,200,106 |           |     |

## Conversion-only (WASM, baseline-subtracted)

Each cell is the corresponding WASM-metered cell with the matching
`baseline_N` row subtracted, isolating the cost attributable to the
conversion itself rather than to dispatch and operand passing.

Before baseline (from above): 263,201 CPU, 1,158,730 mem bytes.

| Bench              | Before CPU | After CPU | Δ % | Before Mem | After Mem | Δ % |
|--------------------|-----------:|----------:|----:|-----------:|----------:|----:|
| `to_array_32`      |    180,180 |           |     |      4,336 |           |     |
| `to_array_ref_32`  |    180,180 |           |     |      4,336 |           |     |
| `to_array_48`      |    269,824 |           |     |      6,888 |           |     |
| `to_array_ref_48`  |    269,824 |           |     |      6,888 |           |     |
| `to_array_96`      |    536,464 |           |     |     16,080 |           |     |
| `to_array_ref_96`  |    536,464 |           |     |     16,080 |           |     |
| `to_array_192`     |  1,071,472 |           |     |     41,376 |           |     |
| `to_array_ref_192` |  1,071,474 |           |     |     41,376 |           |     |
