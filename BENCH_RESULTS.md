# `Bytes` / `BytesN` iteration bench

Branch: `dc/bytes-iter-bench` (off `upstream/main`, commit `e5cb4b52`).

Benches iterating every byte of a `Bytes` via `Bytes::iter()`, forwards
(`sum_iter`) and in reverse (`sum_iter_rev`). `BytesN` iterates through the
same `BytesIter`, so the results apply to both.

## Host-side (host-function metering only)

Run: `cargo test --release --package soroban-sdk --lib --features testutils -- tests::bytes_iter_bench:: --ignored --nocapture`

| Bench             | Before CPU | After CPU |    Δ % | Before Mem | After Mem |   Δ % |
|-------------------|-----------:|----------:|-------:|-----------:|----------:|------:|
| `iter_32`         |     48,602 |     4,026 | -91.7% |      3,568 |         0 | -100% |
| `iter_rev_32`     |     44,698 |     4,026 | -91.0% |      3,568 |         0 | -100% |
| `iter_48`         |     72,938 |     5,978 | -91.8% |      5,736 |         0 | -100% |
| `iter_rev_48`     |     67,082 |     5,978 | -91.1% |      5,736 |         0 | -100% |
| `iter_96`         |    146,330 |    11,834 | -91.9% |     13,776 |         0 | -100% |
| `iter_rev_96`     |    134,618 |    11,834 | -91.2% |     13,776 |         0 | -100% |
| `iter_192`        |    294,842 |    23,546 | -92.0% |     36,768 |         0 | -100% |
| `iter_rev_192`    |    271,418 |    23,546 | -91.3% |     36,768 |         0 | -100% |

## WASM-metered (guest WASM + host-function metering)

Run: `make build-test-wasms && cargo test --release --package soroban-sdk --lib --features testutils -- tests::bytes_iter_bench_wasm --ignored --nocapture`

| Bench             | Before CPU | After CPU |    Δ % | Before Mem | After Mem |   Δ % |
|-------------------|-----------:|----------:|-------:|-----------:|----------:|------:|
| `baseline_*`      |    245,345 |   232,140 |  -5.4% |  1,146,818 | 1,144,946 | -0.2% |
| `iter_32`         |    386,297 |   258,596 | -33.1% |  1,151,154 | 1,144,946 | -0.5% |
| `iter_rev_32`     |    364,213 |   259,068 | -28.9% |  1,151,154 | 1,144,946 | -0.5% |
| `iter_48`         |    456,057 |   271,268 | -40.5% |  1,153,706 | 1,144,946 | -0.8% |
| `iter_rev_48`     |    422,837 |   271,740 | -35.7% |  1,153,706 | 1,144,946 | -0.8% |
| `iter_96`         |    665,721 |   309,284 | -53.5% |  1,162,898 | 1,144,946 | -1.5% |
| `iter_rev_96`     |    599,093 |   309,756 | -48.3% |  1,162,898 | 1,144,946 | -1.5% |
| `iter_192`        |  1,086,777 |   385,316 | -64.5% |  1,188,194 | 1,144,946 | -3.6% |
| `iter_rev_192`    |    953,333 |   385,788 | -59.5% |  1,188,194 | 1,144,946 | -3.6% |

## Iteration-only (WASM, baseline-subtracted)

Each cell is the corresponding WASM-metered cell with the `baseline` row
subtracted, isolating the cost attributable to the iteration itself rather
than to dispatch and operand passing. The `baseline` cost is constant across
sizes (before: 245,345 CPU / 1,146,818 mem; after: 232,140 CPU / 1,144,946
mem) because it never reads the bytes.

| Bench             | Before CPU | After CPU |    Δ % | Before Mem | After Mem |   Δ % |
|-------------------|-----------:|----------:|-------:|-----------:|----------:|------:|
| `iter_32`         |    140,952 |    26,456 | -81.2% |      4,336 |         0 | -100% |
| `iter_rev_32`     |    118,868 |    26,928 | -77.3% |      4,336 |         0 | -100% |
| `iter_48`         |    210,712 |    39,128 | -81.4% |      6,888 |         0 | -100% |
| `iter_rev_48`     |    177,492 |    39,600 | -77.7% |      6,888 |         0 | -100% |
| `iter_96`         |    420,376 |    77,144 | -81.6% |     16,080 |         0 | -100% |
| `iter_rev_96`     |    353,748 |    77,616 | -78.1% |     16,080 |         0 | -100% |
| `iter_192`        |    841,432 |   153,176 | -81.8% |     41,376 |         0 | -100% |
| `iter_rev_192`    |    707,988 |   153,648 | -78.3% |     41,376 |         0 | -100% |
