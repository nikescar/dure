//! WebAuthn bridge using rust2go to call Go's go-webauthn library
//!
//! This module provides a Rust-Go bridge for WebAuthn operations.
//! The Go side uses github.com/go-webauthn/webauthn library.

// Include the generated rust2go bindings
pub mod binding {
    #![allow(warnings)]
    rust2go::r2g_include_binding!();
}

/// Request to begin passkey registration
#[derive(Debug, Clone, rust2go::R2G)]
pub struct SignupBeginRequest {
    /// Username (may be empty for usernameless scenario)
    pub username: String,
    /// Display name shown to user
    pub display_name: String,
    /// Authentication scenario: "mfa", "passwordless", or "usernameless"
    /// - "mfa": Multi-Factor Auth (userVerification: discouraged)
    /// - "passwordless": Passwordless with username (userVerification: required)
    /// - "usernameless": Discoverable credentials (residentKey: required)
    pub scenario: String,
}

/// Response from beginning passkey registration
#[derive(Debug, Clone, rust2go::R2G)]
pub struct SignupBeginResponse {
    /// Session ID to use for finish request
    pub session_id: String,
    /// JSON-serialized CreationChallengeResponse (WebAuthn standard format)
    /// Client should parse this and pass to navigator.credentials.create()
    pub challenge_json: String,
    /// Whether the request was successful
    pub success: bool,
    /// Error message if not successful
    pub error: String,
}

/// Request to finish passkey registration
#[derive(Debug, Clone, rust2go::R2G)]
pub struct SignupFinishRequest {
    /// Session ID from begin response
    pub session_id: String,
    /// JSON-serialized RegisterPublicKeyCredential from navigator.credentials.create()
    pub credential_json: String,
}

/// Response from finishing passkey registration
#[derive(Debug, Clone, rust2go::R2G)]
pub struct SignupFinishResponse {
    /// Whether registration was successful
    pub success: bool,
    /// User ID if successful
    pub user_id: String,
    /// Error message if not successful
    pub error: String,
}

/// Request to begin passkey authentication
#[derive(Debug, Clone, rust2go::R2G)]
pub struct SigninBeginRequest {
    /// Username (empty for usernameless scenario)
    pub username: String,
    /// Authentication scenario: "mfa", "passwordless", or "usernameless"
    pub scenario: String,
}

/// Response from beginning passkey authentication
#[derive(Debug, Clone, rust2go::R2G)]
pub struct SigninBeginResponse {
    /// Session ID to use for finish request
    pub session_id: String,
    /// JSON-serialized RequestChallengeResponse (WebAuthn standard format)
    /// Client should parse this and pass to navigator.credentials.get()
    pub challenge_json: String,
    /// Whether the request was successful
    pub success: bool,
    /// Error message if not successful
    pub error: String,
}

/// Request to finish passkey authentication
#[derive(Debug, Clone, rust2go::R2G)]
pub struct SigninFinishRequest {
    /// Session ID from begin response
    pub session_id: String,
    /// JSON-serialized PublicKeyCredential from navigator.credentials.get()
    pub credential_json: String,
}

/// Response from finishing passkey authentication
#[derive(Debug, Clone, rust2go::R2G)]
pub struct SigninFinishResponse {
    /// Whether authentication was successful
    pub success: bool,
    /// User ID if successful
    pub user_id: String,
    /// Session token for authenticated session
    pub session_token: String,
    /// Error message if not successful
    pub error: String,
}

// ============================================================================
// Passkey Login Ceremony (Discoverable Credentials)
// ============================================================================

/// Request to begin passkey login (discoverable credentials)
#[derive(Debug, Clone, rust2go::R2G)]
pub struct PasskeyLoginBeginRequest {
    /// Mediation mode: "silent", "optional", "conditional", or "required"
    pub mediation: String,
}

/// Response from beginning passkey login
#[derive(Debug, Clone, rust2go::R2G)]
pub struct PasskeyLoginBeginResponse {
    /// Session ID to use for finish request
    pub session_id: String,
    /// JSON-serialized CredentialAssertion (WebAuthn standard format)
    /// Client should parse this and pass to navigator.credentials.get()
    pub challenge_json: String,
    /// Whether the request was successful
    pub success: bool,
    /// Error message if not successful
    pub error: String,
}

/// Request to finish passkey login
#[derive(Debug, Clone, rust2go::R2G)]
pub struct PasskeyLoginFinishRequest {
    /// Session ID from begin response
    pub session_id: String,
    /// JSON-serialized PublicKeyCredential from navigator.credentials.get()
    pub credential_json: String,
}

