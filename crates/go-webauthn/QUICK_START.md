# Quick Start Guide

## Overview

The go-webauthn crate now supports all three WebAuthn ceremonies:

1. **Credential Creation** - Register new authenticators
2. **Passkey Login** - Passwordless authentication (discoverable credentials)
3. **Multi-Factor Login** - Second-factor authentication

## Quick Reference

### Credential Creation

```rust
use go_webauthn::*;

// Begin registration
let req = SignupBeginRequest {
    username: "alice@example.com".to_string(),
    display_name: "Alice Anderson".to_string(),
    scenario: "passwordless".to_string(), // "mfa", "passwordless", or "usernameless"
};
let response = webauthn_signup_begin(&req).await;

// Send response.challenge_json to browser
// Browser calls navigator.credentials.create(JSON.parse(challenge_json))
// Browser returns credential

// Finish registration
let finish_req = SignupFinishRequest {
    session_id: response.session_id,
    credential_json: credential_from_browser,
};
let result = webauthn_signup_finish(&finish_req).await;
```

### Passkey Login

```rust
use go_webauthn::*;

// Begin passkey login (no username needed!)
let req = PasskeyLoginBeginRequest {
    mediation: "optional".to_string(),
};
let response = webauthn_passkey_login_begin(&req).await;

// Send response.challenge_json to browser
// Browser calls navigator.credentials.get(JSON.parse(challenge_json))
// Authenticator shows list of stored credentials
// User selects and authenticates
// Browser returns assertion

// Finish passkey login
let finish_req = PasskeyLoginFinishRequest {
    session_id: response.session_id,
    credential_json: assertion_from_browser,
};
let result = webauthn_passkey_login_finish(&finish_req).await;
// result.username contains discovered username
```

### Multi-Factor Login

```rust
use go_webauthn::*;

// User already authenticated via password (1st factor)
// Now require authenticator as 2nd factor

// Begin MFA login
let req = MfaLoginBeginRequest {
    username: "alice@example.com".to_string(), // Known from 1st factor
    mediation: "optional".to_string(),
};
let response = webauthn_mfa_login_begin(&req).await;

// Send response.challenge_json to browser
// Browser calls navigator.credentials.get(JSON.parse(challenge_json))
// User authenticates with device
// Browser returns assertion

// Finish MFA login
let finish_req = MfaLoginFinishRequest {
    session_id: response.session_id,
    credential_json: assertion_from_browser,
};
let result = webauthn_mfa_login_finish(&finish_req).await;
// Upgrade session to fully authenticated
```

## Scenarios

### Registration Scenarios

| Scenario | Resident Key | User Verification | Use Case |
|----------|--------------|-------------------|----------|
| `"mfa"` | Not required | Discouraged | 2FA after password |
| `"passwordless"` | Preferred | Required | Login with username + passkey |
| `"usernameless"` | Required | Required | Login without username |

### Mediation Modes

| Mode | Description |
|------|-------------|
| `"default"` | Browser decides |
| `"optional"` | UI may be shown (recommended) |
| `"required"` | Always show UI |
| `"conditional"` | Autofill UI |
| `"silent"` | No UI if possible |

## Running Examples

```bash
# Credential Creation
cargo run --example credential_creation

# Passkey Login
cargo run --example passkey_login

# Multi-Factor Login
cargo run --example multifactor_login
```

## When to Use Which Ceremony

### Use Credential Creation when:
- User is registering for the first time
- User wants to add a new authenticator
- Upgrading from password to passkey

### Use Passkey Login when:
- You want passwordless authentication
- Best user experience is priority
- Users have devices with resident key support

### Use Multi-Factor Login when:
- You already have password/email authentication
- You want 2FA for extra security
- Gradual migration from passwords
- Step-up authentication for sensitive operations

## Browser-Side Code

### Creating a Credential

```javascript
// Parse challenge from server
const challenge = JSON.parse(challengeJson);

// Create credential
const credential = await navigator.credentials.create({
  publicKey: challenge.publicKey
});

// Send back to server
const credentialJson = JSON.stringify(credential);
```

### Getting an Assertion (Login)

```javascript
// Parse challenge from server
const challenge = JSON.parse(challengeJson);

// Get assertion
const assertion = await navigator.credentials.get({
  publicKey: challenge.publicKey
});

// Send back to server
const assertionJson = JSON.stringify(assertion);
```

## Full Documentation

See [`examples/README.md`](./examples/README.md) for comprehensive documentation.

## Reference

- **Implementation Details:** [`IMPLEMENTATION_SUMMARY.md`](./IMPLEMENTATION_SUMMARY.md)
- **Go WebAuthn:** https://github.com/go-webauthn/webauthn
- **WebAuthn Spec:** https://www.w3.org/TR/webauthn-2/
