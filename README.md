
## Study Project: Blockchain (Rust)

This repository contains a study/learning implementation of basic blockchain concepts written in Rust. The main goal is to serve as a hands-on lab to understand blocks, block linking, a simple proof-of-work, persistent storage, and a small CLI.

### Goals

- Learn the fundamental components of a blockchain (block, header, hash, nonce, Merkle tree).
- Implement a simple chain with persistence (using `sled`).
- Understand serialization/deserialization with `bincode` and `serde`.
- Provide a CLI to create and inspect blocks and the chain.

### Features

- Create blocks with arbitrary data.
- Link blocks by validating previous block hashes.
- Persistent storage of blocks on disk (under `data/blocks`).
- Binary serialization for efficient storage.
- Uses common crates: `sha2`, `bincode`, `serde`, `sled`, `clap`.

### Repository layout

Key files and folders:

- `Cargo.toml` — project manifest and dependencies.
- `src/main.rs` — application entry point (CLI).
- `src/cli.rs` — command-line parsing and options (`clap`).
- `src/block.rs` — block definition and logic (hashing, nonce, optional signatures).
- `src/blockchain.rs` — chain structure and operations (add, validate, iterate).
- `src/lib.rs` — reusable library code.
- `data/blocks/` — on-disk data directory (created at runtime).

Note: File names may evolve; the list above reflects the current expected organization for this learning project.

### Highlighted dependencies

As declared in `Cargo.toml`:

- `sha2` — SHA-256 hashing to compute block identifiers.
- `bincode` — binary serialization for efficient block storage.
- `serde` / `serde_json` — serialization/deserialization (JSON optional for debugging/inspection).
- `sled` — embedded key-value database for persistence.
- `clap` — command-line parsing for the CLI.
- `anyhow`, `failure` — convenience for error handling during development.

### Building

1. Install Rust (rustup). A recent stable toolchain is recommended.
2. From the project root run:

```bash
cargo build
```

For an optimized build:

```bash
cargo build --release
```

### Running (usage examples)

The project exposes a CLI. Below are generic examples — update as needed to match the real flags in `src/cli.rs`:

Show help / available commands:

```bash
cargo run -- --help
```

Add a new block (example):

```bash
cargo run addblock "test"
```

Show chain state / all blocks:

```bash
cargo run printchain
```

Clear the blockchain (remove persisted data):

```bash
cargo run clear
```

The real subcommands implemented in `src/cli.rs` are `addblock <DATA>`, `printchain` and `clear`.

### Architecture and design notes

- Blocks: defined in `src/block.rs`. Each block contains a header (previous block hash, timestamp, nonce, optional Merkle root) and a payload (transactions or arbitrary data).
- Hashing: SHA-256 (`sha2`) is used to produce block identifiers.
- Merkle: a Merkle tree (e.g. via `merkle-cbt`) may be used to compress transaction sets and produce a Merkle root.
- Persistence: `sled` provides an embedded key-value store; blocks are serialized with `bincode` and stored under an appropriate key (for example block index or block hash).
- CLI: `clap` provides subcommands and flags for common operations.

Design decisions:

- Simplicity over completeness: this project is educational. Networked P2P, real digital signatures, and distributed consensus are out of scope.
- Binary storage: `bincode` + `serde` for compact and fast persistence and easy Rust object round-trips.

### Tests

Run automated tests (if any) with:

```bash
cargo test
```

If tests are not present yet, adding unit tests for `block.rs` and `blockchain.rs` is recommended (hashing correctness, chain validation, serialization round-trip).

### Local data

- Persistent data used by the application is stored under `data/blocks` by default. Be careful when removing or cleaning this directory if you want to reset the chain.

### Suggested next steps

1. Add unit tests for block and chain operations.
2. Implement a simple proof-of-work (PoW) and configurable difficulty.
3. Add snapshot export/import (JSON or binary) for inspection and backups.
4. Document public APIs with doc comments and generate docs via `cargo doc`.
5. Create tutorial notes or a short screencast showing block creation and validation.

### Contributing

Contributions are welcome — issues and text corrections are useful. For code contributions:

1. Fork the repository.
2. Create a branch with a single responsibility (e.g. `feat/pow`, `fix/serialization-test`).
3. Open a pull request describing the changes and test cases.

### License

Add your preferred license here (e.g. MIT, Apache-2.0). If you haven't chosen one yet, add a `LICENSE` file to the repository with the desired license.

---

I can also:

- Produce a README in Portuguese as well.
- Create initial unit tests for `block.rs` and `blockchain.rs`.
- Read `src/*.rs` and update the README with exact CLI flags and examples.

Tell me which next step you want and I will continue.