/// Response from finishing passkey login
#[derive(Debug, Clone, rust2go::R2G)]
pub struct PasskeyLoginFinishResponse {
    /// Whether authentication was successful
    pub success: bool,
    /// User ID if successful
    pub user_id: String,
    /// Username if successful
    pub username: String,
    /// Session token for authenticated session
    pub session_token: String,
    /// Error message if not successful
    pub error: String,
}

// ============================================================================
// Multi-Factor Login Ceremony
// ============================================================================

/// Request to begin multi-factor login
#[derive(Debug, Clone, rust2go::R2G)]
pub struct MfaLoginBeginRequest {
    /// Username for the user performing MFA
    pub username: String,
    /// Mediation mode: "silent", "optional", "conditional", or "required"
    pub mediation: String,
}

/// Response from beginning multi-factor login
#[derive(Debug, Clone, rust2go::R2G)]
pub struct MfaLoginBeginResponse {
    /// Session ID to use for finish request
    pub session_id: String,
    /// JSON-serialized CredentialAssertion (WebAuthn standard format)
    /// Client should parse this and pass to navigator.credentials.get()
    pub challenge_json: String,
    /// Whether the request was successful
    pub success: bool,
    /// Error message if not successful
    pub error: String,
}

/// Request to finish multi-factor login
#[derive(Debug, Clone, rust2go::R2G)]
pub struct MfaLoginFinishRequest {
    /// Session ID from begin response
    pub session_id: String,
    /// JSON-serialized PublicKeyCredential from navigator.credentials.get()
    pub credential_json: String,
}

/// Response from finishing multi-factor login
#[derive(Debug, Clone, rust2go::R2G)]
pub struct MfaLoginFinishResponse {
    /// Whether authentication was successful
    pub success: bool,
    /// User ID if successful
    pub user_id: String,
    /// Session token for authenticated session
    pub session_token: String,
    /// Error message if not successful
    pub error: String,
}

/// WebAuthn bridge trait
///
/// Implemented in Go using github.com/go-webauthn/webauthn
///
/// Implements three WebAuthn ceremonies as defined in go-webauthn/webauthn/doc.go:
/// 1. Credential Creation Ceremony (signup_begin/finish)
/// 2. Passkey Login Ceremony (passkey_login_begin/finish) - Discoverable credentials
/// 3. Multi-Factor Login Ceremony (mfa_login_begin/finish) - User must be already authenticated
#[rust2go::r2g(binding = binding)]
pub trait WebAuthnBridge {
    // ========================================================================
    // Credential Creation Ceremony
    // ========================================================================

    /// Begin passkey registration (signup)
    fn signup_begin(
        req: &SignupBeginRequest,
    ) -> impl std::future::Future<Output = SignupBeginResponse>;

    /// Finish passkey registration (signup)
    fn signup_finish(
        req: &SignupFinishRequest,
    ) -> impl std::future::Future<Output = SignupFinishResponse>;

    // ========================================================================
    // Legacy signin methods (DEPRECATED - use passkey_login or mfa_login instead)
    // ========================================================================

    /// Begin passkey authentication (signin) - DEPRECATED
    /// Use passkey_login_begin for discoverable credentials or mfa_login_begin for multi-factor
    fn signin_begin(
        req: &SigninBeginRequest,
    ) -> impl std::future::Future<Output = SigninBeginResponse>;

    /// Finish passkey authentication (signin) - DEPRECATED
    /// Use passkey_login_finish for discoverable credentials or mfa_login_finish for multi-factor
    fn signin_finish(
        req: &SigninFinishRequest,
    ) -> impl std::future::Future<Output = SigninFinishResponse>;

    // ========================================================================
    // Passkey Login Ceremony (Discoverable Credentials)
    // ========================================================================

    /// Begin passkey login ceremony (discoverable credentials)
    /// Uses BeginDiscoverableMediatedLogin from go-webauthn
    fn passkey_login_begin(
        req: &PasskeyLoginBeginRequest,
    ) -> impl std::future::Future<Output = PasskeyLoginBeginResponse>;

    /// Finish passkey login ceremony
    /// Uses FinishPasskeyLogin from go-webauthn
    fn passkey_login_finish(
        req: &PasskeyLoginFinishRequest,
    ) -> impl std::future::Future<Output = PasskeyLoginFinishResponse>;

    // ========================================================================
    // Multi-Factor Login Ceremony
    // ========================================================================

    /// Begin multi-factor login ceremony
    /// User must be already authenticated. Uses BeginMediatedLogin from go-webauthn
    fn mfa_login_begin(
        req: &MfaLoginBeginRequest,
    ) -> impl std::future::Future<Output = MfaLoginBeginResponse>;

