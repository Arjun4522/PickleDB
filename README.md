# PickleDB

[![Rust](https://img.shields.io/badge/rust-1.75+-blue.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

**PickleDB** is an encrypted embedded database engine for Rust. It stores opaque encrypted payloads on disk and never sees plaintext data or encryption keys. All cryptographic operations — encryption, decryption, key derivation, and search token generation — happen on the **trusted client side**, keeping the engine itself untrusted.

## Features

- **Encryption-first**: Every record is encrypted with AES-256-GCM before reaching the engine. The engine stores and retrieves ciphertext only.
- **Client-side key management**: A master key (provided by the caller) derives separate encryption and search keys via HKDF-SHA256. The engine never has access to any key material.
- **Blind search**: Search tokens are HMAC-SHA256 digests of field/value pairs, generated client-side. The engine indexes and matches tokens without ever knowing what they represent.
- **Slotted pages**: 4096-byte pages with a compact slot array growing from the top and record data growing from the bottom. Supports insert, delete, update, and in-place compaction.
- **Write-Ahead Log (WAL)**: Append-only crash recovery with checkpoint support. Every mutation is logged before it is applied.
- **Thread-safe**: All components are `Send + Sync` and use `parking_lot::RwLock` for concurrency.
- **Zero unsafe code**: The library crate tree contains no `unsafe` blocks.
- **C FFI**: Stable C ABI bindings for use from other languages (`cdylib` + `staticlib`).
- **CLI**: Batteries-included command-line tool for database operations.
- **Modular**: 10 workspace crates with clean trait boundaries.

## Security Model

```
┌─────────────────────────────────────────┐     ┌──────────────────┐
│            Trusted Client               │     │   Untrusted      │
│                                         │     │   Engine         │
│  Master Key ──► HKDF-SHA256 ──► K_enc  │     │                  │
│                           ──► K_search │     │  ┌────────────┐  │
│                                         │     │  │ WAL        │  │
│  encrypt(record_id, plaintext) ─────────┼────►│  │            │  │
│  decrypt(record_id, payload) ◄──────────┼─────│  │ Pages      │  │
│  derive_search_token(field, value) ────┼────►│  │ (ciphertxt)│  │
│  search(token) ◄───────────────────────┼─────│  └────────────┘  │
└─────────────────────────────────────────┘     └──────────────────┘
```

- The **client** owns the master key, encrypts/decrypts data, generates search tokens, and encrypts search fields before sending them to the engine.
- The **engine** stores opaque encrypted bytes, manages the on-disk page layout, maintains a search index over blind tokens, and recovers from crashes via the WAL.
- The engine **never** possesses encryption keys, never sees plaintext, and never inspects search token contents.

## Quick Start

```bash
# Initialize a database
pickledb-cli /tmp/mydb init

# Set your encryption key (32 bytes)
export PICKLEDB_KEY="0123456789abcdef0123456789abcdef"

# Create (encrypts before storing)
pickledb-cli /tmp/mydb insert 42 "hello world"

# Read (retrieves and decrypts)
pickledb-cli /tmp/mydb get 42
# → Record 42: hello world

# Delete
pickledb-cli /tmp/mydb delete 42
```

## Architecture

The project is structured as a Rust workspace with 10 crates:

| Crate | Description |
|-------|-------------|
| `pickledb-core` | Core types (`RecordId`, `PageId`, `SearchToken`, `EncryptedPayload`, etc.), error hierarchy, and trait definitions (`Engine`, `Client`, `PageManager`, `Wal`, `Index`, `Cache`) |
| `pickledb-crypto` | Client-side cryptography: AES-256-GCM encryptor, HMAC-SHA256 search token generator, HKDF-SHA256 key derivation, and the `PickleClient` implementation |
| `pickledb-pages` | `SlottedPage` (4096-byte buffer with binary header, slot array, and compacting data region) and `PageAllocator` (free list management) |
| `pickledb-storage` | `FileManager` (page-level I/O at fixed `page_id * 4096` offsets) and `BufferPool` (thread-safe LRU-like page cache) |
| `pickledb-wal` | `WalLog` (append-only, length-prefixed bincode entries, checkpoint truncation, crash replay) and recovery logic |
| `pickledb-index` | `HashIndex` — an in-memory `HashMap<SearchToken, Vec<RecordId>>` for blind search |
| `pickledb-cache` | Page cache abstractions (currently a dependency of `pickledb-storage`) |
| `pickledb-engine` | `PickleEngine` — composes all subsystems into the final database engine with WAL crash recovery and `record_map` rebuilding |
| `pickledb-cli` | Command-line interface with `init`, `insert`, `get`, `search`, `delete`, `sync`, `checkpoint`, `compact`, and `stats` commands |
| `pickledb-ffi` | C ABI bindings (`pickledb_open`, `pickledb_close`, `pickledb_insert`, `pickledb_get`, `pickledb_search`, `pickledb_sync`) via `cdylib` and `staticlib` |

## Using the C API

```c
#include "pickledb.h"

pickledb_t* db = pickledb_open("/path/to/db");
if (!db) { /* error */ }

uint8_t data[] = {0x01, 0x02, 0x03};
pickledb_result_t res = pickledb_insert(db, 42, data, 3);

// ... use with get, search, sync ...

pickledb_close(db);
```

## Building from Source

```bash
# Build all crates
cargo build --workspace

# Run all tests
cargo test --workspace

# Build the CLI only
cargo build -p pickledb-cli

# Build the FFI shared/static library
cargo build -p pickledb-ffi --lib
```

### Requirements

- Rust **1.75** or later (edition 2021)
- Dependencies are limited to audited crates: `aes-gcm`, `sha2`, `hkdf`, `hmac`, `parking_lot`, `serde`, `bincode`, `thiserror`, `hex`, `anyhow`, `rand`

## Project Structure

```
├── Cargo.toml                # Workspace manifest
├── README.md
├── docs/
│   └── aegisdb_arch.pdf      # Architecture specification
├── tests/                    # Integration tests
├── crates/
│   ├── core/                 # Core types, errors, traits
│   ├── crypto/               # Client-side crypto (AES-256-GCM, HKDF, HMAC)
│   ├── pages/                # Slotted page + page allocator
│   ├── storage/              # File manager + buffer pool
│   ├── wal/                  # Write-ahead log + recovery
│   ├── cache/                # Page cache abstractions
│   ├── index/                # Blind search index
│   ├── engine/               # Database engine
│   ├── cli/                  # Command-line tool
│   └── ffi/                  # C ABI bindings
```

## Test Suite

```bash
cargo test --workspace
```

Run by crate:

```
pickledb-core:   11 tests
pickledb-crypto: 27 tests
pickledb-pages:  22 tests
pickledb-storage: 11 tests
pickledb-wal:    10 tests
pickledb-index:  11 tests
pickledb-engine:  8 tests
pickledb-ffi:     4 tests
────────────────────────
Total:          104 tests
```

## License

MIT
