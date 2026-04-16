//! ACME (Automatic Certificate Management Environment) functionality
//!
//! Provides SSL certificate management using acme.sh with SQLite-based storage
//! for certificate results and renewal tracking.

use anyhow::{Context, Result};
use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Certificate information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    pub domain: String,
    pub cert_path: String,
    pub key_path: String,
    pub ca_path: String,
    pub fullchain_path: String,
    pub issued_at: u64,
    pub expires_at: u64,
    pub is_valid: bool,
}

impl Certificate {
    pub fn new(
        domain: String,
        cert_path: String,
        key_path: String,
        ca_path: String,
        fullchain_path: String,
        issued_at: u64,
        expires_at: u64,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();

        Self {
            domain,
            cert_path,
            key_path,
            ca_path,
            fullchain_path,
            issued_at,
            expires_at,
            is_valid: now < expires_at,
        }
    }

    /// Check if certificate needs renewal (within 30 days of expiry)
    pub fn needs_renewal(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();

        // Renew if less than 30 days until expiry
        const RENEWAL_THRESHOLD: u64 = 30 * 24 * 60 * 60; // 30 days
        now + RENEWAL_THRESHOLD >= self.expires_at
    }

    /// Check if certificate is still valid
    pub fn is_still_valid(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();

        now < self.expires_at
    }
}

/// Represents a certificate entry from `acme.sh --list`
#[derive(Debug, Clone)]
pub struct AcmeListEntry {
    pub main_domain: String,
    pub key_length: String,
    pub san_domains: Vec<String>,
    /// Unix timestamp parsed from acme.sh Created field
    pub created_at: Option<u64>,
    /// Unix timestamp parsed from acme.sh Renew field
    pub renew_at: Option<u64>,
    /// Original Created string for display
    pub created_str: String,
    /// Original Renew string for display
    pub renew_str: String,
}

/// Check if acme.sh is installed on the system.
///
/// Mirrors acme.sh's own install detection logic:
/// - First checks `LE_WORKING_DIR` environment variable
/// - Falls back to checking `~/.acme.sh/account.conf`
///
/// Returns the working directory path if acme.sh is installed, or `None` otherwise.
pub fn check_acme_installed() -> Option<String> {
    // Check LE_WORKING_DIR env var first (user may have custom install location)
    if let Ok(le_dir) = std::env::var("LE_WORKING_DIR") {
        if std::path::Path::new(&format!("{}/account.conf", le_dir)).exists() {
            return Some(le_dir);
        }
    }

    // Check default install location: ~/.acme.sh
    if let Ok(home) = std::env::var("HOME") {
        let default_home = format!("{}/.acme.sh", home);
        if std::path::Path::new(&format!("{}/account.conf", default_home)).exists() {
            return Some(default_home);
        }
    }

    None
}

/// Get the path to the acme.sh binary, respecting LE_WORKING_DIR.
pub fn get_acme_sh_path() -> Result<String> {
    let working_dir = check_acme_installed().unwrap_or_else(|| {
        std::env::var("HOME")
            .map(|h| format!("{}/.acme.sh", h))
            .unwrap_or_default()
    });
    Ok(format!("{}/acme.sh", working_dir))
}

