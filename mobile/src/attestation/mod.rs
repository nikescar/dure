//! Binary attestation verification for Dure
//!
//! This module provides functionality to verify GitHub Artifact Attestations
//! for the running binary, ensuring integrity and provenance.

mod download;
mod types;
mod verify;

pub use download::download_attestations;
pub use types::{AttestationBundle, AttestationResult, VerificationError};
pub use verify::verify_binary;

#[cfg(target_arch = "wasm32")]
pub use verify::verify_wasm;

/// Verify the current running binary against its attestations
///
/// # Arguments
/// * `binary_path` - Path to the binary to verify
/// * `owner` - GitHub repository owner (e.g., "myorg")
/// * `repo` - GitHub repository name (e.g., "dure")
///
/// # Returns
/// * `Ok(AttestationResult)` on successful verification
/// * `Err(VerificationError)` on failure
pub fn verify_current_binary(
    binary_path: &str,
    owner: &str,
    repo: &str,
) -> Result<AttestationResult, VerificationError> {
    verify_binary(binary_path, owner, repo)
}
