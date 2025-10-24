# Contributing to NullChain

Thank you for your interest in contributing to NullChain. This document provides guidelines for contributions.

## Development Environment

### Prerequisites

- Rust 1.89.0 or later
- Git

### Supported Platforms

- Linux (Ubuntu 20.04+, Debian 11+, Arch, Fedora)
- macOS (11.0+)
- Windows (10/11 with WSL2 recommended)

### Setup
```bash
git clone https://github.com/Oblivionsage/nullchain.git
cd nullchain
cargo build
cargo test
```

## Code Standards

### Before Submitting

1. **Run tests**: `cargo test --all`
2. **Run clippy**: `cargo clippy -- -D warnings`
3. **Format code**: `cargo fmt --all`
4. **Check all platforms**: CI will test Ubuntu, macOS, Windows

### Code Quality

- Write unit tests for all new functionality
- Document public APIs with rustdoc comments
- Keep functions small and focused
- Avoid unnecessary dependencies
- Use meaningful variable names

### Cryptographic Code

Cryptographic code requires extra scrutiny:
- Must have comprehensive tests
- Should be reviewed by multiple developers
- Consider formal verification for critical components
- Document security assumptions

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Commit Messages

Follow conventional commits:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation only
- `test:` Adding tests
- `refactor:` Code refactoring
- `perf:` Performance improvement
- `chore:` Maintenance tasks

Example: `feat: add shielded transaction support`

## Testing

### Running Tests
```bash
# All tests
cargo test

# Specific crate
cargo test --package nullchain-types

# Specific test
cargo test test_genesis_mining

# With output
cargo test -- --nocapture
```

### Test Coverage

Aim for >80% code coverage on new functionality.

## Security

Please report security vulnerabilities to the maintainers privately before public disclosure. See SECURITY.md for details.

## Questions?

Open an issue for discussion before starting major work.
