// Example demonstrating WebAuthn Credential Creation Ceremony
//
// This implements the first ceremony described in reference/webauthn/webauthn/doc.go:
// - BeginMediatedRegistration: Start credential creation
// - FinishRegistration: Complete credential creation
//
// Based on reference/webauthn/webauthn/registration_test.go

use futures::executor::LocalPool;
use go_webauthn::*;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     WebAuthn Credential Creation Ceremony Example           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("This example demonstrates the Credential Creation Ceremony");
    println!("as defined in go-webauthn/webauthn/doc.go:");
    println!();
    println!("  1. BeginMediatedRegistration() - Start registration");
    println!("  2. FinishRegistration() - Complete registration");
    println!();
    println!("Three scenarios are supported:");
    println!("  - MFA: Multi-factor authentication (non-discoverable)");
    println!("  - Passwordless: Discoverable credentials with username");
    println!("  - Usernameless: Fully discoverable credentials");
    println!();
    println!("════════════════════════════════════════════════════════════════");
    println!();

    // Create a LocalPool executor for running !Send futures
    let mut pool = LocalPool::new();

    // ========================================================================
    // Scenario 1: MFA (Multi-Factor Authentication)
    // ========================================================================
    println!("Scenario 1: MFA (Multi-Factor Authentication)");
    println!("──────────────────────────────────────────────");
    println!("User Verification: DISCOURAGED");
    println!("Resident Key: NOT REQUIRED");
    println!("Use Case: Second factor after password login");
    println!();

    let mfa_req = SignupBeginRequest {
        username: "alice@example.com".to_string(),
        display_name: "Alice Anderson".to_string(),
        scenario: "mfa".to_string(),
    };

    let response = pool.run_until(webauthn_signup_begin(&mfa_req));

    if response.success {
        println!("✓ Registration started successfully!");
        println!("  Session ID: {}", response.session_id);
        println!("  Challenge size: {} bytes", response.challenge_json.len());

        // In a real application, the challenge_json would be sent to the browser
        // and used with navigator.credentials.create()
        println!();
        println!("  Next step: Send challenge_json to browser");
        println!("  Browser calls: navigator.credentials.create(challenge)");
        println!("  Then call signup_finish() with the credential response");
    } else {
        println!("✗ Registration failed: {}", response.error);
    }
    println!();
    println!();

    // ========================================================================
    // Scenario 2: Passwordless (Discoverable Credentials with Username)
    // ========================================================================
    println!("Scenario 2: Passwordless");
    println!("─────────────────────────");
    println!("User Verification: REQUIRED");
    println!("Resident Key: PREFERRED");
    println!("Use Case: Login with username + passkey (no password)");
    println!();

    let passwordless_req = SignupBeginRequest {
        username: "bob@example.com".to_string(),
        display_name: "Bob Brown".to_string(),
        scenario: "passwordless".to_string(),
    };

    let response = pool.run_until(webauthn_signup_begin(&passwordless_req));

    if response.success {
        println!("✓ Registration started successfully!");
        println!("  Session ID: {}", response.session_id);
        println!("  Challenge size: {} bytes", response.challenge_json.len());

        // Show first few bytes of challenge for verification
        let challenge_preview = if response.challenge_json.len() > 100 {
            format!("{}...", &response.challenge_json[..100])
        } else {
            response.challenge_json.clone()
        };
        println!("  Challenge preview: {}", challenge_preview);
    } else {
        println!("✗ Registration failed: {}", response.error);
    }
    println!();
    println!();

    // ========================================================================
    // Scenario 3: Usernameless (Fully Discoverable Credentials)
    // ========================================================================
    println!("Scenario 3: Usernameless");
    println!("────────────────────────");
    println!("User Verification: REQUIRED");
    println!("Resident Key: REQUIRED");
    println!("Use Case: Login without entering username");
    println!();

    let usernameless_req = SignupBeginRequest {
        username: "".to_string(),
        display_name: "Charlie Chen".to_string(),
        scenario: "usernameless".to_string(),
    };

    let response = pool.run_until(webauthn_signup_begin(&usernameless_req));

    if response.success {
        println!("✓ Registration started successfully!");
        println!("  Session ID: {}", response.session_id);
        println!("  Challenge size: {} bytes", response.challenge_json.len());
        println!();
        println!("  For usernameless, the credential MUST be discoverable");
        println!("  (stored on the authenticator)");
    } else {
        println!("✗ Registration failed: {}", response.error);
    }
    println!();
    println!();

    // ========================================================================
    // Error Handling: Invalid Scenario
    // ========================================================================
    println!("Error Handling: Invalid Scenario");
    println!("─────────────────────────────────");

    let invalid_req = SignupBeginRequest {
        username: "test@example.com".to_string(),
        display_name: "Test User".to_string(),
        scenario: "invalid_scenario".to_string(),
    };

    let response = pool.run_until(webauthn_signup_begin(&invalid_req));

    if response.success {
        println!("✗ Should have failed but succeeded!");
    } else {
        println!("✓ Correctly handled invalid scenario");
        println!("  Error: {}", response.error);
    }
    println!();
    println!();

    // ========================================================================
    // Summary
    // ========================================================================
    println!("════════════════════════════════════════════════════════════════");
    println!("Summary");
    println!("════════════════════════════════════════════════════════════════");
    println!();
    println!("The Credential Creation Ceremony consists of:");
    println!();
    println!("1. Begin Phase:");
    println!("   - Server calls BeginMediatedRegistration()");
    println!("   - Returns SessionData (stored securely) and CredentialCreation");
    println!("   - CredentialCreation sent to browser as JSON");
    println!();
    println!("2. Browser Phase:");
    println!("   - navigator.credentials.create(credentialCreation)");
    println!("   - User approves with authenticator (fingerprint, Face ID, etc.)");
    println!("   - Returns PublicKeyCredential");
    println!();
    println!("3. Finish Phase:");
    println!("   - Browser sends PublicKeyCredential back to server");
    println!("   - Server calls FinishRegistration(user, sessionData, response)");
    println!("   - Validates and stores the credential");
    println!();
    println!("Key Points:");
    println!("  • SessionData must be stored securely (opaque to client)");
    println!("  • Use exclusion list to prevent duplicate registration");
    println!("  • Check credProps extension to verify if credential is discoverable");
    println!("  • Resident key requirement affects where credential is stored");
    println!();
    println!("Reference: reference/webauthn/webauthn/doc.go");
    println!("Example code: reference/webauthn/webauthn/registration_test.go");
}