/// List certificates currently managed by the acme.sh system.
///
/// Runs `acme.sh --list` and parses its output into structured entries.
///
/// # Examples
///
/// ```rust,no_run
/// use dure::calc::acme::list_system_certificates;
///
/// # fn example() -> anyhow::Result<()> {
/// let certs = list_system_certificates()?;
/// for cert in certs {
///     println!("{}: created {}", cert.main_domain, cert.created_str);
/// }
/// # Ok(())
/// # }
/// ```
pub fn list_system_certificates() -> Result<Vec<AcmeListEntry>> {
    #[cfg(not(target_os = "linux"))]
    {
        anyhow::bail!("acme.sh is only supported on Linux");
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;

        let acme_sh = get_acme_sh_path()?;

        let output = Command::new(&acme_sh)
            .arg("--list")
            .output()
            .context("Failed to execute acme.sh --list")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("acme.sh --list failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        parse_acme_list_output(&stdout)
    }
}

/// Parse the output of `acme.sh --list` into structured entries.
///
/// acme.sh uses fixed-width padded columns separated by 2+ spaces.
fn parse_acme_list_output(output: &str) -> Result<Vec<AcmeListEntry>> {
    let mut entries = Vec::new();

    for (i, line) in output.lines().enumerate() {
        if i == 0 {
            continue; // skip header row
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let cols = split_padded_columns(line);
        if cols.is_empty() {
            continue;
        }

        let main_domain = cols.first().cloned().unwrap_or_default();
        let key_length = cols.get(1).cloned().unwrap_or_default();
        let san_str = cols.get(2).cloned().unwrap_or_default();
        let created_str = cols.get(3).cloned().unwrap_or_default();
        let renew_str = cols.get(4).cloned().unwrap_or_default();

        let san_domains = san_str.split_whitespace().map(|s| s.to_string()).collect();

        entries.push(AcmeListEntry {
            main_domain,
            key_length,
            san_domains,
            created_at: parse_acme_date(&created_str),
            renew_at: parse_acme_date(&renew_str),
            created_str,
            renew_str,
        });
    }

    Ok(entries)
}

/// Split a padded-column line by runs of 2+ spaces.
///
/// acme.sh `--list` output uses printf-style fixed-width columns separated by
/// at least two spaces, so single spaces may appear within a column value (e.g.
/// SAN domain lists).
fn split_padded_columns(line: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut space_count = 0usize;

    for ch in line.chars() {
        if ch == ' ' {
            space_count += 1;
            if space_count < 2 {
                current.push(ch);
            }
        } else {
            if space_count >= 2 {
                let trimmed = current.trim().to_string();
                if !trimmed.is_empty() {
                    result.push(trimmed);
                }
                current = String::new();
            }
            space_count = 0;
            current.push(ch);
        }
    }

    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        result.push(trimmed);
    }

    result
}

/// Parse an acme.sh date string (`YYYY-MM-DD,HH:MM:SS`) into a Unix timestamp.
fn parse_acme_date(date_str: &str) -> Option<u64> {
    // acme.sh format: "2024-01-15,06:31:19"
    let s = date_str.replace(',', " ");
    let (date_part, time_part) = s.split_once(' ')?;

    let mut date_nums = date_part.split('-');
    let year: i32 = date_nums.next()?.parse().ok()?;
    let month: u32 = date_nums.next()?.parse().ok()?;
    let day: u32 = date_nums.next()?.parse().ok()?;

    let mut time_nums = time_part.split(':');
    let hour: u32 = time_nums.next()?.parse().ok()?;
    let min: u32 = time_nums.next()?.parse().ok()?;
    let sec: u32 = time_nums.next()?.parse().ok()?;

    let dt = Utc
        .with_ymd_and_hms(year, month, day, hour, min, sec)
        .single()?;
    Some(dt.timestamp() as u64)
}

/// Install acme.sh to the system
///
/// This function will download and install acme.sh if not already installed.
/// Requires root/sudo privileges on Linux.
///
/// # Examples
///
/// ```rust,no_run
/// use dure::calc::acme::install_acme;
///
/// # async fn example() -> anyhow::Result<()> {
/// install_acme()?;
/// # Ok(())
/// # }
/// ```
pub fn install_acme() -> Result<()> {
    #[cfg(not(target_os = "linux"))]
    {
        anyhow::bail!("ACME installation is only supported on Linux");
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;

        // Check if acme.sh is already installed
        let home = std::env::var("HOME").context("HOME environment variable not set")?;
        let acme_sh_path = format!("{}/.acme.sh/acme.sh", home);

        if std::path::Path::new(&acme_sh_path).exists() {
            eprintln!("acme.sh is already installed at {}", acme_sh_path);
            return Ok(());
        }

        // Install acme.sh using the official installation script
        eprintln!("Installing acme.sh...");

        let output = Command::new("sh")
            .arg("-c")
            .arg("curl https://get.acme.sh | sh -s email=my@example.com")
            .output()
            .context("Failed to execute acme.sh installation")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to install acme.sh: {}", stderr);
        }

        eprintln!("acme.sh installed successfully");
        Ok(())
    }
}

