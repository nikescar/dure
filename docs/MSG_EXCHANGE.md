# Message Exchange Cryptography

## Question 1: How does A send a message that only B can read?

**Scenario:** A and B have their own private/public key pairs. A knows their private/public keys and B's public key. B knows their private/public keys and A's public key.

**Answer:** To send a message that only B can read, A encrypts the message using B's public key.

### Basic Process

1. **A encrypts:** `Message + B's public key → Encrypted message`
2. **B decrypts:** `Encrypted message + B's private key → Original message`

Only B (who holds B's private key) can decrypt this message.

### With Authentication (Recommended)

For B to also verify the message came from A:

1. **A signs and encrypts:**
   - **Sign:** `Hash(message) + A's private key → Signature`
   - **Encrypt:** `(Message + Signature) + B's public key → Encrypted bundle`

2. **B decrypts and verifies:**
   - **Decrypt:** `Encrypted bundle + B's private key → Message + Signature`
   - **Verify:** `Signature + A's public key → Confirms it's from A`

### Implementation in Rust (for Dure)

Common crates for this:

```rust
// For asymmetric encryption
use rsa::{RsaPublicKey, Pkcs1v15Encrypt};

// A encrypts for B
let encrypted = b_public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &message)?;

// B decrypts
let decrypted = b_private_key.decrypt(Pkcs1v15Encrypt, &encrypted)?;
```

**Modern alternatives:**
- **age** - Simple, modern encryption
- **ed25519-dalek** - For signing (X25519 variant for encryption)
- **orion** - High-level crypto primitives

**Key principle:** Encrypt with recipient's public key; only they can decrypt with their private key.

---

## Question 2: Can you explain more about modern alternatives?

## Modern Encryption Alternatives in Rust

### 1. age (Recommended for Simplicity)

**Best for:** File encryption, simple message encryption

```rust
use age::{x25519, Encryptor, Decryptor};

// A encrypts a message for B
let b_public_key = "age1ql3z7hjy54pw3hyww5ayyfg7zqgvc7w3j2elw8zmrj2kg5sfn9aqmcac8p";
let encrypted = {
    let encryptor = Encryptor::with_recipients(vec![
        Box::new(b_public_key.parse::<x25519::Recipient>()?)
    ])?;

    let mut output = vec![];
    let mut writer = encryptor.wrap_output(&mut output)?;
    writer.write_all(message.as_bytes())?;
    writer.finish()?;
    output
};

// B decrypts with private key
let b_identity = age::x25519::Identity::from_str(b_private_key_str)?;
let decrypted = {
    let decryptor = Decryptor::new(&encrypted[..])?;
    let mut decrypted = vec![];
    let mut reader = decryptor.decrypt(iter::once(&b_identity as &dyn age::Identity))?;
    reader.read_to_end(&mut decrypted)?;
    decrypted
};
```

**Pros:**
- Modern, audited, simple API
- Small keys (X25519: 32 bytes vs RSA: 256+ bytes)
- Fast
- Multiple recipients supported
- Format designed for files/messages

**Cons:**
- No built-in signing (need separate solution)

---

### 2. X25519 + ChaCha20-Poly1305 (Industry Standard)

**Best for:** When you need both encryption AND signing

```rust
use x25519_dalek::{EphemeralSecret, PublicKey};
use chacha20poly1305::{ChaCha20Poly1305, KeyInit};
use ed25519_dalek::{SigningKey, VerifyingKey, Signer, Verifier};

// === ENCRYPTION ===

// A encrypts for B using ECDH (Elliptic Curve Diffie-Hellman)
let a_ephemeral_secret = EphemeralSecret::random_from_rng(&mut rng);
let a_ephemeral_public = PublicKey::from(&a_ephemeral_secret);

// Compute shared secret with B's public key
let shared_secret = a_ephemeral_secret.diffie_hellman(&b_public_key);

// Use shared secret for symmetric encryption
let cipher = ChaCha20Poly1305::new(shared_secret.as_bytes().into());
let nonce = ChaCha20Poly1305::generate_nonce(&mut rng);
let ciphertext = cipher.encrypt(&nonce, message.as_bytes())?;

// Send: (a_ephemeral_public, nonce, ciphertext)

// B decrypts
let shared_secret = b_private_key.diffie_hellman(&a_ephemeral_public);
let cipher = ChaCha20Poly1305::new(shared_secret.as_bytes().into());
let plaintext = cipher.decrypt(&nonce, &ciphertext[..])?;

// === SIGNING (separate keys) ===

// A signs message
let a_signing_key = SigningKey::from_bytes(&a_private_signing_key);
let signature = a_signing_key.sign(message.as_bytes());

// B verifies
let a_verifying_key = VerifyingKey::from_bytes(&a_public_signing_key)?;
a_verifying_key.verify(message.as_bytes(), &signature)?;
```

**Pros:**
- State-of-the-art (used in Signal, WireGuard, TLS 1.3)
- Very fast
- Small keys (32 bytes)
- Constant-time operations (side-channel resistant)
- Separate signing/encryption keys (best practice)

**Cons:**
- More complex (need to manage ephemeral keys, nonces)
- Need to combine multiple crates

---

### 3. orion (High-Level, Batteries Included)

**Best for:** When you want safety and simplicity without crypto expertise

```rust
use orion::aead;
use orion::kdf;

// Orion focuses on authenticated encryption with shared secrets
// For public key crypto, combine with X25519:

use x25519_dalek::{StaticSecret, PublicKey};

// A and B derive shared secret via ECDH
let a_secret = StaticSecret::random_from_rng(&mut rng);
let a_public = PublicKey::from(&a_secret);

let shared_secret = a_secret.diffie_hellman(&b_public_key);

// Use orion for authenticated encryption with the shared secret
let secret_key = aead::SecretKey::from_slice(shared_secret.as_bytes())?;
let ciphertext = aead::seal(&secret_key, message.as_bytes())?;

// B decrypts
let shared_secret = b_secret.diffie_hellman(&a_public);
let secret_key = aead::SecretKey::from_slice(shared_secret.as_bytes())?;
let plaintext = aead::open(&secret_key, &ciphertext)?;
```

**Pros:**
- Misuse-resistant API (hard to use wrong)
- Good defaults
- Pure Rust
- Focused on correctness

**Cons:**
- Doesn't handle public key operations directly
- Still need X25519 or similar for key exchange

---

### 4. Comparison for Dure

| Feature         | age           | X25519+ChaCha20 | orion         | RSA         |
|-----------------|---------------|-----------------|---------------|-------------|
| Ease of use     | ⭐⭐⭐⭐⭐    | ⭐⭐⭐          | ⭐⭐⭐⭐      | ⭐⭐        |
| Speed           | ⭐⭐⭐⭐⭐    | ⭐⭐⭐⭐⭐      | ⭐⭐⭐⭐⭐    | ⭐⭐        |
| Key size        | 32 bytes      | 32 bytes        | 32 bytes      | 256+ bytes  |
| WASM support    | ✅            | ✅              | ✅            | ⚠️ (slower) |
| Mobile perf     | Excellent     | Excellent       | Excellent     | Poor        |
| Multi-recipient | ✅ Built-in   | Manual          | Manual        | Manual      |
| Signing         | ❌ (separate) | ✅ Ed25519      | ❌ (separate) | ✅          |

---

### 5. Recommendation for Dure

Given your requirements (WASM, mobile, desktop):

#### Option A: age + Ed25519 (Simplest)

```toml
[dependencies]
age = "0.10"
ed25519-dalek = "2.1"
```

- **Encryption:** Use `age`
- **Signatures:** Use `ed25519-dalek`
- **Why:** Simple APIs, great for user-to-user messages, excellent cross-platform

#### Option B: RustCrypto Ecosystem (Most Flexible)

```toml
[dependencies]
x25519-dalek = "2.0"
chacha20poly1305 = "0.10"
ed25519-dalek = "2.1"
sha2 = "0.10"
```

- Full control over primitives
- Used by major projects (libp2p, Signal protocol implementations)
- Best performance

---

## Complete Example: A→B Secure Message

```rust
// Dependencies
use age::x25519;
use ed25519_dalek::{SigningKey, VerifyingKey, Signer, Signature, Verifier};
use rand::rngs::OsRng;

// A sends authenticated, encrypted message to B
fn a_send_to_b(
    message: &[u8],
    a_signing_key: &SigningKey,
    b_encryption_pubkey: &x25519::Recipient,
) -> Result<Vec<u8>> {
    // 1. Sign message
    let signature = a_signing_key.sign(message);

    // 2. Bundle message + signature
    let bundle = [message, signature.to_bytes().as_slice()].concat();

    // 3. Encrypt bundle for B
    let encryptor = age::Encryptor::with_recipients(vec![
        Box::new(b_encryption_pubkey.clone())
    ])?;

    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted)?;
    writer.write_all(&bundle)?;
    writer.finish()?;

    Ok(encrypted)
}

// B receives and verifies
fn b_receive_from_a(
    encrypted: &[u8],
    b_decryption_key: &x25519::Identity,
    a_verify_pubkey: &VerifyingKey,
) -> Result<Vec<u8>> {
    // 1. Decrypt
    let decryptor = age::Decryptor::new(encrypted)?;
    let mut bundle = vec![];
    let mut reader = decryptor.decrypt(iter::once(b_decryption_key as &dyn age::Identity))?;
    reader.read_to_end(&mut bundle)?;

    // 2. Split message and signature
    let sig_bytes = &bundle[bundle.len()-64..];
    let message = &bundle[..bundle.len()-64];
    let signature = Signature::from_bytes(sig_bytes.try_into()?);

    // 3. Verify signature
    a_verify_pubkey.verify(message, &signature)?;

    Ok(message.to_vec())
}
```

---

## Key Takeaways

1. **For Dure's use case:** Use `age` for encryption (simple, modern, multi-platform)
2. **For signatures:** Use `ed25519-dalek` (fast, small, standard)
3. **Why not RSA:** Slower, larger keys, outdated, poor mobile/WASM performance
4. **Pattern:** Each user has 2 key pairs:
   - **X25519 (encryption)** - for receiving encrypted messages
   - **Ed25519 (signing)** - for signing messages they send

This matches modern messaging apps (Signal, WhatsApp) and works great across all your platforms (desktop, Android, WASM).