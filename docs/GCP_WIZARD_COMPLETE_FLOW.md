# GCP Wizard - Complete Implementation

## Overview

The GCP wizard now implements the **complete flow** using real GCP REST API calls:

1. ✅ **OAuth Authentication** - User authorizes app
2. ✅ **Project & Billing Selection** - Lists actual projects and billing accounts from API
3. ✅ **Server Configuration** - Lists actual regions/zones from API
4. ✅ **VM Creation** - Actually creates the instance via API
5. ✅ **Completion** - Verifies creation and shows instance details

## Wizard Flow

```
┌─────────────────────────────────────────┐
│  1. Connect Account (OAuth)             │
│  - Opens browser                        │
│  - User authorizes                      │
│  - Gets access token                    │
│  - Stores refresh token in keyring     │
└─────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────┐
│  2. Select Project & Billing            │
│  API: list_projects()                   │
│  API: list_billing_accounts()           │
│  API: get_project_billing_info()        │
│  - Shows dropdown of actual projects    │
│  - Shows dropdown of billing accounts   │
│  - Validates billing is enabled         │
└─────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────┐
│  3. Configure Server                    │
│  API: list_regions()                    │
│  - Shows dropdown of actual regions     │
│  - Shows dropdown of zones              │
│  - Select machine type                  │
│  - Enter instance name                  │
└─────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────┐
│  4. Creating Server                     │
│  API: create_instance()                 │
│  API: wait_for_operation()              │
│  - Sends creation request               │
│  - Polls operation status               │
│  - Waits up to 10 minutes               │
│  - Shows progress                       │
└─────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────┐
│  5. Complete                            │
│  API: get_instance()                    │
│  - Shows instance details               │
│  - Displays external IP                 │
│  - Shows RUNNING status                 │
│  - Link to GCP Console                  │
└─────────────────────────────────────────┘
```

## API Endpoints Used

### Step 1: OAuth (already implemented)
- `https://accounts.google.com/o/oauth2/v2/auth`
- `https://oauth2.googleapis.com/token`

### Step 2: Projects & Billing
```rust
// List all accessible projects
GET https://cloudresourcemanager.googleapis.com/v3/projects
→ Returns: ProjectList { projects: Vec<Project> }

// List billing accounts
GET https://cloudbilling.googleapis.com/v1/billingAccounts
→ Returns: BillingAccountList { billing_accounts: Vec<BillingAccount> }

// Check if project has billing enabled
GET https://cloudbilling.googleapis.com/v1/projects/{projectId}/billingInfo
→ Returns: ProjectBillingInfo { billing_enabled: bool, billing_account_name: String }
```

### Step 3: Configuration
```rust
// List regions for a project
GET https://compute.googleapis.com/compute/v1/projects/{project}/regions
→ Returns: RegionList { items: Vec<Region> }
```

### Step 4 & 5: VM Creation
```rust
// Create VM instance
POST https://compute.googleapis.com/compute/v1/projects/{project}/zones/{zone}/instances
Body: InstanceRequest { name, machine_type, disks, network_interfaces, ... }
→ Returns: Operation { name, status, ... }

// Wait for operation to complete (polling)
GET https://compute.googleapis.com/compute/v1/projects/{project}/zones/{zone}/operations/{operation}
→ Returns: Operation { status: "DONE", error: Option<...> }

// Get instance details
GET https://compute.googleapis.com/compute/v1/projects/{project}/zones/{zone}/instances/{instance}
→ Returns: Instance { status: "RUNNING", network_interfaces: [...], ... }
```

## UI Features

### Step 2: Select Project & Billing

**Project Dropdown:**
```
┌─────────────────────────────────────────┐
│ Project: [My Production (prod-123)  ▼] │
│          My Development (dev-456)       │
│          Customer Portal (portal-789)   │
└─────────────────────────────────────────┘
```

**Billing Dropdown:**
```
┌─────────────────────────────────────────┐
│ Billing: [✓ Main Account (012345)   ▼] │
│          ✓ Dev Account (678901)         │
│          ✗ Closed Old Account (111111)  │
└─────────────────────────────────────────┘
```

**Billing Status Indicator:**
```
✓ Billing enabled: billingAccounts/012345-ABCDEF-678901
```
or
```
⚠ Billing not enabled for this project
```

### Step 3: Configure Server

**Region Dropdown:**
```
┌─────────────────────────────────────────┐
│ Region: [us-central1 (Iowa, USA)    ▼] │
│         us-east1 (South Carolina)       │
│         asia-northeast3 (Seoul)         │
└─────────────────────────────────────────┘
```

**Zone Dropdown** (filtered by selected region):
```
┌─────────────────────────────────────────┐
│ Zone: [us-central1-a                 ▼] │
│       us-central1-b                     │
│       us-central1-c                     │
└─────────────────────────────────────────┘
```

**Machine Type:**
```
┌─────────────────────────────────────────┐
│ Machine: [e2-micro - 0.25-2 vCPU... ▼]  │
│          e2-small - 0.5-2 vCPU, 2 GB    │
│          n1-standard-1 - 1 vCPU, 3.75GB │
└─────────────────────────────────────────┘
```

