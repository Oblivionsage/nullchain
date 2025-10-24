# NullChain Security Audit

## Critical Issues Found

### 1. Private Key Storage
- **Issue**: Keys stored as plaintext hex
- **Fix**: Encrypt with passphrase, use secure deletion

### 2. Memory Safety
- **Issue**: Sensitive data not wiped from memory
- **Fix**: Implement zeroize for all key material

### 3. Timing Attacks
- **Issue**: Some operations not constant-time
- **Fix**: Use constant-time comparisons

### 4. Random Number Generation
- **Status**: Using OsRng (GOOD)
- **Note**: Keep using cryptographically secure RNG

## Hardening Plan

1. Add zeroize for sensitive data
2. Encrypt private keys at rest
3. Add constant-time operations
4. Minimize dependencies
5. Add security documentation
6. Code style cleanup