    /// Finish multi-factor login ceremony
    /// Uses FinishLogin from go-webauthn
    fn mfa_login_finish(
        req: &MfaLoginFinishRequest,
    ) -> impl std::future::Future<Output = MfaLoginFinishResponse>;
}

// Re-export the generated implementation from binding module
// The rust2go macro generates {TraitName}Impl in the binding module
pub use binding::*;

// ============================================================================
// Safe wrapper functions to avoid unsafe in calling code
// ============================================================================

// Credential Creation Ceremony
/// Begin WebAuthn signup (safe wrapper)
pub async fn webauthn_signup_begin(req: &SignupBeginRequest) -> SignupBeginResponse {
    unsafe { WebAuthnBridgeImpl::signup_begin(req) }.await
}

/// Finish WebAuthn signup (safe wrapper)
pub async fn webauthn_signup_finish(req: &SignupFinishRequest) -> SignupFinishResponse {
    unsafe { WebAuthnBridgeImpl::signup_finish(req) }.await
}

// Legacy signin (DEPRECATED)
/// Begin WebAuthn signin (safe wrapper) - DEPRECATED
pub async fn webauthn_signin_begin(req: &SigninBeginRequest) -> SigninBeginResponse {
    unsafe { WebAuthnBridgeImpl::signin_begin(req) }.await
}

/// Finish WebAuthn signin (safe wrapper) - DEPRECATED
pub async fn webauthn_signin_finish(req: &SigninFinishRequest) -> SigninFinishResponse {
    unsafe { WebAuthnBridgeImpl::signin_finish(req) }.await
}

// Passkey Login Ceremony
/// Begin Passkey login (safe wrapper)
pub async fn webauthn_passkey_login_begin(
    req: &PasskeyLoginBeginRequest,
) -> PasskeyLoginBeginResponse {
    unsafe { WebAuthnBridgeImpl::passkey_login_begin(req) }.await
}

/// Finish Passkey login (safe wrapper)
pub async fn webauthn_passkey_login_finish(
    req: &PasskeyLoginFinishRequest,
) -> PasskeyLoginFinishResponse {
    unsafe { WebAuthnBridgeImpl::passkey_login_finish(req) }.await
}

// Multi-Factor Login Ceremony
/// Begin Multi-Factor login (safe wrapper)
pub async fn webauthn_mfa_login_begin(req: &MfaLoginBeginRequest) -> MfaLoginBeginResponse {
    unsafe { WebAuthnBridgeImpl::mfa_login_begin(req) }.await
}

/// Finish Multi-Factor login (safe wrapper)
pub async fn webauthn_mfa_login_finish(req: &MfaLoginFinishRequest) -> MfaLoginFinishResponse {
    unsafe { WebAuthnBridgeImpl::mfa_login_finish(req) }.await
}

// ============================================================================
// Cryptography Functions - ChaCha20-Poly1305, Ed25519, and ACME AutoCert
// ============================================================================

// ─── ChaCha20-Poly1305 Encryption ──────────────────────────────────────────

/// Request to encrypt data using ChaCha20-Poly1305
#[derive(Debug, Clone, rust2go::R2G)]
pub struct ChaCha20Poly1305EncryptRequest {
    /// 32-byte key
    pub key: Vec<u8>,
    /// 24-byte nonce for XChaCha20-Poly1305
    pub nonce: Vec<u8>,
    /// Plaintext data to encrypt
    pub plaintext: Vec<u8>,
    /// Additional authenticated data (optional)
    pub additional_data: Vec<u8>,
}

/// Response from ChaCha20-Poly1305 encryption
#[derive(Debug, Clone, rust2go::R2G)]
pub struct ChaCha20Poly1305EncryptResponse {
    /// Encrypted ciphertext (includes authentication tag)
    pub ciphertext: Vec<u8>,
    /// Whether encryption was successful
    pub success: bool,
    /// Error message if not successful
    pub error: String,
}

/// Request to decrypt data using ChaCha20-Poly1305
#[derive(Debug, Clone, rust2go::R2G)]
pub struct ChaCha20Poly1305DecryptRequest {
    /// 32-byte key
    pub key: Vec<u8>,
    /// 24-byte nonce for XChaCha20-Poly1305
    pub nonce: Vec<u8>,
    /// Ciphertext to decrypt (includes authentication tag)
    pub ciphertext: Vec<u8>,
    /// Additional authenticated data (optional)
    pub additional_data: Vec<u8>,
}

/// Response from ChaCha20-Poly1305 decryption
#[derive(Debug, Clone, rust2go::R2G)]
pub struct ChaCha20Poly1305DecryptResponse {
    /// Decrypted plaintext
    pub plaintext: Vec<u8>,
    /// Whether decryption was successful
    pub success: bool,
    /// Error message if not successful
    pub error: String,
}

