# WebAuthn Go-Rust Bridge Examples

This directory contains examples demonstrating the three WebAuthn ceremonies as defined in the [go-webauthn library](https://github.com/go-webauthn/webauthn).

## Overview

The WebAuthn specification supports three distinct authentication ceremonies, each designed for different use cases. This implementation exposes all three through a Rust-Go bridge using rust2go.

## The Three Ceremonies

### 1. Credential Creation Ceremony

**File:** [`credential_creation.rs`](./credential_creation.rs)

**Purpose:** Register a new authenticator credential for a user.

**Functions:**
- `signup_begin()` → Uses `BeginMediatedRegistration`
- `signup_finish()` → Uses `FinishRegistration` (via `CreateCredential`)

**Use Cases:**
- First-time user registration
- Adding a new authenticator to an existing account
- Upgrading from password to passkey

**Key Features:**
- Supports three scenarios: MFA, Passwordless, Usernameless
- Configures resident key requirements based on scenario
- Uses exclusion list to prevent duplicate registration
- Validates credential properties

**Run:**
```bash
cargo run --example credential_creation
```

### 2. Passkey Login Ceremony

**File:** [`passkey_login.rs`](./passkey_login.rs)

**Purpose:** Authenticate using discoverable credentials (no username required).

**Functions:**
- `passkey_login_begin()` → Uses `BeginDiscoverableMediatedLogin`
- `passkey_login_finish()` → Uses `FinishPasskeyLogin` (via `ValidatePasskeyLogin`)

**Use Cases:**
- Passwordless authentication
- Single-gesture sign-in
- Enhanced user experience (no typing)

**Key Features:**
- No username required from user
- `allowCredentials` list is EMPTY
- Server discovers user from credential's `userHandle`
- Requires discoverable/resident key support

**Requirements:**
- Authenticator must support resident keys
- Credential must be registered as discoverable
- User must have registered on this device/authenticator

**Run:**
```bash
cargo run --example passkey_login
```

### 3. Multi-Factor Login Ceremony

**File:** [`multifactor_login.rs`](./multifactor_login.rs)

**Purpose:** Second-factor authentication after password/email login.

**Functions:**
- `mfa_login_begin()` → Uses `BeginMediatedLogin`
- `mfa_login_finish()` → Uses `FinishLogin` (via `ValidateLogin`)

**Use Cases:**
- 2FA after password login
- Step-up authentication (before sensitive operations)
- Gradual migration from passwords to passkeys

**Key Features:**
- User is already authenticated (1st factor complete)
- Server knows which user is logging in
- `allowCredentials` list is POPULATED with user's credentials
- Authenticator only allows user's own credentials

**Requirements:**
- User must be authenticated via first factor
- User must have at least one registered credential
- Credential does not need to be discoverable

**Run:**
```bash
cargo run --example multifactor_login
```

## Comparison Table

| Feature | Credential Creation | Passkey Login | Multi-Factor Login |
|---------|-------------------|---------------|-------------------|
| **Purpose** | Register credential | Sign in | 2nd factor |
| **User known?** | Yes | No | Yes |
| **Username required?** | Yes | No | Yes |
| **Discoverable credential?** | Varies by scenario | Required | Optional |
| **allowCredentials** | Exclusions only | Empty | Filled |
| **User verification** | Scenario-dependent | Required | Varies |
| **Begin function** | `BeginMediatedRegistration` | `BeginDiscoverableMediatedLogin` | `BeginMediatedLogin` |
| **Finish function** | `FinishRegistration` | `FinishPasskeyLogin` | `FinishLogin` |

## Architecture

### Rust Side (`src/lib.rs`)

Defines the request/response types and trait for all three ceremonies:

```rust
pub trait WebAuthnBridge {
    // Credential Creation
    fn signup_begin(req: &SignupBeginRequest) -> SignupBeginResponse;
    fn signup_finish(req: &SignupFinishRequest) -> SignupFinishResponse;
    
    // Passkey Login
    fn passkey_login_begin(req: &PasskeyLoginBeginRequest) -> PasskeyLoginBeginResponse;
    fn passkey_login_finish(req: &PasskeyLoginFinishRequest) -> PasskeyLoginFinishResponse;
    
    // Multi-Factor Login
    fn mfa_login_begin(req: &MfaLoginBeginRequest) -> MfaLoginBeginResponse;
    fn mfa_login_finish(req: &MfaLoginFinishRequest) -> MfaLoginFinishResponse;
}
```

### Go Side (`go/impl.go`)

Implements the trait using the go-webauthn library:

- Manages users, credentials, and sessions in-memory
- Calls appropriate go-webauthn functions
- Handles JSON serialization/deserialization
- Validates credentials and updates counters

### Bridge Layer

The rust2go framework automatically generates:
- CGO bindings from Rust to Go
- Type conversions between Rust and Go
- Async/await support for Go functions

## Mediation Modes

All ceremonies support different mediation modes that control the browser's UI behavior:

| Mode | Description | Use Case |
|------|-------------|----------|
| `default` | Browser decides | Let browser handle it |
| `optional` | UI may be shown | Standard flow |
| `required` | Always show UI | Force user interaction |
| `conditional` | Autofill UI | Form field integration |
| `silent` | No UI if possible | Background authentication |

## Session Management

**Critical Security Requirement:** Session data must be stored securely!

- Store `SessionData` server-side (never client-side)
- Bind session to user agent (use opaque session cookies)
- Session is one-time use (deleted after finish)
- Session must not be modifiable by client

## Credential Counter

All ceremonies update the credential's sign counter:

- Increments on each authentication
- Detects cloned/compromised authenticators
- Must be persisted after each validation
- Rollback indicates possible attack

## Reference Documentation

- **Go Library:** [github.com/go-webauthn/webauthn](https://github.com/go-webauthn/webauthn)
- **Ceremonies:** `reference/webauthn/webauthn/doc.go`
- **Registration Example:** `reference/webauthn/webauthn/registration_test.go`
- **Passkey Example:** `reference/webauthn/webauthn/example_passkey_test.go`
- **MFA Example:** `reference/webauthn/webauthn/example_multifactor_test.go`
- **WebAuthn Spec:** [W3C WebAuthn](https://www.w3.org/TR/webauthn-2/)

## Testing Flow

These examples only test the Rust-Go bridge. A complete WebAuthn flow requires:

1. **Registration:**
   - Server: `signup_begin()` → challenge JSON
   - Browser: `navigator.credentials.create(challenge)`
   - User authenticates with device
   - Browser: credential → server
   - Server: `signup_finish(credential)` → stores credential

2. **Passkey Login:**
   - Server: `passkey_login_begin()` → challenge JSON
   - Browser: `navigator.credentials.get(challenge)`
   - Authenticator shows user list
   - User selects and authenticates
   - Browser: assertion → server
   - Server: `passkey_login_finish(assertion)` → creates session

3. **Multi-Factor Login:**
   - User enters username + password
   - Server validates password (1st factor)
   - Server: `mfa_login_begin(username)` → challenge JSON
   - Browser: `navigator.credentials.get(challenge)`
   - User authenticates with device (2nd factor)
   - Browser: assertion → server
   - Server: `mfa_login_finish(assertion)` → creates session

## Legacy Functions

The `signin_begin()` and `signin_finish()` functions are deprecated in favor of the more specific `passkey_login_*` and `mfa_login_*` functions. Use the ceremony-specific functions for clarity and correctness.

## Building

Build all examples:
```bash
cargo build --examples
```

Build specific example:
```bash
cargo build --example credential_creation
cargo build --example passkey_login
cargo build --example multifactor_login
```

## Regenerating Go Bindings

The Go bindings are automatically regenerated during build via `build.rs`. To manually regenerate:

```bash
cargo clean
cargo build
```

This will:
1. Run rust2go-cli to generate `go/gen.go` from `src/lib.rs`
2. Compile the Go code
3. Build the Rust library

## Implementation Notes

- **User Storage:** In-memory for examples. Production should use a database.
- **Credential Storage:** In-memory for examples. Production needs encrypted storage.
- **Session Storage:** In-memory for examples. Production needs secure session store.
- **RPID/Origin:** Hardcoded to localhost. Production needs proper configuration.
- **Error Handling:** Basic for examples. Production needs comprehensive error handling.

## Next Steps

To use in production:

1. Replace in-memory storage with persistent database
2. Implement proper session management (Redis, database, etc.)
3. Configure RPID and RPOrigins for your domain
4. Add HTTPS/TLS support
5. Implement proper user authentication for 1st factor
6. Add audit logging
7. Implement rate limiting and anti-abuse measures
8. Test with real authenticators and browsers
9. Review security requirements for your use case

## License

This code is part of the Dure project and is dual-licensed under MIT/Apache-2.0.
