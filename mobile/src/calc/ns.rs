//! NS (nameserver) management controller
//!
//! Provides domain and DNS record management functionality for multiple providers:
//! - Cloudflare
//! - Google Cloud DNS
//! - DuckDNS
//! - Porkbun
//!
//! Supports A, AAAA, TXT, and SSHFP record types.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
use crate::calc::acme::{DnsProvider, DnsProviderType, set_a_record, set_txt_record};

/// DNS record type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecordType {
    A,
    AAAA,
    TXT,
    SSHFP,
}

impl RecordType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RecordType::A => "a",
            RecordType::AAAA => "aaaa",
            RecordType::TXT => "txt",
            RecordType::SSHFP => "sshfp",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "a" => Some(RecordType::A),
            "aaaa" => Some(RecordType::AAAA),
            "txt" => Some(RecordType::TXT),
            "sshfp" => Some(RecordType::SSHFP),
            _ => None,
        }
    }
}

/// DNS record entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecord {
    pub record_type: RecordType,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u32>,
}

/// Domain configuration with DNS records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEntry {
    pub domain: String,
    pub provider: String, // "cloudflare", "gcloud", "duckdns", "porkbun"
    pub api_token: String,
    #[serde(default)]
    pub records: Vec<DnsRecord>,
}

/// NS configuration stored in config.yml
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NsConfig {
    #[serde(default)]
    pub domains: Vec<DomainEntry>,
}

impl NsConfig {
    /// Load from YAML string
    pub fn from_yaml(yaml: &str) -> Result<Self> {
        serde_yaml::from_str(yaml).context("Failed to parse NS config")
    }

    /// Save to YAML string
    pub fn to_yaml(&self) -> Result<String> {
        serde_yaml::to_string(self).context("Failed to serialize NS config")
    }

    /// Get domain entry by name
    pub fn get_domain(&self, domain: &str) -> Option<&DomainEntry> {
        self.domains.iter().find(|d| d.domain == domain)
    }

    /// Get mutable domain entry by name
    pub fn get_domain_mut(&mut self, domain: &str) -> Option<&mut DomainEntry> {
        self.domains.iter_mut().find(|d| d.domain == domain)
    }

    /// Add a new domain
    pub fn add_domain(
        &mut self,
        domain: String,
        provider: String,
        api_token: String,
    ) -> Result<()> {
        if self.get_domain(&domain).is_some() {
            anyhow::bail!("Domain {} already exists", domain);
        }

        self.domains.push(DomainEntry {
            domain,
            provider,
            api_token,
            records: Vec::new(),
        });

        Ok(())
    }

    /// Remove a domain
    pub fn remove_domain(&mut self, domain: &str) -> Result<()> {
        let index = self
            .domains
            .iter()
            .position(|d| d.domain == domain)
            .ok_or_else(|| anyhow::anyhow!("Domain {} not found", domain))?;

        self.domains.remove(index);
        Ok(())
    }

    /// Add a record to a domain
    pub fn add_record(
        &mut self,
        domain: &str,
        record_type: RecordType,
        value: String,
    ) -> Result<()> {
        let domain_entry = self
            .get_domain_mut(domain)
            .ok_or_else(|| anyhow::anyhow!("Domain {} not found", domain))?;

        // Check if record already exists
        if domain_entry
            .records
            .iter()
            .any(|r| r.record_type == record_type && r.value == value)
        {
            anyhow::bail!(
                "Record already exists: {} {} {}",
                domain,
                record_type.as_str(),
                value
            );
        }

        domain_entry.records.push(DnsRecord {
            record_type,
            value,
            ttl: None,
        });

        Ok(())
    }

    /// Remove a record from a domain
    pub fn remove_record(
        &mut self,
        domain: &str,
        record_type: RecordType,
        value: &str,
    ) -> Result<()> {
        let domain_entry = self
            .get_domain_mut(domain)
            .ok_or_else(|| anyhow::anyhow!("Domain {} not found", domain))?;

        let index = domain_entry
            .records
            .iter()
            .position(|r| r.record_type == record_type && r.value == value)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Record not found: {} {} {}",
                    domain,
                    record_type.as_str(),
                    value
                )
            })?;

        domain_entry.records.remove(index);
        Ok(())
    }

    /// List all domains
    pub fn list_domains(&self) -> Vec<String> {
        self.domains.iter().map(|d| d.domain.clone()).collect()
    }

    /// Get all records for a domain
    pub fn get_records(&self, domain: &str) -> Option<Vec<DnsRecord>> {
        self.get_domain(domain).map(|d| d.records.clone())
    }
}

