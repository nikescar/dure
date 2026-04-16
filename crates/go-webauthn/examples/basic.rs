// Example showing how to use the go-webauthn library
// This demonstrates calling into the Go WebAuthn implementation from Rust

use futures::executor::LocalPool;
use go_webauthn::*;

fn main() {
    println!("go-webauthn library test");
    println!("========================\n");

    // Create a LocalPool executor for running !Send futures
    let mut pool = LocalPool::new();

    // Test 1: MFA Signup Begin
    println!("Test 1: MFA Signup Begin");
    println!("------------------------");
    let signup_req = SignupBeginRequest {
        username: "alice_mfa".to_string(),
        display_name: "Alice Smith (MFA)".to_string(),
        scenario: "mfa".to_string(),
    };

    let response = pool.run_until(webauthn_signup_begin(&signup_req));

    if response.success {
        println!("✓ Success! Session ID: {}", response.session_id);
        println!(
            "  Challenge length: {} bytes",
            response.challenge_json.len()
        );
    } else {
        println!("✗ Failed: {}", response.error);
    }
    println!();

    // Test 2: Passwordless Signup Begin
    println!("Test 2: Passwordless Signup Begin");
    println!("----------------------------------");
    let signup_req = SignupBeginRequest {
        username: "bob_passwordless".to_string(),
        display_name: "Bob Jones".to_string(),
        scenario: "passwordless".to_string(),
    };

    let response = pool.run_until(webauthn_signup_begin(&signup_req));

    if response.success {
        println!("✓ Success! Session ID: {}", response.session_id);
        println!(
            "  Challenge length: {} bytes",
            response.challenge_json.len()
        );
    } else {
        println!("✗ Failed: {}", response.error);
    }
    println!();

    // Test 3: Usernameless Signup Begin
    println!("Test 3: Usernameless Signup Begin");
    println!("----------------------------------");
    let signup_req = SignupBeginRequest {
        username: "".to_string(),
        display_name: "Charlie (Usernameless)".to_string(),
        scenario: "usernameless".to_string(),
    };

    let response = pool.run_until(webauthn_signup_begin(&signup_req));

    if response.success {
        println!("✓ Success! Session ID: {}", response.session_id);
        println!(
            "  Challenge length: {} bytes",
            response.challenge_json.len()
        );
    } else {
        println!("✗ Failed: {}", response.error);
    }
    println!();

    // Test 4: Invalid Scenario (should fail gracefully)
    println!("Test 4: Invalid Scenario (Error Handling)");
    println!("------------------------------------------");
    let signup_req = SignupBeginRequest {
        username: "test".to_string(),
        display_name: "Test User".to_string(),
        scenario: "invalid_scenario".to_string(),
    };

    let response = pool.run_until(webauthn_signup_begin(&signup_req));

    if response.success {
        println!("✗ Should have failed but succeeded!");
    } else {
        println!("✓ Expected failure: {}", response.error);
    }
    println!();

    println!("All tests completed!");
    println!("\nNote: These tests verify the Rust->Go bridge works correctly.");
    println!("They generate challenge data but don't complete the full WebAuthn flow,");
    println!("as that would require browser interaction with a real authenticator.");
}