// ─── Ed25519 Digital Signatures ────────────────────────────────────────────

/// Request to generate Ed25519 key pair
#[derive(Debug, Clone, rust2go::R2G)]
pub struct Ed25519GenerateKeyRequest {}

/// Response from Ed25519 key generation
#[derive(Debug, Clone, rust2go::R2G)]
pub struct Ed25519GenerateKeyResponse {
    /// Public key (32 bytes)
    pub public_key: Vec<u8>,
    /// Private key (64 bytes)
    pub private_key: Vec<u8>,
    /// Whether generation was successful
    pub success: bool,
    /// Error message if not successful
    pub error: String,
}

/// Request to sign a message with Ed25519
#[derive(Debug, Clone, rust2go::R2G)]
pub struct Ed25519SignRequest {
    /// Private key (64 bytes)
    pub private_key: Vec<u8>,
    /// Message to sign
    pub message: Vec<u8>,
}

/// Response from Ed25519 signing
#[derive(Debug, Clone, rust2go::R2G)]
pub struct Ed25519SignResponse {
    /// Signature (64 bytes)
    pub signature: Vec<u8>,
    /// Whether signing was successful
    pub success: bool,
    /// Error message if not successful
    pub error: String,
}

/// Request to verify an Ed25519 signature
#[derive(Debug, Clone, rust2go::R2G)]
pub struct Ed25519VerifyRequest {
    /// Public key (32 bytes)
    pub public_key: Vec<u8>,
    /// Message that was signed
    pub message: Vec<u8>,
    /// Signature to verify (64 bytes)
    pub signature: Vec<u8>,
}

/// Response from Ed25519 verification
#[derive(Debug, Clone, rust2go::R2G)]
pub struct Ed25519VerifyResponse {
    /// Whether the signature is valid
    pub valid: bool,
    /// Whether verification was successful (not the same as valid)
    pub success: bool,
    /// Error message if verification failed to run
    pub error: String,
}

/// Crypto bridge trait - exposes ChaCha20-Poly1305 and Ed25519 functions
#[rust2go::r2g(binding = binding)]
pub trait CryptoBridge {
    // ChaCha20-Poly1305
    fn chacha20poly1305_encrypt(
        req: &ChaCha20Poly1305EncryptRequest,
    ) -> impl std::future::Future<Output = ChaCha20Poly1305EncryptResponse>;
    fn chacha20poly1305_decrypt(
        req: &ChaCha20Poly1305DecryptRequest,
    ) -> impl std::future::Future<Output = ChaCha20Poly1305DecryptResponse>;

    // Ed25519
    fn ed25519_generate_key(
        req: &Ed25519GenerateKeyRequest,
    ) -> impl std::future::Future<Output = Ed25519GenerateKeyResponse>;
    fn ed25519_sign(
        req: &Ed25519SignRequest,
    ) -> impl std::future::Future<Output = Ed25519SignResponse>;
    fn ed25519_verify(
        req: &Ed25519VerifyRequest,
    ) -> impl std::future::Future<Output = Ed25519VerifyResponse>;
}

// ============================================================================
// Safe wrapper functions for crypto operations
// ============================================================================

// ChaCha20-Poly1305
/// Encrypt data using ChaCha20-Poly1305 (safe wrapper)
pub async fn crypto_chacha20poly1305_encrypt(
    req: &ChaCha20Poly1305EncryptRequest,
) -> ChaCha20Poly1305EncryptResponse {
    unsafe { CryptoBridgeImpl::chacha20poly1305_encrypt(req) }.await
}

/// Decrypt data using ChaCha20-Poly1305 (safe wrapper)
pub async fn crypto_chacha20poly1305_decrypt(
    req: &ChaCha20Poly1305DecryptRequest,
) -> ChaCha20Poly1305DecryptResponse {
    unsafe { CryptoBridgeImpl::chacha20poly1305_decrypt(req) }.await
}

// Ed25519
/// Generate Ed25519 key pair (safe wrapper)
pub async fn crypto_ed25519_generate_key(
    req: &Ed25519GenerateKeyRequest,
) -> Ed25519GenerateKeyResponse {
    unsafe { CryptoBridgeImpl::ed25519_generate_key(req) }.await
}

/// Sign message with Ed25519 (safe wrapper)
pub async fn crypto_ed25519_sign(req: &Ed25519SignRequest) -> Ed25519SignResponse {
    unsafe { CryptoBridgeImpl::ed25519_sign(req) }.await
}

/// Verify Ed25519 signature (safe wrapper)
pub async fn crypto_ed25519_verify(req: &Ed25519VerifyRequest) -> Ed25519VerifyResponse {
    unsafe { CryptoBridgeImpl::ed25519_verify(req) }.await
}