/// Apply DNS record to actual DNS provider
#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
pub fn apply_record(domain_entry: &DomainEntry, record: &DnsRecord) -> Result<()> {
    let provider_type = match domain_entry.provider.to_lowercase().as_str() {
        "cloudflare" | "cf" => DnsProviderType::Cloudflare,
        "gcloud" | "googlecloud" | "gcp" => DnsProviderType::GoogleCloud,
        "duckdns" => DnsProviderType::DuckDNS,
        "porkbun" => DnsProviderType::Porkbun,
        _ => anyhow::bail!("Invalid DNS provider: {}", domain_entry.provider),
    };

    let provider = DnsProvider {
        provider_type,
        api_token: domain_entry.api_token.clone(),
    };

    match record.record_type {
        RecordType::A => set_a_record(&provider, &domain_entry.domain, &record.value)?,
        RecordType::TXT => set_txt_record(&provider, &domain_entry.domain, &record.value)?,
        RecordType::AAAA => {
            // TODO: Implement AAAA record support
            anyhow::bail!("AAAA records not yet implemented");
        }
        RecordType::SSHFP => {
            // TODO: Implement SSHFP record support
            anyhow::bail!("SSHFP records not yet implemented");
        }
    }

    Ok(())
}

/// Apply all records for a domain to the DNS provider
#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
pub fn apply_all_records(
    domain_entry: &DomainEntry,
) -> Result<Vec<(RecordType, String, Result<()>)>> {
    let mut results = Vec::new();

    for record in &domain_entry.records {
        let result = apply_record(domain_entry, record);
        results.push((record.record_type.clone(), record.value.clone(), result));
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_type_conversion() {
        assert_eq!(RecordType::from_str("a"), Some(RecordType::A));
        assert_eq!(RecordType::from_str("A"), Some(RecordType::A));
        assert_eq!(RecordType::from_str("aaaa"), Some(RecordType::AAAA));
        assert_eq!(RecordType::from_str("txt"), Some(RecordType::TXT));
        assert_eq!(RecordType::from_str("sshfp"), Some(RecordType::SSHFP));
        assert_eq!(RecordType::from_str("invalid"), None);
    }

    #[test]
    fn test_ns_config_add_domain() {
        let mut config = NsConfig::default();
        config
            .add_domain(
                "example.com".to_string(),
                "cloudflare".to_string(),
                "token123".to_string(),
            )
            .unwrap();

        assert_eq!(config.domains.len(), 1);
        assert_eq!(config.domains[0].domain, "example.com");
    }

    #[test]
    fn test_ns_config_add_record() {
        let mut config = NsConfig::default();
        config
            .add_domain(
                "example.com".to_string(),
                "cloudflare".to_string(),
                "token123".to_string(),
            )
            .unwrap();
        config
            .add_record("example.com", RecordType::A, "1.2.3.4".to_string())
            .unwrap();

        let domain = config.get_domain("example.com").unwrap();
        assert_eq!(domain.records.len(), 1);
        assert_eq!(domain.records[0].record_type, RecordType::A);
        assert_eq!(domain.records[0].value, "1.2.3.4");
    }

    #[test]
    fn test_ns_config_yaml_serialization() {
        let mut config = NsConfig::default();
        config
            .add_domain(
                "example.com".to_string(),
                "cloudflare".to_string(),
                "token123".to_string(),
            )
            .unwrap();
        config
            .add_record("example.com", RecordType::A, "1.2.3.4".to_string())
            .unwrap();

        let yaml = config.to_yaml().unwrap();
        assert!(yaml.contains("example.com"));
        assert!(yaml.contains("cloudflare"));

        let loaded = NsConfig::from_yaml(&yaml).unwrap();
        assert_eq!(loaded.domains.len(), 1);
        assert_eq!(loaded.domains[0].domain, "example.com");
    }
}
