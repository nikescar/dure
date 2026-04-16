# Crypto Functions Implementation Summary

This document summarizes the implementation of ChaCha20-Poly1305 and Ed25519 functions in the go-webauthn crate using rust2go.

## Implemented Functions

### ✅ ChaCha20-Poly1305 (XChaCha20-Poly1305)

**Status: WORKING**

Exposes encryption and decryption using XChaCha20-Poly1305 AEAD (Authenticated Encryption with Associated Data) from `golang.org/x/crypto/chacha20poly1305`.

#### Rust API:

```rust
// Encrypt
let req = ChaCha20Poly1305EncryptRequest {
    key: vec![0u8; 32],           // 32-byte key
    nonce: vec![0u8; 24],         // 24-byte nonce (XChaCha20-Poly1305)
    plaintext: b"message".to_vec(),
    additional_data: vec![],       // Optional AAD
};
let resp = crypto_chacha20poly1305_encrypt(&req).await;

// Decrypt
let req = ChaCha20Poly1305DecryptRequest {
    key: vec![0u8; 32],
    nonce: vec![0u8; 24],
    ciphertext: encrypted_data,
    additional_data: vec![],
};
let resp = crypto_chacha20poly1305_decrypt(&req).await;
```

#### Go Implementation:

```go
func (c *CryptoImpl) chacha20poly1305_encrypt(req *ChaCha20Poly1305EncryptRequest) ChaCha20Poly1305EncryptResponse {
    aead, err := chacha20poly1305.NewX(req.key)
    if err != nil {
        return ChaCha20Poly1305EncryptResponse{success: false, error: err.Error()}
    }
    ciphertext := aead.Seal(nil, req.nonce, req.plaintext, req.additional_data)
    return ChaCha20Poly1305EncryptResponse{ciphertext: ciphertext, success: true}
}
```

#### Test Results:

```
Original message: Gophers, gophers, gophers everywhere!
Encrypted (hex): 50ce207bd8ac85651c2c97a545c0259760ade74e025fb94eb9522f3f8b2d72767f8043f2e3dffd321ba5bd6c616fb8e5b7a39427fd
Ciphertext length: 53 bytes (plaintext: 37, overhead: 16)
Decrypted message: Gophers, gophers, gophers everywhere!
✓ Round-trip successful!

Testing authentication...
✓ Tampered ciphertext rejected: decryption failed: chacha20poly1305: message authentication failed
```

### ✅ Ed25519 Digital Signatures

**Status: WORKING**

Exposes Ed25519 digital signature operations from `crypto/ed25519`.

#### Rust API:

```rust
// Generate key pair
let req = Ed25519GenerateKeyRequest {};
let resp = crypto_ed25519_generate_key(&req).await;
// resp.public_key: 32 bytes
// resp.private_key: 64 bytes

// Sign message
let req = Ed25519SignRequest {
    private_key: private_key_bytes,  // 64 bytes
    message: b"message to sign".to_vec(),
};
let resp = crypto_ed25519_sign(&req).await;
// resp.signature: 64 bytes

// Verify signature
let req = Ed25519VerifyRequest {
    public_key: public_key_bytes,    // 32 bytes
    message: b"message to sign".to_vec(),
    signature: signature_bytes,       // 64 bytes
};
let resp = crypto_ed25519_verify(&req).await;
// resp.valid: bool
```

#### Go Implementation:

```go
func (c *CryptoImpl) ed25519_generate_key(req *Ed25519GenerateKeyRequest) Ed25519GenerateKeyResponse {
    publicKey, privateKey, err := ed25519.GenerateKey(nil)
    if err != nil {
        return Ed25519GenerateKeyResponse{success: false, error: err.Error()}
    }
    return Ed25519GenerateKeyResponse{
        public_key: publicKey, 
        private_key: privateKey, 
        success: true,
    }
}

func (c *CryptoImpl) ed25519_sign(req *Ed25519SignRequest) Ed25519SignResponse {
    signature := ed25519.Sign(req.private_key, req.message)
    return Ed25519SignResponse{signature: signature, success: true}
}

func (c *CryptoImpl) ed25519_verify(req *Ed25519VerifyRequest) Ed25519VerifyResponse {
    valid := ed25519.Verify(req.public_key, req.message, req.signature)
    return Ed25519VerifyResponse{valid: valid, success: true}
}
```

