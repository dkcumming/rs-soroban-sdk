# `Bytes` / `BytesN` iteration bench

Branch: `dc/bytes-iter-bench` (off `upstream/main`, commit `e5cb4b52`).

Benches iterating every byte of a `Bytes` via `Bytes::iter()`, forwards
(`sum_iter`) and in reverse (`sum_iter_rev`). `BytesN` iterates through the
same `BytesIter`, so the results apply to both.

## Host-side (host-function metering only)

Run: `cargo test --release --package soroban-sdk --lib --features testutils -- tests::bytes_iter_bench:: --ignored --nocapture`

| Bench             | Before CPU | After CPU | Δ % | Before Mem | After Mem | Δ % |
|-------------------|-----------:|----------:|----:|-----------:|----------:|----:|
| `iter_32`         |     48,602 |           |     |      3,568 |           |     |
| `iter_rev_32`     |     44,698 |           |     |      3,568 |           |     |
| `iter_48`         |     72,938 |           |     |      5,736 |           |     |
| `iter_rev_48`     |     67,082 |           |     |      5,736 |           |     |
| `iter_96`         |    146,330 |           |     |     13,776 |           |     |
| `iter_rev_96`     |    134,618 |           |     |     13,776 |           |     |
| `iter_192`        |    294,842 |           |     |     36,768 |           |     |
| `iter_rev_192`    |    271,418 |           |     |     36,768 |           |     |

## WASM-metered (guest WASM + host-function metering)

Run: `make build-test-wasms && cargo test --release --package soroban-sdk --lib --features testutils -- tests::bytes_iter_bench_wasm --ignored --nocapture`

| Bench             | Before CPU | After CPU | Δ % | Before Mem | After Mem | Δ % |
|-------------------|-----------:|----------:|----:|-----------:|----------:|----:|
| `baseline_*`      |    245,345 |           |     |  1,146,818 |           |     |
| `iter_32`         |    386,297 |           |     |  1,151,154 |           |     |
| `iter_rev_32`     |    364,213 |           |     |  1,151,154 |           |     |
| `iter_48`         |    456,057 |           |     |  1,153,706 |           |     |
| `iter_rev_48`     |    422,837 |           |     |  1,153,706 |           |     |
| `iter_96`         |    665,721 |           |     |  1,162,898 |           |     |
| `iter_rev_96`     |    599,093 |           |     |  1,162,898 |           |     |
| `iter_192`        |  1,086,777 |           |     |  1,188,194 |           |     |
| `iter_rev_192`    |    953,333 |           |     |  1,188,194 |           |     |

## Iteration-only (WASM, baseline-subtracted)

Each cell is the corresponding WASM-metered cell with the `baseline` row
subtracted, isolating the cost attributable to the iteration itself rather
than to dispatch and operand passing. The `baseline` cost is constant across
sizes (245,345 CPU, 1,146,818 mem bytes) because it never reads the bytes.

| Bench             | Before CPU | After CPU | Δ % | Before Mem | After Mem | Δ % |
|-------------------|-----------:|----------:|----:|-----------:|----------:|----:|
| `iter_32`         |    140,952 |           |     |      4,336 |           |     |
| `iter_rev_32`     |    118,868 |           |     |      4,336 |           |     |
| `iter_48`         |    210,712 |           |     |      6,888 |           |     |
| `iter_rev_48`     |    177,492 |           |     |      6,888 |           |     |
| `iter_96`         |    420,376 |           |     |     16,080 |           |     |
| `iter_rev_96`     |    353,748 |           |     |     16,080 |           |     |
| `iter_192`        |    841,432 |           |     |     41,376 |           |     |
| `iter_rev_192`    |    707,988 |           |     |     41,376 |           |     |
