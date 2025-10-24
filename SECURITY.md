# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

If you discover a security vulnerability in NullChain, please report it privately:

1. **GitHub Security Advisory**: Use GitHub's private security advisory feature
2. **Email**: security@nullchain.org (if available)
3. **Response Time**: The NullChain team aims to respond within 48 hours
4. **Updates**: You will receive regular updates on the progress

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)
- Your contact information (for follow-up)

### Disclosure Policy

- The team will acknowledge receipt within 48 hours
- Initial assessment provided within 7 days
- We will work on a fix and coordinate disclosure timing with you
- Security researchers will be credited in release notes (unless anonymity is preferred)

### Bug Bounty

Currently, NullChain does not have a formal bug bounty program. However, the development team deeply appreciates security researchers and will acknowledge all contributions.

## Security Best Practices

When using NullChain:

- Keep your software up to date
- Never share private keys
- Use hardware wallets for large amounts
- Verify transaction details before signing
- Run your own node when possible

## Cryptographic Algorithms

NullChain uses industry-standard cryptographic primitives:
- **Hashing**: Blake3
- **Signatures**: Ed25519 (current), BLS12-381 (planned)
- **ZK Proofs**: Plonky2/Halo2 (planned)

All cryptographic primitives use well-audited libraries maintained by the Rust community.

## Contact

For security-related inquiries, contact the NullChain development team through:
- GitHub Security Advisories
- GitHub Issues (for non-sensitive matters)
