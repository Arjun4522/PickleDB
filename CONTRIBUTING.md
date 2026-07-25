# Contributing to PickleDB

Thank you for your interest in contributing! PickleDB follows the same standards as production database projects like PostgreSQL, SQLite, and DuckDB.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Code Style](#code-style)
- [Testing](#testing)
- [Documentation](#documentation)
- [Pull Request Process](#pull-request-process)
- [Release Process](#release-process)

## Code of Conduct

All contributors must adhere to our [Code of Conduct](CODE_OF_CONDUCT.md).

## Getting Started

```bash
# Clone the repository
git clone https://github.com/pickledb/pickledb.git
cd pickledb

# Build all crates
cargo build --workspace

# Run all tests
cargo test --workspace
```

### Prerequisites

- Rust **1.75** or later
- No external dependencies (pure Rust)

## Development Workflow

1. **Fork** the repository
2. **Create a feature branch**: `git checkout -b feat/your-feature`
3. **Make changes** following the code style
4. **Write tests** for all new functionality
5. **Run tests**: `cargo test --workspace`
6. **Run lints**: `cargo clippy --workspace -- -D warnings`
7. **Format**: `cargo fmt --all`
8. **Commit** with a descriptive message
9. **Push** to your fork
10. **Open a pull request**

## Code Style

We follow the [PickleDB Style Guide](STYLE_GUIDE.md). Key points:

- Default to **idiomatic Rust** with clear naming
- **Zero unsafe code** in library crates (FFI is the only exception)
- **Document all public items** with doc comments
- **No TODO comments** in committed code
- Error messages must be **user-friendly and actionable**
- All public APIs must be **Send + Sync**
- Prefer `thiserror` for error types

## Testing

- All new features must include tests
- Tests should cover: happy path, edge cases, and error conditions
- Integration tests go in `tests/` at the workspace root
- Unit tests go in a `#[cfg(test)] mod tests` block at the bottom of each source file
- Property-based testing is encouraged for cryptographic and storage components

```bash
# Run full test suite
cargo test --workspace

# Run tests with all features
cargo test --workspace --all-features

# Run a specific crate's tests
cargo test -p pickledb-engine

# Run benchmarks
cargo bench
```

## Documentation

- All public APIs must have doc comments
- Examples in doc comments must compile (`cargo test` will verify)
- Architecture decisions should be documented in `docs/`
- README examples must be tested
- Update `CHANGELOG.md` with each significant change

## Pull Request Process

1. Ensure all CI checks pass
2. Update documentation (doc comments, README, docs/)
3. Add a changelog entry
4. Request review from maintainers
5. Address review feedback
6. Squash commits before merge

### PR Title Convention

```
<type>(<scope>): <description>
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`, `security`

Examples:
- `feat(engine): add WAL checkpoint compression`
- `fix(crypto): handle empty plaintext in AES-GCM`
- `docs(cli): document shell completion`
- `security(crypto): constant-time comparison for search tokens`

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Tag release: `git tag v0.1.x`
4. Push tag: `git push origin v0.1.x`
5. CI builds and publishes to crates.io
6. GitHub Release is created with release notes

## Project Structure

```
crates/
  core/       Core types, errors, traits
  crypto/     Client-side cryptography
  pages/      Slotted page implementation
  storage/    File manager + buffer pool
  wal/        Write-ahead log + recovery
  cache/      Page cache abstractions
  index/      Blind search index
  engine/     Database engine
  cli/        CLI binary
  ffi/        C ABI bindings
tests/        Integration tests
docs/         Documentation
benches/      Benchmarks
```

## Getting Help

- Open a [Discussion](https://github.com/pickledb/pickledb/discussions)
- Join our [Discord](https://discord.gg/pickledb)
- Read the [Architecture Guide](ARCHITECTURE.md)
