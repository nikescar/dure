//! Example demonstrating crypto functions: ChaCha20-Poly1305 and Ed25519
//!
//! This example shows how to use the crypto bridge to:
//! 1. Encrypt and decrypt data with ChaCha20-Poly1305
//! 2. Generate Ed25519 keys, sign and verify messages

use futures::executor::block_on;
use go_webauthn::*;

fn main() {
    println!("=== Crypto Bridge Example ===\n");

    // ========================================================================
    // Example 1: ChaCha20-Poly1305 Encryption/Decryption
    // ========================================================================
    println!("--- ChaCha20-Poly1305 Example ---");

    // Generate a random 32-byte key (in production, use a proper key derivation function)
    let key: Vec<u8> = (0..32).map(|i| (i * 7) as u8).collect();

    // Generate a random 24-byte nonce for XChaCha20-Poly1305
    let nonce: Vec<u8> = (0..24).map(|i| (i * 11) as u8).collect();

    let plaintext = b"Gophers, gophers, gophers everywhere!";
    let additional_data = b"metadata";

    println!("Original message: {}", String::from_utf8_lossy(plaintext));

    // Encrypt
    let encrypt_req = ChaCha20Poly1305EncryptRequest {
        key: key.clone(),
        nonce: nonce.clone(),
        plaintext: plaintext.to_vec(),
        additional_data: additional_data.to_vec(),
    };

    let encrypt_resp = block_on(crypto_chacha20poly1305_encrypt(&encrypt_req));

    if !encrypt_resp.success {
        eprintln!("Encryption failed: {}", encrypt_resp.error);
        return;
    }

    println!("Encrypted (hex): {}", hex::encode(&encrypt_resp.ciphertext));
    println!(
        "Ciphertext length: {} bytes (plaintext: {}, overhead: {})",
        encrypt_resp.ciphertext.len(),
        plaintext.len(),
        encrypt_resp.ciphertext.len() - plaintext.len()
    );

    // Decrypt
    let decrypt_req = ChaCha20Poly1305DecryptRequest {
        key: key.clone(),
        nonce: nonce.clone(),
        ciphertext: encrypt_resp.ciphertext.clone(),
        additional_data: additional_data.to_vec(),
    };

    let decrypt_resp = block_on(crypto_chacha20poly1305_decrypt(&decrypt_req));

    if !decrypt_resp.success {
        eprintln!("Decryption failed: {}", decrypt_resp.error);
        return;
    }

    println!(
        "Decrypted message: {}",
        String::from_utf8_lossy(&decrypt_resp.plaintext)
    );

    // Verify round-trip
    assert_eq!(
        plaintext,
        decrypt_resp.plaintext.as_slice(),
        "Round-trip failed!"
    );
    println!("✓ Round-trip successful!\n");

    // Test authentication: tampering should fail
    println!("Testing authentication...");
    let mut tampered_ciphertext = encrypt_resp.ciphertext.clone();
    tampered_ciphertext[0] ^= 0xFF; // Flip some bits

    let tamper_req = ChaCha20Poly1305DecryptRequest {
        key: key.clone(),
        nonce: nonce.clone(),
        ciphertext: tampered_ciphertext,
        additional_data: additional_data.to_vec(),
    };

    let tamper_resp = block_on(crypto_chacha20poly1305_decrypt(&tamper_req));

    if tamper_resp.success {
        eprintln!("WARNING: Tampered ciphertext was accepted!");
    } else {
        println!("✓ Tampered ciphertext rejected: {}\n", tamper_resp.error);
    }

    // ========================================================================
    // Example 2: Ed25519 Digital Signatures
    // ========================================================================
    println!("--- Ed25519 Digital Signatures Example ---");

    // Generate key pair
    let gen_req = Ed25519GenerateKeyRequest {};
    let gen_resp = block_on(crypto_ed25519_generate_key(&gen_req));

    if !gen_resp.success {
        eprintln!("Key generation failed: {}", gen_resp.error);
        return;
    }

    println!("Generated Ed25519 key pair:");
    println!(
        "  Public key:  {} bytes (hex: {})",
        gen_resp.public_key.len(),
        hex::encode(&gen_resp.public_key[..8])
    );
    println!(
        "  Private key: {} bytes (hex: {})",
        gen_resp.private_key.len(),
        hex::encode(&gen_resp.private_key[..8])
    );

    // Sign a message
    let message = b"The quick brown fox jumps over the lazy dog";
    println!("\nMessage to sign: {}", String::from_utf8_lossy(message));

    let sign_req = Ed25519SignRequest {
        private_key: gen_resp.private_key.clone(),
        message: message.to_vec(),
    };

    let sign_resp = block_on(crypto_ed25519_sign(&sign_req));

    if !sign_resp.success {
        eprintln!("Signing failed: {}", sign_resp.error);
        return;
    }

    println!(
        "Signature: {} bytes (hex: {})",
        sign_resp.signature.len(),
        hex::encode(&sign_resp.signature)
    );

    // Verify the signature
    let verify_req = Ed25519VerifyRequest {
        public_key: gen_resp.public_key.clone(),
        message: message.to_vec(),
        signature: sign_resp.signature.clone(),
    };

    let verify_resp = block_on(crypto_ed25519_verify(&verify_req));

    if !verify_resp.success {
        eprintln!("Verification failed: {}", verify_resp.error);
        return;
    }

    if verify_resp.valid {
        println!("✓ Signature is VALID\n");
    } else {
        println!("✗ Signature is INVALID\n");
    }

    // Test invalid signature
    println!("Testing invalid signature...");
    let wrong_message = b"Different message";
    let verify_wrong_req = Ed25519VerifyRequest {
        public_key: gen_resp.public_key.clone(),
        message: wrong_message.to_vec(),
        signature: sign_resp.signature.clone(),
    };

    let verify_wrong_resp = block_on(crypto_ed25519_verify(&verify_wrong_req));

    if verify_wrong_resp.success && !verify_wrong_resp.valid {
        println!("✓ Invalid signature correctly rejected\n");
    } else {
        println!("✗ Invalid signature was accepted!\n");
    }

    println!("\n=== All Examples Complete ===");
}
