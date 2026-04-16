// Example demonstrating WebAuthn Multi-Factor Login Ceremony
//
// This implements the third ceremony described in reference/webauthn/webauthn/doc.go:
// - BeginMediatedLogin: Start multi-factor login
// - FinishLogin: Complete multi-factor login
//
// Based on reference/webauthn/webauthn/example_multifactor_test.go

use futures::executor::LocalPool;
use go_webauthn::*;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║      WebAuthn Multi-Factor Login Ceremony Example           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("This example demonstrates the Multi-Factor Login Ceremony");
    println!("as defined in go-webauthn/webauthn/doc.go:");
    println!();
    println!("  1. BeginMediatedLogin() - Start multi-factor login");
    println!("  2. FinishLogin() - Complete multi-factor login");
    println!();
    println!("Key Characteristics:");
    println!("  • User is ALREADY AUTHENTICATED (1st factor complete)");
    println!("  • Server KNOWS which user is logging in");
    println!("  • allowCredentials list is POPULATED");
    println!("  • Authenticator only allows user's own credentials");
    println!("  • Used as 2nd factor after password/email/etc.");
    println!();
    println!("════════════════════════════════════════════════════════════════");
    println!();

    // Create a LocalPool executor for running !Send futures
    let mut pool = LocalPool::new();

    // ========================================================================
    // Setup: Register users with credentials first
    // ========================================================================
    println!("Setup: Registering users with MFA credentials");
    println!("───────────────────────────────────────────────────────────────");
    println!();

    // Register Alice with MFA credential
    let alice_signup = SignupBeginRequest {
        username: "alice@example.com".to_string(),
        display_name: "Alice Anderson".to_string(),
        scenario: "mfa".to_string(), // MFA scenario
    };

    let alice_response = pool.run_until(webauthn_signup_begin(&alice_signup));

    if alice_response.success {
        println!("✓ Alice's MFA credential registration initiated");
        println!("  Session ID: {}", alice_response.session_id);
        println!();
        println!("  In a real application:");
        println!("  1. Alice first logs in with password");
        println!("  2. Server asks her to register MFA credential");
        println!("  3. She uses fingerprint/Face ID as 2nd factor");
        println!("  4. Credential is saved for future logins");
    } else {
        println!("✗ Alice's registration failed: {}", alice_response.error);
    }
    println!();

    // Register Bob with MFA credential
    let bob_signup = SignupBeginRequest {
        username: "bob@example.com".to_string(),
        display_name: "Bob Brown".to_string(),
        scenario: "mfa".to_string(),
    };

    let bob_response = pool.run_until(webauthn_signup_begin(&bob_signup));

    if bob_response.success {
        println!("✓ Bob's MFA credential registration initiated");
        println!("  Session ID: {}", bob_response.session_id);
    } else {
        println!("✗ Bob's registration failed: {}", bob_response.error);
    }
    println!();
    println!();

    // ========================================================================
    // Multi-Factor Login: Begin Phase
    // ========================================================================
    println!("Multi-Factor Login: Begin Phase");
    println!("──────────────────────────────────────────────────────────────");
    println!();
    println!("Typical flow:");
    println!("  1. User enters username + password (1st factor)");
    println!("  2. Server validates password ✓");
    println!("  3. Server creates session (partial authentication)");
    println!("  4. Server calls BeginMediatedLogin() for 2nd factor");
    println!();

    let login_req = MfaLoginBeginRequest {
        username: "alice@example.com".to_string(),
        mediation: "optional".to_string(),
    };

    let response = pool.run_until(webauthn_mfa_login_begin(&login_req));

    if response.success {
        println!("✓ MFA login challenge generated for Alice");
        println!("  Session ID: {}", response.session_id);
        println!("  Challenge size: {} bytes", response.challenge_json.len());
        println!();
        println!("  The challenge includes:");
        println!("    • allowCredentials: FILLED (Alice's registered credentials)");
        println!("    • userVerification: depends on registration settings");
        println!("    • rpId: localhost");
        println!();
        println!("  Key difference from Passkey Login:");
        println!("    • allowCredentials is POPULATED (only Alice's credentials)");
        println!("    • Server already knows it's Alice (from 1st factor)");
        println!("    • Authenticator will reject other users' credentials");
    } else {
        println!("✗ MFA login challenge failed: {}", response.error);
    }
    println!();
    println!();

    // ========================================================================
    // Different Mediation Modes
    // ========================================================================
    println!("Testing Different Mediation Modes");
    println!("──────────────────────────────────────────────────────────────");
    println!();

    let mediation_modes = vec![
        ("optional", "UI may be shown (default)"),
        ("required", "Always show UI"),
        ("conditional", "Autofill UI (for form fields)"),
        ("silent", "No UI if possible"),
    ];

    for (mode, description) in mediation_modes {
        println!("Mediation: '{}' - {}", mode, description);

        let req = MfaLoginBeginRequest {
            username: "bob@example.com".to_string(),
            mediation: mode.to_string(),
        };

        let response = pool.run_until(webauthn_mfa_login_begin(&req));

        if response.success {
            println!("  ✓ Challenge generated (session: {})", response.session_id);
        } else {
            println!("  ✗ Failed: {}", response.error);
        }
        println!();
    }
    println!();

    // ========================================================================
    // Multi-Factor Login: Finish Phase (Simulated)
    // ========================================================================
    println!("Multi-Factor Login: Finish Phase");
    println!("───────────────────────────────────────────────────────────────");
    println!();
    println!("NOTE: This phase requires a real authenticator response,");
    println!("which can only be generated by an actual browser and device.");
    println!();
    println!("In a real application, the flow would be:");
    println!();
    println!("1. Browser calls navigator.credentials.get(challenge)");
    println!();
    println!("2. Authenticator receives request with allowCredentials:");
    println!("   {{");
    println!("     challenge: <base64-challenge>,");
    println!("     allowCredentials: [");
    println!("       {{ type: 'public-key', id: <alice-cred-1> }},");
    println!("       {{ type: 'public-key', id: <alice-cred-2> }}");
    println!("     ],");
    println!("     userVerification: 'discouraged',");
    println!("     rpId: 'localhost'");
    println!("   }}");
    println!();
    println!("3. Authenticator checks if any allowed credential is present:");
    println!("   • If yes: Prompt user to authenticate");
    println!("   • If no: Return error (user doesn't have credential)");
    println!();
    println!("4. User authenticates (fingerprint, Face ID, PIN, etc.)");
    println!();
    println!("5. Authenticator returns PublicKeyCredential:");
    println!("   • rawId: Credential ID (matches allowCredentials)");
    println!("   • response.authenticatorData: Attestation data");
    println!("   • response.signature: Cryptographic proof");
    println!("   • response.userHandle: Optional (may be empty)");
    println!();
    println!("6. Browser sends credential to server");
    println!();
    println!("7. Server calls mfa_login_finish():");
    println!();
    println!("   let finish_req = MfaLoginFinishRequest {{");
    println!("       session_id: session_id,");
    println!("       credential_json: credential_from_browser,");
    println!("   }};");
    println!();
    println!("   let result = webauthn_mfa_login_finish(&finish_req).await;");
    println!();
    println!("8. FinishLogin validates:");
    println!("   • Challenge matches session");
    println!("   • Credential ID is in allowCredentials list");
    println!("   • Signature is valid");
    println!("   • Credential belongs to the authenticated user");
    println!("   • Counter prevents replay attacks");
    println!();
    println!("9. On success, returns:");
    println!("   • user_id: User's unique ID");
    println!("   • session_token: Fully authenticated session");
    println!();
    println!("10. Server upgrades session to fully authenticated");
    println!();
    println!();

    // ========================================================================
    // Security Considerations
    // ========================================================================
    println!("════════════════════════════════════════════════════════════════");
    println!("Security Considerations");
    println!("════════════════════════════════════════════════════════════════");
    println!();
    println!("1. Session Management:");
    println!("   • 1st factor (password) creates partial auth session");
    println!("   • Session must be bound to user agent (HTTP-only cookie)");
    println!("   • 2nd factor (WebAuthn) upgrades to full auth");
    println!("   • Sessions must have timeout and CSRF protection");
    println!();
    println!("2. allowCredentials List:");
    println!("   • MUST only include user's own credentials");
    println!("   • Prevents credential swapping attacks");
    println!("   • Server knows which user based on 1st factor");
    println!();
    println!("3. Credential Storage:");
    println!("   • Update credential counter after each use");
    println!("   • Detect and reject counter rollback (replay attacks)");
    println!("   • Store credentials securely (encrypted at rest)");
    println!();
    println!("4. User Verification:");
    println!("   • For MFA, can be 'discouraged' (possession is enough)");
    println!("   • For higher security, use 'required'");
    println!("   • Depends on your threat model and UX requirements");
    println!();
    println!();

    // ========================================================================
    // Comparison Table
    // ========================================================================
    println!("════════════════════════════════════════════════════════════════");
    println!("Ceremony Comparison");
    println!("════════════════════════════════════════════════════════════════");
    println!();
    println!("┌────────────────────┬─────────────┬─────────────┬──────────────┐");
    println!("│ Feature            │ Credential  │ Passkey     │ Multi-Factor │");
    println!("│                    │ Creation    │ Login       │ Login        │");
    println!("├────────────────────┼─────────────┼─────────────┼──────────────┤");
    println!("│ Purpose            │ Register    │ Sign in     │ 2nd factor   │");
    println!("│ User known?        │ Yes         │ No          │ Yes          │");
    println!("│ Discoverable?      │ Varies      │ Required    │ Optional     │");
    println!("│ allowCredentials   │ Exclusions  │ Empty       │ Filled       │");
    println!("│ User verification  │ Varies      │ Required    │ Varies       │");
    println!("│ Begin function     │ BeginReg    │ BeginDisc   │ BeginLogin   │");
    println!("│ Finish function    │ FinishReg   │ FinishPass  │ FinishLogin  │");
    println!("└────────────────────┴─────────────┴─────────────┴──────────────┘");
    println!();
    println!("Use Cases:");
    println!();
    println!("  Credential Creation:");
    println!("    • First-time user registration");
    println!("    • Adding additional authenticator");
    println!("    • Upgrading account to passkey");
    println!();
    println!("  Passkey Login:");
    println!("    • Passwordless authentication");
    println!("    • Single-gesture sign-in");
    println!("    • No username required");
    println!("    • Best UX but requires resident key support");
    println!();
    println!("  Multi-Factor Login:");
    println!("    • 2nd factor after password");
    println!("    • Step-up authentication (e.g., before sensitive action)");
    println!("    • Gradual migration from passwords");
    println!("    • Works with non-resident keys");
    println!();
    println!();

    // ========================================================================
    // Summary
    // ========================================================================
    println!("════════════════════════════════════════════════════════════════");
    println!("Summary");
    println!("════════════════════════════════════════════════════════════════");
    println!();
    println!("The Multi-Factor Login Ceremony flow:");
    println!();
    println!("1. User enters username + password (1st factor)");
    println!("   → Server validates and creates partial auth session");
    println!();
    println!("2. Server: BeginMediatedLogin(user, mediation)");
    println!("   → Returns SessionData + CredentialAssertion");
    println!("   → allowCredentials is FILLED with user's credentials");
    println!();
    println!("3. Browser: navigator.credentials.get(assertion)");
    println!("   → Authenticator checks for matching credential");
    println!("   → User authenticates (2nd factor)");
    println!("   → Returns PublicKeyCredential");
    println!();
    println!("4. Server: FinishLogin(user, session, response)");
    println!("   → Validates signature and credential");
    println!("   → Updates credential counter");
    println!("   → Returns validated credential");
    println!();
    println!("5. Server upgrades session to fully authenticated");
    println!();
    println!("Key Implementation Details:");
    println!();
    println!("  • User must be known before calling BeginMediatedLogin");
    println!();
    println!("  • User object must implement WebAuthnCredentials()");
    println!();
    println!("  • allowCredentials limits which credentials are acceptable");
    println!();
    println!("  • SessionData must be stored securely between begin/finish");
    println!();
    println!("  • Credential counter must be updated to prevent replays");
    println!();
    println!("  • Session must be upgraded only after successful validation");
    println!();
    println!("Reference: reference/webauthn/webauthn/doc.go");
    println!("Example code: reference/webauthn/webauthn/example_multifactor_test.go");
    println!();
}
