# sorostream-contracts

![Rust](https://img.shields.io/badge/Rust-1.84+-orange?logo=rust)
![Soroban SDK](https://img.shields.io/badge/soroban--sdk-22.0.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![CI](https://github.com/SoroStream/sorostream-contracts/actions/workflows/test.yml/badge.svg)

Soroban smart contracts for **SoroStream** — a real-time payment streaming protocol on Stellar. Stream USDC by the second for salaries, subscriptions, vesting schedules, and grant disbursements.

## How It Works

1. **Sender** calls `create_stream()` locking USDC for a recipient over a defined duration.
2. Contract computes `flow_rate = amount / duration_seconds`.
3. **Recipient** calls `withdraw()` at any time to claim `flow_rate × elapsed_seconds`.
4. **Sender** can `cancel_stream()` — recipient gets earned amount, sender gets remainder.
5. **Sender** can `top_up()` to add more USDC, automatically extending the end time.
6. Streams can `auto_renew` — restarting automatically on completion.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust 1.84+ |
| Smart Contract SDK | soroban-sdk 22.0.0 |
| CLI | stellar-cli |
| CI | GitHub Actions |

## Local Setup

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Add WASM target
rustup target add wasm32-unknown-unknown

# 3. Install Stellar CLI
cargo install --locked stellar-cli --features opt

# 4. Run tests
cargo test

# 5. Lint
cargo clippy -- -D warnings

# 6. Build WASM
stellar contract build
```

## Contract Function Reference

| Function | Description |
|----------|-------------|
| `create_stream(sender, recipient, token, amount, duration_seconds, auto_renew)` | Creates a new stream, returns `stream_id` |
| `withdraw(stream_id, recipient)` | Recipient claims all earned tokens |
| `cancel_stream(stream_id, sender)` | Cancels stream, splits balance |
| `top_up(stream_id, sender, amount)` | Adds tokens, extends duration |
| `get_stream(stream_id)` | Returns full `Stream` struct |
| `get_claimable(stream_id)` | Returns currently claimable amount |
| `get_streams_by_sender(sender)` | Returns all streams for a sender |
| `get_streams_by_recipient(recipient)` | Returns all streams for a recipient |

## Testnet Deployment

| Contract | Address |
|----------|---------|
| StreamContract | See [deployments/testnet.json](./deployments/testnet.json) |

## Contributing via Drips Wave

This project participates in the **Stellar Wave Program** on [Drips Wave](https://drips.network/wave). Contributors earn rewards for resolving issues during weekly Wave sprints — funded by the Stellar Development Foundation, free for contributors to participate.

See [CONTRIBUTING.md](./CONTRIBUTING.md) for the full workflow.

## Open Issues

### Issue #1 — [Trivial] Add doc comments to all public contract functions in lib.rs

**Description:** Several public functions in `contracts/stream/src/lib.rs` are missing inline doc comments (`///`). Add complete Rustdoc comments to every public function explaining parameters, return values, and behaviour.

**Acceptance Criteria:**
- [ ] All public functions in `lib.rs` have `///` doc comments
- [ ] `cargo doc --no-deps` runs without warnings
- [ ] `cargo test` passes

**Complexity:** `trivial` | `good first issue`

---

### Issue #2 — [Trivial] Write unit test for top_up when stream has already partially elapsed

**Description:** The existing `test_top_up_extends_duration` test top-ups at time 0. Add a test that advances the ledger to 50% of the stream duration before calling `top_up()`, and asserts the new end time is correct relative to the already-elapsed time.

**Acceptance Criteria:**
- [ ] New test `test_top_up_after_partial_elapsed` added to `test.rs`
- [ ] Test advances ledger to mid-stream before top_up
- [ ] Asserts `end_time` extends correctly
- [ ] `cargo test` passes

**Complexity:** `trivial` | `good first issue`

---

### Issue #3 — [Trivial] Add StreamCompleted event emitted when stream naturally reaches end_time

**Description:** When `withdraw()` is called and `now >= stream.end_time` with `auto_renew = false`, the stream transitions to `Completed`. A `StreamCompleted` event should be emitted at this point. The event function exists in `events.rs` but needs to be wired into the right place in `lib.rs`.

**Acceptance Criteria:**
- [ ] `stream_completed()` is called in `withdraw()` when status transitions to `Completed`
- [ ] Event appears in test output via `env.events()`
- [ ] `cargo test` passes

**Complexity:** `trivial` | `good first issue`

---

### Issue #4 — [Medium] Implement get_streams_by_recipient pagination (limit + offset params)

**Description:** `get_streams_by_recipient()` currently returns all streams for an address. For addresses with many streams this is expensive. Add `limit: u32` and `offset: u32` parameters to support paginated queries.

**Acceptance Criteria:**
- [ ] Function signature updated to `get_streams_by_recipient(env, recipient, limit, offset)`
- [ ] Returns at most `limit` streams starting from `offset`
- [ ] Unit test covering pagination boundary conditions
- [ ] `cargo test` and `cargo clippy` pass

**Complexity:** `medium`

---

### Issue #5 — [Medium] Add stream pause/resume functionality

**Description:** Implement `pause_stream(stream_id, sender)` and `resume_stream(stream_id, sender)`. When paused, time stops accumulating for the recipient. When resumed, the stream continues from where it left off, adjusting `end_time` by the paused duration.

**Acceptance Criteria:**
- [ ] `StreamStatus::Paused` variant added to enum
- [ ] `pause_stream()` and `resume_stream()` functions implemented
- [ ] `paused_at: Option<u64>` field added to `Stream` struct
- [ ] `end_time` adjusted on resume to account for paused duration
- [ ] Events `StreamPaused` and `StreamResumed` emitted
- [ ] Unit tests for pause/resume time accounting
- [ ] `cargo test` passes

**Complexity:** `medium`

---

### Issue #6 — [High] Implement multi-token streaming

**Description:** Allow a stream to split its flow across 2 different SAC-compatible tokens (e.g. 50% USDC + 50% EURC). Extend `create_stream()` to accept an optional second token address and split ratio, and update `withdraw()` and `cancel_stream()` to handle both tokens proportionally.

**Acceptance Criteria:**
- [ ] `Stream` struct supports optional `token_b: Option<Address>` and `split_ratio: u32` (basis points, e.g. 5000 = 50%)
- [ ] `create_stream()` accepts and locks both tokens proportionally
- [ ] `withdraw()` transfers both tokens proportionally
- [ ] `cancel_stream()` refunds both tokens proportionally
- [ ] Unit tests cover dual-token create, withdraw, and cancel
- [ ] `cargo test` and `cargo clippy` pass

**Complexity:** `high`

---

### Issue #7 — [High] Build auto-renew logic: contract re-locks deposit on expiry

**Description:** When `auto_renew = true` and `end_time` passes, the contract should automatically re-lock the same `deposit` amount from the sender and reset `start_time`/`end_time`. Currently this only triggers on `withdraw()`. Implement a dedicated `renew_stream(stream_id)` function callable by anyone, and add a check in `get_claimable()` to account for renewed periods.

**Acceptance Criteria:**
- [ ] `renew_stream(stream_id)` is a callable public function
- [ ] Re-locks `deposit` from sender via token transfer
- [ ] Resets `start_time`, `end_time`, `last_withdraw_time`
- [ ] `StreamRenewed` event emitted
- [ ] `get_claimable()` handles post-renewal timestamps correctly
- [ ] Unit tests for manual renew and multiple renewals
- [ ] `cargo test` passes

**Complexity:** `high`
