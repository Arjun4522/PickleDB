# Security Policy

## Threat Model

PickleDB is designed with a **zero-trust storage model**. The database engine is treated as an untrusted component that never has access to encryption keys or plaintext data.

### Trusted Components

- **PickleClient**: Client-side cryptography (encryption, decryption, key derivation, token generation)
- **Application code**: All plaintext data and keys

### Untrusted Components

- **PickleEngine**: On-disk storage engine (page management, WAL, indexing)
- **On-disk files**: Data files, WAL logs, index metadata
- **Operating system I/O**: File system, block device drivers
- **Backup infrastructure**: Snapshots, replication

## Encryption Model

| Layer | Algorithm | Key Size | Purpose |
|-------|-----------|----------|---------|
| Record encryption | AES-256-GCM | 256-bit | Confidentiality + integrity |
| Key derivation | HKDF-SHA256 | 256-bit input | Derives K_enc + K_search |
| Search tokens | HMAC-SHA256 | 256-bit | Blind search indexing |
| Random nonces | OsRng | 96-bit | Unique IV per record |

## Key Management

- The **master key** is provided by the caller at runtime
- Two sub-keys are derived: `K_enc` (encryption) and `K_search` (search tokens)
- Neither key is ever persisted by PickleDB
- The application is responsible for secure key storage (HSM, keychain, env vars, etc.)

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | ✅ Active development |

## Reporting a Vulnerability

Please **do not** file public GitHub issues for security vulnerabilities.

Contact the maintainers directly at: security@pickledb.dev

You should receive a response within 48 hours. If you do not, please follow up.

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if known)

### Process

1. Report received and acknowledged
2. Maintainers validate the report
3. Fix developed in private
4. Security advisory published
5. Fix released in next patch version

## Security Recommendations

- Always set the `PICKLEDB_KEY` environment variable via secure key management
- Use separate master keys per database instance
- Enable filesystem-level encryption on the database directory
- Restrict file permissions on `.pickledb/` directories
- Run `pickledb verify` periodically to detect corruption
- Enable audit logging in production deployments
