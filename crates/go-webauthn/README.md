# go-webauthn

Rust library providing WebAuthn functionality via Go's `go-webauthn` library using rust2go FFI bridge.

## Architecture

This crate provides a clean Rust API that calls into Go's excellent WebAuthn implementation:
- **Rust side**: Type-safe API with `SignupBeginRequest`, `SigninBeginRequest`, etc.
- **Go side**: `go-webauthn` library for WebAuthn protocol implementation
- **Bridge**: rust2go with shared memory communication

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
go-webauthn = { path = "../go-webauthn" }
```

Example:

```rust
use go_webauthn::*;

// Start registration
let request = SignupBeginRequest {
    username: "alice".to_string(),
    display_name: "Alice Smith".to_string(),
    scenario: "passwordless".to_string(),
};

let response = WebAuthnBridgeImpl::signup_begin(&request).await;

if response.success {
    // Send response.challenge_json to the browser
    println!("Challenge: {}", response.challenge_json);
} else {
    eprintln!("Error: {}", response.error);
}
```

## Scenarios

The library supports three WebAuthn scenarios:

1. **"mfa"** - Multi-Factor Authentication
   - Used alongside username/password
   - UserVerification: discouraged (faster UX)

2. **"passwordless"** - Passwordless with username
   - No password required
   - UserVerification: required (biometric/PIN)

3. **"usernameless"** - Discoverable credentials
   - No username needed
   - ResidentKey: required
   - UserVerification: required

## API

### Types

- `SignupBeginRequest` / `SignupBeginResponse` - Start registration
- `SignupFinishRequest` / `SignupFinishResponse` - Complete registration
- `SigninBeginRequest` / `SigninBeginResponse` - Start authentication
- `SigninFinishRequest` / `SigninFinishResponse` - Complete authentication

### Trait

```rust
pub trait WebAuthnBridge {
    fn signup_begin(req: &SignupBeginRequest) 
        -> impl Future<Output = SignupBeginResponse>;
    fn signup_finish(req: &SignupFinishRequest) 
        -> impl Future<Output = SignupFinishResponse>;
    fn signin_begin(req: &SigninBeginRequest) 
        -> impl Future<Output = SigninBeginResponse>;
    fn signin_finish(req: &SigninFinishRequest) 
        -> impl Future<Output = SigninFinishResponse>;
}
```

## Building

The build process:
1. Compiles Go code in `go/` directory
2. Generates C bindings via rust2go
3. Links the Go static library into your Rust binary

```bash
cargo build
```

## Platform Support

- ✅ Linux
- ✅ macOS
- ✅ Windows
- ❌ WASM (not supported - WebAuthn is browser-native there)
- ❌ Android (not yet tested)

## License

Dual-licensed under MIT OR Apache-2.0