### Step 5: Complete

**Success Screen:**
```
┌──────────────────────────────────────────────┐
│ ✓ Instance is RUNNING and ready to use!     │
│                                              │
│ ┌──────────────────────────────────────────┐ │
│ │ 📦 Instance Name: my-server              │ │
│ │ 🆔 Instance ID: 123456789                │ │
│ │ ⚙️  Machine Type: e2-micro               │ │
│ │ 📍 Zone: us-central1-a                   │ │
│ │ ────────────────────────────────────────  │ │
│ │ 📊 Status: RUNNING                       │ │
│ │ 🌐 External IP: 34.123.45.67             │ │
│ │ 💻 SSH: ssh user@34.123.45.67            │ │
│ │ 🔒 Internal IP: 10.0.0.2                 │ │
│ └──────────────────────────────────────────┘ │
│                                              │
│ 📋 Next steps:                               │
│   2. 🔑 Configure SSH access                 │
│   3. 🔐 Set up firewall rules if needed      │
│   4. 📦 Install your application             │
│                                              │
│ 💡 View in GCP Console:                      │
│ https://console.cloud.google.com/compute/... │
└──────────────────────────────────────────────┘
```

## Code Structure

### REST Client (`mobile/src/calc/gcp_rest.rs`)
```rust
impl GcpRestClient {
    // Projects
    pub fn list_projects(&self) -> Result<ProjectList>
    pub fn get_project(&self, project_id: &str) -> Result<Project>

    // Billing
    pub fn list_billing_accounts(&self) -> Result<BillingAccountList>
    pub fn get_project_billing_info(&self, project_id: &str) -> Result<ProjectBillingInfo>

    // Compute
    pub fn list_regions(&self, project_id: &str) -> Result<RegionList>
    pub fn create_instance(&self, project_id: &str, zone: &str, instance: &InstanceRequest) -> Result<Operation>
    pub fn wait_for_operation(&self, project_id: &str, zone: &str, operation: &str, timeout: u64) -> Result<Operation>
    pub fn get_instance(&self, project_id: &str, zone: &str, instance: &str) -> Result<Instance>
}
```

### Wizard State Machine (`mobile/src/ui_dlg/platform_gcp.rs`)
```rust
pub enum WizardState {
    ConnectAccount,     // OAuth flow
    SelectProject,      // Project & billing selection
    ConfigureServer,    // Region, zone, machine type
    CreatingServer,     // VM creation in progress
    Complete,           // Success, show details
    Error(String),      // Error state
}

impl GcpWizard {
    fn load_projects_and_billing(&mut self)  // Called after OAuth
    fn load_regions(&mut self)                // Called before configure
    fn start_server_creation(&mut self)       // Creates VM in background thread
}
```

## Error Handling

The wizard handles errors gracefully:

1. **OAuth fails**: Shows error, allows retry
2. **No projects**: Shows empty list with message
3. **Billing not enabled**: Shows warning, allows selection anyway
4. **Region loading fails**: Falls back to static list
5. **VM creation fails**: Shows error details, allows back navigation
6. **Operation timeout**: Cancels after 10 minutes with error

## Testing the Flow

### Step-by-Step Test

1. **Launch app:**
   ```bash
   cargo run -- --gui
   ```

2. **Navigate to Platform tab**

3. **Add or select a GCP platform**

4. **Click "Init Server"**

5. **OAuth (Step 1):**
   - Browser opens
   - Login to Google
   - Authorize app
   - Browser shows success
   - Return to app

6. **Select Project (Step 2):**
   - See your actual projects in dropdown
   - See your billing accounts in dropdown
   - Select both
   - See billing status check
   - Click "Next"

7. **Configure (Step 3):**
   - See actual regions from your project
   - See zones for selected region
   - Choose machine type
   - Enter instance name
   - Click "Create Server"

8. **Creating (Step 4):**
   - Watch progress spinner
   - See progress logs
   - Wait for completion (2-5 minutes)

9. **Complete (Step 5):**
   - See instance details
   - See RUNNING status
   - See external IP
   - Click link to view in GCP Console

## Required Permissions

The OAuth token needs these scopes (already configured):
- `userinfo.email` - For user identification
- `compute` - For VM operations
- `cloudplatformprojects` - For listing projects
- `cloud-billing` - For billing info

## Next Enhancements

Possible future additions:
- [ ] Firewall rule creation during setup
- [ ] SSH key injection during creation
- [ ] Custom disk size selection
- [ ] Multiple network interface support
- [ ] Startup script injection
- [ ] Label/tag assignment
- [ ] Region/zone recommendation based on latency

## Summary

✅ **Complete wizard implementation using real GCP REST APIs**  
✅ **No async dependencies - all synchronous with ureq**  
✅ **Full OAuth → Project Selection → VM Creation flow**  
✅ **Proper error handling and user feedback**  
✅ **Beautiful Material Design 3 UI**  

The wizard is now **production-ready** for creating GCP VMs! 🎉
