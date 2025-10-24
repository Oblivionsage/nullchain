# Contributing to NullChain

Thank you for your interest in contributing to NullChain. This document provides guidelines for contributions to the project.

## Development Team

NullChain is developed by a distributed team of privacy and cryptography enthusiasts. All contributors are welcome to join the development effort.

## Development Environment

### Prerequisites

- Rust 1.89.0 or later
- Git
- Basic understanding of blockchain and cryptography concepts

### Supported Platforms

- **Linux**: Ubuntu 20.04+, Debian 11+, Arch Linux, Fedora 35+
- **macOS**: 11.0 (Big Sur) or later
- **Windows**: Windows 10/11 (WSL2 recommended for development)

### Setup
```bash
git clone https://github.com/Oblivionsage/nullchain.git
cd nullchain
cargo build
cargo test
```

## Code Standards

### Before Submitting

All contributions must pass the following checks:

1. **Run tests**: `cargo test --all`
2. **Run clippy**: `cargo clippy -- -D warnings`
3. **Format code**: `cargo fmt --all`
4. **CI validation**: GitHub Actions will test on Ubuntu, macOS, and Windows

### Code Quality Standards

- Write comprehensive unit tests for all new functionality
- Document public APIs with rustdoc comments
- Keep functions small, focused, and readable
- Minimize external dependencies (each dependency is a liability)
- Use meaningful, self-documenting variable names
- Follow Rust idioms and best practices

### Cryptographic Code

Cryptographic code requires exceptional scrutiny:
- Must have comprehensive test coverage (>90%)
- Should be reviewed by multiple team members
- Consider formal verification for critical components
- Document all security assumptions explicitly
- Use constant-time operations where applicable

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes with clear, atomic commits
4. Ensure all tests pass locally
5. Push to your fork (`git push origin feature/amazing-feature`)
6. Open a Pull Request with detailed description

### Commit Message Convention

Follow conventional commits format:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation updates
- `test:` Adding or updating tests
- `refactor:` Code refactoring without behavior change
- `perf:` Performance improvements
- `chore:` Maintenance tasks

Example: `feat: add shielded transaction support with zk-SNARKs`

## Testing

### Running Tests
```bash
# Run all tests
cargo test

# Test specific crate
cargo test --package nullchain-types

# Test with output
cargo test -- --nocapture

# Run ignored tests (e.g., slow integration tests)
cargo test -- --ignored
```

### Test Coverage Goals

- Core functionality: >90% coverage
- Cryptographic code: >95% coverage
- General code: >80% coverage

## Architecture Decisions

For major architectural changes:
1. Open an issue for discussion first
2. Provide rationale and alternatives considered
3. Allow time for community feedback
4. Document decision in code comments

## Questions & Discussion

- **Issues**: For bug reports and feature requests
- **Discussions**: For general questions and ideas
- **Pull Requests**: For code contributions

## Code of Conduct

Be respectful, constructive, and professional in all interactions. We're building privacy infrastructure for everyone.

---

Built by the NullChain team and contributors worldwide.