/// Issue a new certificate for the specified domain
///
/// Uses standalone mode which requires port 80/443 to be available.
/// Supports multiple domains with -d flags.
///
/// # Examples
///
/// ```rust,no_run
/// use dure::calc::acme::issue_certificate;
///
/// # async fn example() -> anyhow::Result<()> {
/// issue_certificate(&["example.com", "www.example.com"])?;
/// # Ok(())
/// # }
/// ```
pub fn issue_certificate(domains: &[&str]) -> Result<Certificate> {
    #[cfg(not(target_os = "linux"))]
    {
        anyhow::bail!("ACME certificate issuance is only supported on Linux");
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;

        if domains.is_empty() {
            anyhow::bail!("At least one domain must be specified");
        }

        let home = std::env::var("HOME").context("HOME environment variable not set")?;
        let acme_sh = format!("{}/.acme.sh/acme.sh", home);

        // Build domain arguments
        let mut domain_args = Vec::new();
        for domain in domains {
            domain_args.push("-d");
            domain_args.push(*domain);
        }

        eprintln!("Issuing certificate for: {}", domains.join(", "));

        let output = Command::new(&acme_sh)
            .arg("--issue")
            .arg("--standalone")
            .args(&domain_args)
            .output()
            .context("Failed to execute acme.sh --issue")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to issue certificate: {}", stderr);
        }

        // Parse certificate paths from acme.sh directory structure
        let primary_domain = domains[0];
        let cert_dir = format!("{}/.acme.sh/{}", home, primary_domain);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();

        // Certificates are valid for 90 days
        let expires_at = now + 90 * 24 * 60 * 60;

        Ok(Certificate::new(
            primary_domain.to_string(),
            format!("{}/{}.cer", cert_dir, primary_domain),
            format!("{}/{}.key", cert_dir, primary_domain),
            format!("{}/ca.cer", cert_dir),
            format!("{}/fullchain.cer", cert_dir),
            now,
            expires_at,
        ))
    }
}

/// Renew an existing certificate for the specified domain
///
/// Typically should be run every 60 days. acme.sh will check if renewal is needed.
///
/// # Examples
///
/// ```rust,no_run
/// use dure::calc::acme::renew_certificate;
///
/// # async fn example() -> anyhow::Result<()> {
/// renew_certificate("example.com", false)?;
/// # Ok(())
/// # }
/// ```
pub fn renew_certificate(domain: &str, force: bool) -> Result<Certificate> {
    #[cfg(not(target_os = "linux"))]
    {
        anyhow::bail!("ACME certificate renewal is only supported on Linux");
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;

        let home = std::env::var("HOME").context("HOME environment variable not set")?;
        let acme_sh = format!("{}/.acme.sh/acme.sh", home);

        eprintln!("Renewing certificate for: {}", domain);

        let mut cmd = Command::new(&acme_sh);
        cmd.arg("--renew").arg("-d").arg(domain);

        if force {
            cmd.arg("--force");
        }

        let output = cmd.output().context("Failed to execute acme.sh --renew")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to renew certificate: {}", stderr);
        }

        let cert_dir = format!("{}/.acme.sh/{}", home, domain);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();

        let expires_at = now + 90 * 24 * 60 * 60;

        Ok(Certificate::new(
            domain.to_string(),
            format!("{}/{}.cer", cert_dir, domain),
            format!("{}/{}.key", cert_dir, domain),
            format!("{}/ca.cer", cert_dir),
            format!("{}/fullchain.cer", cert_dir),
            now,
            expires_at,
        ))
    }
}

/// DNS provider configuration for setting records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsProvider {
    pub provider_type: DnsProviderType,
    pub api_token: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DnsProviderType {
    Cloudflare,
    GoogleCloud,
    DuckDNS,
    Porkbun,
}

impl DnsProviderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DnsProviderType::Cloudflare => "cloudflare",
            DnsProviderType::GoogleCloud => "gcloud",
            DnsProviderType::DuckDNS => "duckdns",
            DnsProviderType::Porkbun => "porkbun",
        }
    }
}