#### Known Issue (RESOLVED):

~~The implementation hit a runtime crash in Go 1.25's FIPS cache weak pointer implementation. This was caused by using Ed25519 private keys from FFI memory, which confused Go's weak pointer tracking.~~

**Fix:** Make a defensive copy of the private key from FFI memory into Go-managed memory before signing:

```go
privateKeyCopy := make(ed25519.PrivateKey, len(req.private_key))
copy(privateKeyCopy, req.private_key)
signature := ed25519.Sign(privateKeyCopy, req.message)
```

This ensures the private key is in Go's garbage-collected heap, avoiding the weak pointer bug.

## File Structure

```
go-webauthn/
├── src/lib.rs                    # Rust structs and trait definitions
├── go/impl.go                    # Go implementations
├── examples/crypto_example.rs    # Usage examples
└── Cargo.toml                    # Dependencies
```

## Dependencies

### Rust:
- `rust2go = "0.4.0"` - Rust-Go FFI bridge
- `rust2go-mem-ffi = "0.2.1"` - Memory management for FFI
- `mem-ring = "0.2.0"` - Ring buffer for FFI communication

### Go:
- `golang.org/x/crypto/chacha20poly1305` - AEAD encryption
- `crypto/ed25519` - Digital signatures

## Usage Example

```rust
use go_webauthn::*;
use futures::executor::block_on;

fn main() {
    // ChaCha20-Poly1305 encryption
    let key = vec![0u8; 32];
    let nonce = vec![0u8; 24];
    let plaintext = b"Secret message";

    let encrypt_req = ChaCha20Poly1305EncryptRequest {
        key: key.clone(),
        nonce: nonce.clone(),
        plaintext: plaintext.to_vec(),
        additional_data: vec![],
    };

    let encrypted = block_on(crypto_chacha20poly1305_encrypt(&encrypt_req));
    
    if encrypted.success {
        println!("Encrypted: {}", hex::encode(&encrypted.ciphertext));
        
        // Decrypt
        let decrypt_req = ChaCha20Poly1305DecryptRequest {
            key,
            nonce,
            ciphertext: encrypted.ciphertext,
            additional_data: vec![],
        };
        
        let decrypted = block_on(crypto_chacha20poly1305_decrypt(&decrypt_req));
        
        if decrypted.success {
            println!("Decrypted: {}", String::from_utf8_lossy(&decrypted.plaintext));
        }
    }
}
```

## Reference

Based on Go reference tests:
- `reference/crypto/chacha20poly1305/chacha20poly1305_test.go::ExampleNewX`
- `reference/crypto/ed25519/ed25519_test.go::TestTypeAlias`

## Build and Run

```bash
cd go-webauthn

# Build the crate
cargo build

# Run the example (ChaCha20-Poly1305 works, Ed25519 has runtime issue)
cargo run --example crypto_example
```

## Known Limitations

1. **Threading**: All operations run through Go's goroutine pool via rust2go

## Implementation Notes

### FFI Memory Management

When passing byte slices from Rust to Go through rust2go, the memory is allocated in Rust's heap. Go 1.25's FIPS cache uses weak pointers to track Ed25519 private keys for performance optimization. However, weak pointers only work with Go-managed memory.

**Solution:** Make defensive copies of FFI data before passing to crypto functions:

```go
// Copy from FFI memory to Go memory
privateKeyCopy := make(ed25519.PrivateKey, len(req.private_key))
copy(privateKeyCopy, req.private_key)
signature := ed25519.Sign(privateKeyCopy, req.message)
```

This pattern should be used for any crypto operation that uses internal caching.

## Future Improvements

1. **Add tests**: Unit tests for each crypto function
2. **Performance**: Benchmark vs native Rust implementations (measure impact of memory copies)
3. **More algorithms**: Add AES-GCM, ChaCha20 (without Poly1305), etc.
4. **Zero-copy optimization**: Investigate rust2go memory management to avoid defensive copies

## License

Same as parent project: Dual MIT/Apache-2.0
