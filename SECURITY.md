# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

If you discover a security vulnerability in NullChain, please report it privately:

1. **Email**: Open a private security advisory on GitHub
2. **Response Time**: We aim to respond within 48 hours
3. **Updates**: You will receive updates on the progress

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### Disclosure Policy

- We will acknowledge receipt within 48 hours
- We will provide an initial assessment within 7 days
- We will work on a fix and coordinate disclosure timing with you
- We will credit you in the release notes (unless you prefer anonymity)

### Bug Bounty

Currently, NullChain does not have a formal bug bounty program. However, we deeply appreciate security researchers and will acknowledge contributions.

## Security Best Practices

When using NullChain:

- Keep your software up to date
- Never share private keys
- Use hardware wallets for large amounts
- Verify transaction details before signing
- Run your own node when possible

## Cryptographic Algorithms

NullChain uses:
- **Hashing**: Blake3
- **Signatures**: Ed25519 (current), BLS12-381 (future)
- **ZK Proofs**: Plonky2/Halo2 (future)

All cryptographic primitives use well-audited libraries.