/// Set A record for a domain
///
/// # Examples
///
/// ```rust,no_run
/// use dure::calc::acme::{DnsProvider, DnsProviderType, set_a_record};
///
/// # async fn example() -> anyhow::Result<()> {
/// let provider = DnsProvider {
///     provider_type: DnsProviderType::Cloudflare,
///     api_token: "your_api_token".to_string(),
/// };
/// set_a_record(&provider, "example.com", "1.2.3.4")?;
/// # Ok(())
/// # }
/// ```
pub fn set_a_record(provider: &DnsProvider, domain: &str, ip: &str) -> Result<()> {
    match provider.provider_type {
        DnsProviderType::Cloudflare => set_cloudflare_a_record(&provider.api_token, domain, ip),
        DnsProviderType::GoogleCloud => {
            // TODO: Implement Google Cloud DNS API
            // Reference: https://github.com/googleapis/google-cloud-rust/blob/main/tests/dns/src/lib.rs
            anyhow::bail!("Google Cloud DNS not yet implemented")
        }
        DnsProviderType::DuckDNS => set_duckdns_a_record(&provider.api_token, domain, ip),
        DnsProviderType::Porkbun => set_porkbun_a_record(&provider.api_token, domain, ip),
    }
}

/// Set TXT record for a domain
///
/// Used for storing public keys or verification records.
///
/// # Examples
///
/// ```rust,no_run
/// use dure::calc::acme::{DnsProvider, DnsProviderType, set_txt_record};
///
/// # async fn example() -> anyhow::Result<()> {
/// let provider = DnsProvider {
///     provider_type: DnsProviderType::Cloudflare,
///     api_token: "your_api_token".to_string(),
/// };
/// // NOTE: Public key generation not yet implemented
/// // set_txt_record(&provider, "example.com", "durepubkey=...")?;
/// # Ok(())
/// # }
/// ```
pub fn set_txt_record(provider: &DnsProvider, domain: &str, txt_value: &str) -> Result<()> {
    match provider.provider_type {
        DnsProviderType::Cloudflare => {
            set_cloudflare_txt_record(&provider.api_token, domain, txt_value)
        }
        DnsProviderType::GoogleCloud => {
            // TODO: Implement Google Cloud DNS API
            anyhow::bail!("Google Cloud DNS not yet implemented")
        }
        DnsProviderType::DuckDNS => set_duckdns_txt_record(&provider.api_token, domain, txt_value),
        DnsProviderType::Porkbun => set_porkbun_txt_record(&provider.api_token, domain, txt_value),
    }
}

// Cloudflare DNS API implementations
fn set_cloudflare_a_record(_api_token: &str, domain: &str, ip: &str) -> Result<()> {
    // Reference: https://github.com/acmesh-official/acme.sh/blob/master/dnsapi/dns_cf.sh
    // TODO: Implement Cloudflare API call using ureq
    // Steps:
    // 1. Get zone ID for domain
    // 2. Create or update A record
    eprintln!("Setting Cloudflare A record: {} -> {}", domain, ip);
    eprintln!(
        "API Token: {}...",
        &_api_token.chars().take(8).collect::<String>()
    );
    // Implementation placeholder
    anyhow::bail!("Cloudflare A record API not yet implemented")
}

fn set_cloudflare_txt_record(_api_token: &str, domain: &str, txt_value: &str) -> Result<()> {
    // Reference: https://github.com/acmesh-official/acme.sh/blob/master/dnsapi/dns_cf.sh
    eprintln!("Setting Cloudflare TXT record: {} -> {}", domain, txt_value);
    eprintln!(
        "API Token: {}...",
        &_api_token.chars().take(8).collect::<String>()
    );
    // Implementation placeholder
    anyhow::bail!("Cloudflare TXT record API not yet implemented")
}

