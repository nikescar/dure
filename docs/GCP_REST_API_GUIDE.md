# GCP Compute Engine REST API with ureq

## Why Direct REST API Implementation?

Instead of using `google-cloud-rust` or `google-apis-rs` (which require tokio/hyper), we implement the GCP Compute Engine REST API directly using `ureq`. This:

✅ **Avoids async dependencies** - Matches your synchronous architecture  
✅ **Lightweight** - Only implement what you need  
✅ **Full control** - No hidden complexity  
✅ **Simple** - REST API is straightforward  

## Architecture

```
┌─────────────────────────────────────────┐
│         GCP Wizard (egui UI)            │
│    mobile/src/ui_dlg/platform_gcp.rs    │
└─────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────┐
│      GCP Client Wrapper (optional)      │
│       mobile/src/calc/gcp.rs            │
└─────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────┐
│        GCP REST API Client              │
│     mobile/src/calc/gcp_rest.rs         │
│  (Direct REST calls using ureq)         │
└─────────────────────────────────────────┘
                    ↓
          GCP Compute Engine API
    https://compute.googleapis.com/compute/v1
```

## Usage Example

### 1. Get Access Token from OAuth

```rust
use crate::api::gcp_oauth::{OAuthHandler, refresh_access_token};

// During OAuth flow
let oauth_result = handler.run_oauth_flow()?;
let access_token = oauth_result.access_token;

// Or refresh later
let oauth_result = refresh_access_token(
    &client_id,
    &client_secret,
    &refresh_token
)?;
```

### 2. Create GCP REST Client

```rust
use crate::calc::gcp_rest::{GcpRestClient, InstanceRequest};

let client = GcpRestClient::new(access_token);
```

### 3. Create a VM Instance

```rust
// Simple Debian micro instance
let instance_req = InstanceRequest::debian_micro(
    "my-server".to_string(),
    "us-central1-a".to_string(),
);

// Create the instance
let operation = client.create_instance(
    "my-project-id",
    "us-central1-a",
    &instance_req,
)?;

println!("Operation started: {}", operation.name);

// Wait for completion
let result = client.wait_for_operation(
    "my-project-id",
    "us-central1-a",
    &operation.name,
    300, // 5 minute timeout
)?;

if result.status == "DONE" {
    println!("✓ Instance created successfully!");
}
```

### 4. List Instances

```rust
let instances = client.list_instances("my-project-id", "us-central1-a")?;

for instance in instances.items {
    println!("Instance: {} ({})", instance.name, instance.status);
    if let Some(ip) = instance.external_ip() {
        println!("  External IP: {}", ip);
    }
}
```

### 5. Get Instance Details

```rust
let instance = client.get_instance(
    "my-project-id",
    "us-central1-a",
    "my-server",
)?;

println!("Status: {}", instance.status);
println!("External IP: {:?}", instance.external_ip());
println!("Internal IP: {:?}", instance.internal_ip());
```

### 6. Delete Instance

```rust
let operation = client.delete_instance(
    "my-project-id",
    "us-central1-a",
    "my-server",
)?;

// Wait for deletion to complete
client.wait_for_operation(
    "my-project-id",
    "us-central1-a",
    &operation.name,
    300,
)?;

println!("✓ Instance deleted");
```

## Integration with Existing Code

Update `mobile/src/calc/gcp.rs` to use the REST client:

```rust
use super::gcp_rest::{GcpRestClient, InstanceRequest};

impl GcpClient {
    pub fn create_instance(&self, config: InstanceConfig) -> Result<Instance> {
        let rest_client = GcpRestClient::new(self.config.access_token.clone());

        // Convert our config to GCP REST request
        let instance_req = InstanceRequest {
            name: config.name.clone(),
            machine_type: format!("zones/{}/machineTypes/{}", config.zone, config.machine_type),
            // ... fill in other fields
        };

        // Create instance
        let operation = rest_client.create_instance(
            &self.config.project_id,
            &config.zone,
            &instance_req,
        )?;

        // Wait for completion
        let result = rest_client.wait_for_operation(
            &self.config.project_id,
            &config.zone,
            &operation.name,
            600, // 10 minute timeout
        )?;

        if let Some(error) = result.error {
            return Err(anyhow::anyhow!(
                "Instance creation failed: {:?}",
                error.errors
            ));
        }

        // Get instance details
        let instance = rest_client.get_instance(
            &self.config.project_id,
            &config.zone,
            &config.name,
        )?;

        // Convert to our Instance type
        Ok(Instance {
            id: instance.id,
            name: instance.name,
            machine_type: instance.machine_type,
            zone: instance.zone,
            status: instance.status,
            external_ip: instance.external_ip(),
            internal_ip: instance.internal_ip(),
            creation_timestamp: "".to_string(), // Not in basic response
        })
    }
}
```

## API Reference

All GCP Compute Engine REST API documentation:
https://cloud.google.com/compute/docs/reference/rest/v1

### Common Endpoints

| Operation | Method | Endpoint |
|-----------|--------|----------|
| List instances | GET | `/projects/{project}/zones/{zone}/instances` |
| Get instance | GET | `/projects/{project}/zones/{zone}/instances/{instance}` |
| Create instance | POST | `/projects/{project}/zones/{zone}/instances` |
| Delete instance | DELETE | `/projects/{project}/zones/{zone}/instances/{instance}` |
| List regions | GET | `/projects/{project}/regions` |
| List zones | GET | `/projects/{project}/zones` |
| Get operation | GET | `/projects/{project}/zones/{zone}/operations/{operation}` |

### Authentication

All requests need `Authorization: Bearer {access_token}` header.

Get access token from:
1. OAuth flow (initial)
2. Refresh token (subsequent calls)

Token expires after ~1 hour - refresh when needed.

## Error Handling

```rust
match client.create_instance(project, zone, &instance) {
    Ok(operation) => {
        // Wait for operation
        match client.wait_for_operation(project, zone, &operation.name, 600) {
            Ok(result) => {
                if result.status == "DONE" {
                    if let Some(error) = result.error {
                        eprintln!("Operation failed: {:?}", error);
                    } else {
                        println!("Success!");
                    }
                }
            }
            Err(e) => eprintln!("Failed to wait for operation: {}", e),
        }
    }
    Err(e) => eprintln!("Failed to create instance: {}", e),
}
```

## Machine Types

Common machine types:
- `e2-micro` - 0.25-2 vCPU, 1 GB (free tier)
- `e2-small` - 0.5-2 vCPU, 2 GB
- `e2-medium` - 1-2 vCPU, 4 GB
- `n1-standard-1` - 1 vCPU, 3.75 GB
- `n2-standard-2` - 2 vCPU, 8 GB

Format: `zones/{zone}/machineTypes/{type}`

## Images

Common boot images:
- Debian 11: `projects/debian-cloud/global/images/family/debian-11`
- Ubuntu 22.04: `projects/ubuntu-os-cloud/global/images/family/ubuntu-2204-lts`
- Container-Optimized OS: `projects/cos-cloud/global/images/family/cos-stable`

## Next Steps

1. ✅ OAuth flow already works (`mobile/src/api/gcp_oauth.rs`)
2. ✅ REST client created (`mobile/src/calc/gcp_rest.rs`)
3. ⏳ Update `mobile/src/calc/gcp.rs` to use REST client
4. ⏳ Test create/list/delete operations
5. ⏳ Add firewall rules support
6. ⏳ Add static IP support

## No Async Dependencies Needed!

```toml
# Cargo.toml - NO tokio, NO hyper, NO async!
[dependencies]
ureq = { version = "2.12", features = ["tls", "json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

That's it! 🎉
