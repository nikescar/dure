//! TLS certificate and key loading for the HTTPS/WSS server.

use asupersync::tls::{CertificateChain, PrivateKey, TlsAcceptor};
use std::io;
use std::path::Path;

fn tls_err(e: impl std::fmt::Display) -> io::Error {
    io::Error::other(e.to_string())
}

/// Build a `TlsAcceptor` from PEM certificate + key files.
pub fn create_acceptor(cert_path: &Path, key_path: &Path) -> io::Result<TlsAcceptor> {
    let chain = CertificateChain::from_pem_file(cert_path).map_err(tls_err)?;
    let key = PrivateKey::from_pem_file(key_path).map_err(tls_err)?;
    TlsAcceptor::builder(chain, key).build().map_err(tls_err)
}

/// Generate a self-signed certificate + key for `domain` and write them to
/// `cert_path` / `key_path` as PEM files.  Existing files are overwritten.
pub fn generate_self_signed(domain: &str, cert_path: &Path, key_path: &Path) -> io::Result<()> {
    use rcgen::{CertifiedKey, generate_simple_self_signed};
    let subject_alt_names = vec![domain.to_string(), "localhost".to_string()];
    let CertifiedKey { cert, key_pair } =
        generate_simple_self_signed(subject_alt_names).map_err(tls_err)?;
    std::fs::write(cert_path, cert.pem())?;
    std::fs::write(key_path, key_pair.serialize_pem())?;
    Ok(())
}