// DuckDNS API implementations
fn set_duckdns_a_record(_token: &str, domain: &str, ip: &str) -> Result<()> {
    // Reference: https://github.com/acmesh-official/acme.sh/blob/master/dnsapi/dns_duckdns.sh
    // DuckDNS API: https://www.duckdns.org/update?domains={domain}&token={token}&ip={ip}
    eprintln!("Setting DuckDNS A record: {} -> {}", domain, ip);
    eprintln!("Token: {}...", &_token.chars().take(8).collect::<String>());
    // Implementation placeholder
    anyhow::bail!("DuckDNS A record API not yet implemented")
}

fn set_duckdns_txt_record(_token: &str, domain: &str, txt_value: &str) -> Result<()> {
    // Reference: https://github.com/acmesh-official/acme.sh/blob/master/dnsapi/dns_duckdns.sh
    eprintln!("Setting DuckDNS TXT record: {} -> {}", domain, txt_value);
    eprintln!("Token: {}...", &_token.chars().take(8).collect::<String>());
    // Implementation placeholder
    anyhow::bail!("DuckDNS TXT record API not yet implemented")
}

// Porkbun API implementations
fn set_porkbun_a_record(_api_token: &str, domain: &str, ip: &str) -> Result<()> {
    // Reference: https://github.com/acmesh-official/acme.sh/blob/master/dnsapi/dns_porkbun.sh
    // Porkbun API: https://porkbun.com/api/json/v3/dns/create/{domain}
    eprintln!("Setting Porkbun A record: {} -> {}", domain, ip);
    eprintln!(
        "API Token: {}...",
        &_api_token.chars().take(8).collect::<String>()
    );
    // Implementation placeholder
    anyhow::bail!("Porkbun A record API not yet implemented")
}

fn set_porkbun_txt_record(_api_token: &str, domain: &str, txt_value: &str) -> Result<()> {
    // Reference: https://github.com/acmesh-official/acme.sh/blob/master/dnsapi/dns_porkbun.sh
    eprintln!("Setting Porkbun TXT record: {} -> {}", domain, txt_value);
    eprintln!(
        "API Token: {}...",
        &_api_token.chars().take(8).collect::<String>()
    );
    // Implementation placeholder
    anyhow::bail!("Porkbun TXT record API not yet implemented")
}

// TODO: Public key generation placeholder
// This will be implemented when cryptographic key management is ready
//
// /// Generate ChaCha20 public key for system
// ///
// /// # Examples
// ///
// /// ```rust,no_run
// /// use dure::calc::acme::generate_pubkey;
// ///
// /// # fn example() -> anyhow::Result<()> {
// /// let pubkey = generate_pubkey()?;
// /// println!("Public key: {}", pubkey);
// /// # Ok(())
// /// # }
// /// ```
// pub fn generate_pubkey() -> Result<String> {
//     // TODO: Implement X25519 + ChaCha20 key generation
//     // Reference: Use chacha20poly1305 and x25519-dalek crates
//     anyhow::bail!("Public key generation not yet implemented")
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_certificate_validity() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let cert = Certificate::new(
            "example.com".to_string(),
            "/path/cert".to_string(),
            "/path/key".to_string(),
            "/path/ca".to_string(),
            "/path/fullchain".to_string(),
            now,
            now + 90 * 24 * 60 * 60, // 90 days
        );

        assert!(cert.is_still_valid());
        assert!(!cert.needs_renewal()); // Should not need renewal yet
    }

    #[test]
    fn test_certificate_needs_renewal() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Certificate expiring in 20 days
        let cert = Certificate::new(
            "example.com".to_string(),
            "/path/cert".to_string(),
            "/path/key".to_string(),
            "/path/ca".to_string(),
            "/path/fullchain".to_string(),
            now,
            now + 20 * 24 * 60 * 60,
        );

        assert!(cert.is_still_valid());
        assert!(cert.needs_renewal()); // Should need renewal (< 30 days)
    }

    #[test]
    fn test_dns_provider_type() {
        assert_eq!(DnsProviderType::Cloudflare.as_str(), "cloudflare");
        assert_eq!(DnsProviderType::GoogleCloud.as_str(), "gcloud");
        assert_eq!(DnsProviderType::DuckDNS.as_str(), "duckdns");
        assert_eq!(DnsProviderType::Porkbun.as_str(), "porkbun");
    }
}
