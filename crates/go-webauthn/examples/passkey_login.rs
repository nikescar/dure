// Example demonstrating WebAuthn Passkey Login Ceremony
//
// This implements the second ceremony described in reference/webauthn/webauthn/doc.go:
// - BeginDiscoverableMediatedLogin: Start passkey login
// - FinishPasskeyLogin: Complete passkey login
//
// Based on reference/webauthn/webauthn/example_passkey_test.go

use futures::executor::LocalPool;
use go_webauthn::*;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║        WebAuthn Passkey Login Ceremony Example              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("This example demonstrates the Passkey Login Ceremony");
    println!("as defined in go-webauthn/webauthn/doc.go:");
    println!();
    println!("  1. BeginDiscoverableMediatedLogin() - Start passkey login");
    println!("  2. FinishPasskeyLogin() - Complete passkey login");
    println!();
    println!("Key Characteristics:");
    println!("  • Uses DISCOVERABLE CREDENTIALS (resident keys)");
    println!("  • User does NOT need to enter username");
    println!("  • Authenticator stores and returns user handle");
    println!("  • Server discovers user from credential");
    println!();
    println!("════════════════════════════════════════════════════════════════");
    println!();

    // Create a LocalPool executor for running !Send futures
    let mut pool = LocalPool::new();

    // ========================================================================
    // Setup: Register a user with discoverable credential first
    // ========================================================================
    println!("Setup: Registering a user with discoverable credential");
    println!("───────────────────────────────────────────────────────────────");
    println!();

    let signup_req = SignupBeginRequest {
        username: "alice@example.com".to_string(),
        display_name: "Alice Anderson".to_string(),
        scenario: "usernameless".to_string(), // Requires discoverable credential
    };

    let signup_response = pool.run_until(webauthn_signup_begin(&signup_req));

    if signup_response.success {
        println!("✓ User registration initiated");
        println!("  Session ID: {}", signup_response.session_id);
        println!(
            "  Challenge size: {} bytes",
            signup_response.challenge_json.len()
        );
        println!();
        println!("  In a real application:");
        println!("  1. Send challenge to browser");
        println!("  2. Browser calls navigator.credentials.create()");
        println!("  3. User authenticates (fingerprint, Face ID, etc.)");
        println!("  4. Browser sends credential back");
        println!("  5. Server calls signup_finish() to save credential");
    } else {
        println!("✗ User registration failed: {}", signup_response.error);
    }
    println!();
    println!();

    // ========================================================================
    // Passkey Login: Begin Phase
    // ========================================================================
    println!("Passkey Login: Begin Phase");
    println!("──────────────────────────────────────────────────────────────");
    println!();
    println!("Mediation modes:");
    println!("  • 'silent' - No UI if possible");
    println!("  • 'optional' - UI may be shown");
    println!("  • 'conditional' - Autofill UI");
    println!("  • 'required' - Always show UI");
    println!();

    // Test different mediation modes
    let mediation_modes = vec![
        ("optional", "UI may be shown (default)"),
        ("conditional", "Autofill UI (for form fields)"),
        ("required", "Always show UI"),
        ("silent", "No UI if possible"),
    ];

    for (mode, description) in mediation_modes {
        println!("Testing mediation mode: '{}' - {}", mode, description);

        let login_req = PasskeyLoginBeginRequest {
            mediation: mode.to_string(),
        };

        let response = pool.run_until(webauthn_passkey_login_begin(&login_req));

        if response.success {
            println!("  ✓ Login challenge generated");
            println!("    Session ID: {}", response.session_id);
            println!(
                "    Challenge size: {} bytes",
                response.challenge_json.len()
            );

            // Show what's in the challenge
            println!();
            println!("    The challenge includes:");
            println!("      • allowCredentials: EMPTY (discoverable mode)");
            println!("      • userVerification: required");
            println!("      • rpId: localhost");
            println!();
            println!("    In a real application:");
            println!("      1. Send challenge_json to browser");
            println!("      2. Browser calls navigator.credentials.get(challenge)");
            println!("      3. Authenticator shows stored credentials");
            println!("      4. User selects credential and authenticates");
            println!("      5. Browser sends assertion back to server");
            println!("      6. Server calls passkey_login_finish()");
        } else {
            println!("  ✗ Login challenge failed: {}", response.error);
        }
        println!();
    }
    println!();

    // ========================================================================
    // Passkey Login: Finish Phase (Simulated)
    // ========================================================================
    println!("Passkey Login: Finish Phase");
    println!("───────────────────────────────────────────────────────────────");
    println!();
    println!("NOTE: This phase requires a real authenticator response,");
    println!("which can only be generated by an actual browser and device.");
    println!();
    println!("In a real application, the flow would be:");
    println!();
    println!("1. Browser calls navigator.credentials.get(challenge)");
    println!();
    println!("2. Authenticator shows list of stored credentials:");
    println!("   ┌─────────────────────────────────────────┐");
    println!("   │  Choose an account to sign in          │");
    println!("   ├─────────────────────────────────────────┤");
    println!("   │  👤 alice@example.com                   │");
    println!("   │     localhost                           │");
    println!("   ├─────────────────────────────────────────┤");
    println!("   │  👤 bob@example.com                     │");
    println!("   │     localhost                           │");
    println!("   └─────────────────────────────────────────┘");
    println!();
    println!("3. User selects alice@example.com and authenticates");
    println!();
    println!("4. Authenticator returns PublicKeyCredential with:");
    println!("   • rawId: Credential ID");
    println!("   • response.userHandle: alice's user ID");
    println!("   • response.authenticatorData: Attestation data");
    println!("   • response.signature: Cryptographic proof");
    println!();
    println!("5. Browser sends credential to server");
    println!();
    println!("6. Server calls passkey_login_finish():");
    println!();
    println!("   let finish_req = PasskeyLoginFinishRequest {{");
    println!("       session_id: session_id,");
    println!("       credential_json: credential_from_browser,");
    println!("   }};");
    println!();
    println!("   let result = webauthn_passkey_login_finish(&finish_req).await;");
    println!();
    println!("7. FinishPasskeyLogin validates:");
    println!("   • Challenge matches session");
    println!("   • Signature is valid");
    println!("   • User handle maps to known user");
    println!("   • Credential belongs to that user");
    println!("   • Counter prevents replay attacks");
    println!();
    println!("8. On success, returns:");
    println!("   • user_id: User's unique ID");
    println!("   • username: User's username");
    println!("   • session_token: New authenticated session");
    println!();
    println!();

    // ========================================================================
    // Comparison with Multi-Factor Login
    // ========================================================================
    println!("════════════════════════════════════════════════════════════════");
    println!("Passkey Login vs Multi-Factor Login");
    println!("════════════════════════════════════════════════════════════════");
    println!();
    println!("┌─────────────────────┬──────────────────┬──────────────────┐");
    println!("│ Feature             │ Passkey Login    │ Multi-Factor     │");
    println!("├─────────────────────┼──────────────────┼──────────────────┤");
    println!("│ User enters name?   │ NO               │ YES              │");
    println!("│ Discoverable?       │ REQUIRED         │ NOT REQUIRED     │");
    println!("│ User verification   │ Required         │ Varies           │");
    println!("│ allowCredentials    │ Empty            │ Filled           │");
    println!("│ Use case            │ Passwordless     │ Second factor    │");
    println!("│ Begin function      │ BeginDiscoverable│ BeginLogin       │");
    println!("│ Finish function     │ FinishPasskey    │ FinishLogin      │");
    println!("└─────────────────────┴──────────────────┴──────────────────┘");
    println!();
    println!("Passkey Login Advantages:");
    println!("  ✓ Faster - no need to type username");
    println!("  ✓ More secure - credential can't be phished");
    println!("  ✓ Better UX - single gesture authentication");
    println!("  ✓ Privacy - credential doesn't reveal username to site");
    println!();
    println!("Passkey Login Requirements:");
    println!("  • Authenticator must support resident keys");
    println!("  • Credential must be registered as discoverable");
    println!("  • User must have registered on this device/authenticator");
    println!();
    println!();

    // ========================================================================
    // Summary
    // ========================================================================
    println!("════════════════════════════════════════════════════════════════");
    println!("Summary");
    println!("════════════════════════════════════════════════════════════════");
    println!();
    println!("The Passkey Login Ceremony flow:");
    println!();
    println!("1. Server: BeginDiscoverableMediatedLogin(mediation)");
    println!("   → Returns SessionData + CredentialAssertion");
    println!("   → allowCredentials is EMPTY (discoverable mode)");
    println!();
    println!("2. Browser: navigator.credentials.get(assertion)");
    println!("   → Authenticator shows list of stored credentials");
    println!("   → User selects and authenticates");
    println!("   → Returns PublicKeyCredential with userHandle");
    println!();
    println!("3. Server: FinishPasskeyLogin(loadUserFunc, session, response)");
    println!("   → loadUserFunc discovers user from userHandle");
    println!("   → Validates signature and updates credential");
    println!("   → Returns validated user + credential");
    println!();
    println!("Key Implementation Details:");
    println!();
    println!("  • loadUserFunc signature:");
    println!("    func(rawID, userHandle []byte) (webauthn.User, error)");
    println!();
    println!("  • Must look up user by userHandle (not rawID)");
    println!();
    println!("  • Must return user with WebAuthnCredentials() populated");
    println!();
    println!("  • SessionData must be stored securely (opaque to client)");
    println!();
    println!("  • Credential counter must be updated after validation");
    println!();
    println!("Reference: reference/webauthn/webauthn/doc.go");
    println!("Example code: reference/webauthn/webauthn/example_passkey_test.go");
    println!();
}
