# simple-execution-events

A simple Rust consumer for Monad execution events.

## Usage

```bash
cargo run
```

Connects to the Monad event ring at `/var/lib/hugetlbfs/user/monad/pagesize-2MB/event-rings/monad-exec-events` and prints block and transaction events.

## Requirements

- Rust 2024 edition (1.85+)
- A running Monad node with execution events enabled ([setup guide](https://docs.monad.xyz/guides/setup-execution-events))
