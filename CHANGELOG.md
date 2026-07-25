# Changelog

All notable changes to PickleDB are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-03-XX

### Added

- **Core engine**: Slotted pages, page allocator, buffer pool, WAL, hash index
- **Client-side encryption**: AES-256-GCM with random nonces, HKDF-SHA256 key derivation
- **Blind search**: HMAC-SHA256 tokens for searchable encryption
- **Crash recovery**: WAL-based recovery with checkpoint support
- **CLI**: Basic command-line tool for database operations
- **C FFI**: C ABI bindings for interoperable use
- **Thread safety**: All components are `Send + Sync` with `parking_lot` locks
- **Zero unsafe code**: Library crates contain no `unsafe` blocks (FFI excluded)
- **104 tests**: Comprehensive unit and integration test suite

### Features

- `init`, `insert`, `get`, `search`, `delete`, `sync`, `checkpoint`, `compact`, `stats` CLI commands
- `PICKLEDB_KEY` environment variable for master key input
- Bincode-serialized WAL entries with length-prefixed format
- FIFO page cache with dirty page tracking
- Free-list-based page allocation with LIFO reuse
- `pickledb_open`, `pickledb_close`, `pickledb_insert`, `pickledb_get`, `pickledb_search`, `pickledb_sync` C API
