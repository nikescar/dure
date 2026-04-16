-- Create hosting table for domain registration, DNS, VM and service management
CREATE TABLE IF NOT EXISTS hosting (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    domain TEXT NOT NULL UNIQUE,
    status TEXT NOT NULL,

    domain_registrar TEXT,
    domain_registrar_token TEXT,
    domain_registered INTEGER NOT NULL DEFAULT 0,

    dns_provider TEXT NOT NULL,
    dns_provider_token TEXT,
    ns_addresses TEXT,
    dns_configured INTEGER NOT NULL DEFAULT 0,

    vm_provider TEXT NOT NULL,
    vm_provider_token TEXT,
    vm_instance_id TEXT,
    vm_ip_address TEXT,
    vm_ssh_user TEXT,
    vm_ssh_key_path TEXT,
    vm_created INTEGER NOT NULL DEFAULT 0,

    service_installed INTEGER NOT NULL DEFAULT 0,
    service_running INTEGER NOT NULL DEFAULT 0,

    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    error_message TEXT
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_hosting_domain ON hosting(domain);
CREATE INDEX IF NOT EXISTS idx_hosting_status ON hosting(status);
