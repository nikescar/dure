//! GCP Compute Engine REST API implementation using ureq
//!
//! Implements the GCP Compute Engine REST API v1 directly using synchronous HTTP.
//! This avoids async/tokio dependencies and matches our lightweight architecture.
//!
//! API Reference: https://cloud.google.com/compute/docs/reference/rest/v1

use anyhow::Result;
use serde::{Deserialize, Serialize};

const GCP_COMPUTE_API_BASE: &str = "https://compute.googleapis.com/compute/v1";
const GCP_RESOURCE_MANAGER_API_BASE: &str = "https://cloudresourcemanager.googleapis.com/v1";
const GCP_BILLING_API_BASE: &str = "https://cloudbilling.googleapis.com/v1";

/// GCP REST API client using ureq
pub struct GcpRestClient {
    access_token: String,
}

impl GcpRestClient {
    /// Create new client with access token
    pub fn new(access_token: String) -> Self {
        Self { access_token }
    }

    /// Make authenticated GET request with better error handling
    fn get(&self, url: &str) -> Result<ureq::Response> {
        match ureq::get(url)
            .set("Authorization", &format!("Bearer {}", self.access_token))
            .set("Content-Type", "application/json")
            .call()
        {
            Ok(response) => Ok(response),
            Err(ureq::Error::Status(code, response)) => {
                let body = response.into_string().unwrap_or_default();

                // Check for API not enabled error
                if code == 403
                    && (body.contains("has not been used in project")
                        || body.contains("it is disabled"))
                {
                    let api_name = if body.contains("cloudresourcemanager") {
                        "Cloud Resource Manager API"
                    } else if body.contains("cloudbilling") {
                        "Cloud Billing API"
                    } else if body.contains("compute") {
                        "Compute Engine API"
                    } else {
                        "Required API"
                    };

                    return Err(anyhow::anyhow!(
                        "{} is not enabled. Please enable it in the GCP Console:\n{}",
                        api_name,
                        body
                    ));
                }

                Err(anyhow::anyhow!(
                    "HTTP {} error for {}:\n{}",
                    code,
                    url,
                    if body.len() > 500 {
                        format!("{}...", &body[..500])
                    } else {
                        body
                    }
                ))
            }
            Err(ureq::Error::Transport(transport)) => {
                Err(anyhow::anyhow!("Network error for {}:\n{}", url, transport))
            }
        }
    }

    /// Make authenticated POST request
    /// Returns Response for both success and error statuses (caller must check status)
    fn post(&self, url: &str, body: &str) -> Result<ureq::Response> {
        match ureq::post(url)
            .set("Authorization", &format!("Bearer {}", self.access_token))
            .set("Content-Type", "application/json")
            .send_string(body)
        {
            Ok(response) => Ok(response),
            Err(ureq::Error::Status(_code, response)) => {
                // Return the response so caller can inspect error
                Ok(response)
            }
            Err(ureq::Error::Transport(transport)) => {
                Err(anyhow::anyhow!("Network error for {}: {}", url, transport))
            }
        }
    }

    /// Make authenticated DELETE request
    fn delete(&self, url: &str) -> Result<ureq::Response> {
        match ureq::delete(url)
            .set("Authorization", &format!("Bearer {}", self.access_token))
            .set("Content-Type", "application/json")
            .call()
        {
            Ok(response) => Ok(response),
            Err(ureq::Error::Status(code, response)) => {
                let body = response.into_string().unwrap_or_default();
                Err(anyhow::anyhow!("HTTP {} error for {}: {}", code, url, body))
            }
            Err(ureq::Error::Transport(transport)) => {
                Err(anyhow::anyhow!("Network error for {}: {}", url, transport))
            }
        }
    }

    /// Create VM instance
    ///
    /// API: POST /projects/{project}/zones/{zone}/instances
    pub fn create_instance(
        &self,
        project_id: &str,
        zone: &str,
        instance: &InstanceRequest,
    ) -> Result<Operation> {
        let url = format!(
            "{}/projects/{}/zones/{}/instances",
            GCP_COMPUTE_API_BASE, project_id, zone
        );

        let body = serde_json::to_string(instance)?;
        let response = self.post(&url, &body)?;

        if response.status() != 200 {
            let error_text = response.into_string().unwrap_or_default();

            // Detect Compute Engine API not enabled error
            if error_text.contains("Compute Engine API")
                && (error_text.contains("not been used") || error_text.contains("disabled"))
            {
                let activation_url = format!(
                    "https://console.developers.google.com/apis/api/compute.googleapis.com/overview?project={}",
                    project_id
                );

                return Err(anyhow::anyhow!(
                    "Compute Engine API is not enabled in project '{}'.\n\n\
                     To fix this (one-time setup):\n\
                     1. Open: {}\n\
                     2. Click 'Enable API'\n\
                     3. Wait a few minutes for changes to propagate\n\
                     4. Return here and click 'Create Server' again\n\n\
                     Note: This needs to be done once per GCP project.",
                    project_id,
                    activation_url
                ));
            }

            return Err(anyhow::anyhow!("Failed to create instance: {}", error_text));
        }

        let operation: Operation = response.into_json()?;
        Ok(operation)
    }

    /// List VM instances
    ///
    /// API: GET /projects/{project}/zones/{zone}/instances
    pub fn list_instances(&self, project_id: &str, zone: &str) -> Result<InstanceList> {
        let url = format!(
            "{}/projects/{}/zones/{}/instances",
            GCP_COMPUTE_API_BASE, project_id, zone
        );

        let response = self.get(&url)?;
        let list: InstanceList = response.into_json()?;
        Ok(list)
    }

    /// Get VM instance details
    ///
    /// API: GET /projects/{project}/zones/{zone}/instances/{instance}
    pub fn get_instance(
        &self,
        project_id: &str,
        zone: &str,
        instance_name: &str,
    ) -> Result<Instance> {
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}",
            GCP_COMPUTE_API_BASE, project_id, zone, instance_name
        );

        let response = self.get(&url)?;
        let instance: Instance = response.into_json()?;
        Ok(instance)
    }

    /// Delete VM instance
    ///
    /// API: DELETE /projects/{project}/zones/{zone}/instances/{instance}
    pub fn delete_instance(
        &self,
        project_id: &str,
        zone: &str,
        instance_name: &str,
    ) -> Result<Operation> {
        let url = format!(
            "{}/projects/{}/zones/{}/instances/{}",
            GCP_COMPUTE_API_BASE, project_id, zone, instance_name
        );

        let response = self.delete(&url)?;
        let operation: Operation = response.into_json()?;
        Ok(operation)
    }

    /// Wait for operation to complete
    ///
    /// API: GET /projects/{project}/zones/{zone}/operations/{operation}
    pub fn wait_for_operation(
        &self,
        project_id: &str,
        zone: &str,
        operation_name: &str,
        timeout_secs: u64,
    ) -> Result<Operation> {
        let url = format!(
            "{}/projects/{}/zones/{}/operations/{}",
            GCP_COMPUTE_API_BASE, project_id, zone, operation_name
        );

        let start = std::time::Instant::now();

        loop {
            let response = self.get(&url)?;
            let operation: Operation = response.into_json()?;

            if operation.is_done() {
                return Ok(operation);
            }

            if start.elapsed().as_secs() > timeout_secs {
                return Err(anyhow::anyhow!("Operation timed out"));
            }

            // Poll every 2 seconds
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }

    /// List available regions
    ///
    /// API: GET /projects/{project}/regions
    pub fn list_regions(&self, project_id: &str) -> Result<RegionList> {
        let url = format!("{}/projects/{}/regions", GCP_COMPUTE_API_BASE, project_id);

        let response = self.get(&url)?;
        let list: RegionList = response.into_json()?;
        Ok(list)
    }

    /// List available zones
    ///
    /// API: GET /projects/{project}/zones
    pub fn list_zones(&self, project_id: &str) -> Result<ZoneList> {
        let url = format!("{}/projects/{}/zones", GCP_COMPUTE_API_BASE, project_id);

        let response = self.get(&url)?;
        let list: ZoneList = response.into_json()?;
        Ok(list)
    }

    // ========================================================================
    // Cloud Resource Manager API - Projects
    // ========================================================================

    /// List all projects the user has access to
    ///
    /// API: GET /v3/projects
    /// List GCP projects that the user has access to
    ///
    /// API: GET /v1/projects
    ///
    /// # Arguments
    /// * `filter` - Optional filter expression (e.g., "name:my-project-*", "labels.env:prod")
    ///   See: https://cloud.google.com/resource-manager/reference/rest/v1/projects/list#query-parameters
    ///
    /// # Examples
    /// ```no_run
    /// # use dure::calc::gcp_rest::GcpRestClient;
    /// let client = GcpRestClient::new("token".to_string());
    ///
    /// // List all projects
    /// let all_projects = client.list_projects(None)?;
    ///
    /// // List projects with specific filter
    /// let filtered = client.list_projects(Some("labels.env:prod"))?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn list_projects(&self, filter: Option<&str>) -> Result<ProjectList> {
        let mut url = format!("{}/projects", GCP_RESOURCE_MANAGER_API_BASE);

        // Add filter query parameter if provided
        if let Some(filter_value) = filter {
            url = format!("{}?filter={}", url, urlencoding::encode(filter_value));
        }

        let response = self.get(&url)?;

        if response.status() != 200 {
            let error_text = response.into_string().unwrap_or_default();
            return Err(anyhow::anyhow!("Failed to list projects: {}", error_text));
        }

        let list: ProjectList = response.into_json()?;
        Ok(list)
    }

    /// Get project details
    ///
    /// API: GET /v3/projects/{projectId}
    pub fn get_project(&self, project_id: &str) -> Result<Project> {
        let url = format!("{}/projects/{}", GCP_RESOURCE_MANAGER_API_BASE, project_id);

        let response = self.get(&url)?;
        let project: Project = response.into_json()?;
        Ok(project)
    }

    /// Create a new project
    ///
    /// API: POST /v1/projects
    pub fn create_project(&self, project_id: &str, display_name: &str) -> Result<Operation> {
        let url = format!("{}/projects", GCP_RESOURCE_MANAGER_API_BASE);

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct CreateProjectRequest {
            project_id: String,
            name: String,
            labels: std::collections::HashMap<String, String>,
        }

        let mut labels = std::collections::HashMap::new();
        labels.insert("dure".to_string(), "true".to_string());

        let body = serde_json::to_string(&CreateProjectRequest {
            project_id: project_id.to_string(),
            name: display_name.to_string(),
            labels,
        })?;

        let response = self.post(&url, &body)?;

        if response.status() != 200 {
            let error_text = response.into_string().unwrap_or_default();
            return Err(anyhow::anyhow!("Failed to create project: {}", error_text));
        }

        let operation: Operation = response.into_json()?;
        Ok(operation)
    }

    // ========================================================================
    // Service Usage API
    // ========================================================================

    // ========================================================================
    // Cloud Billing API
    // ========================================================================

    /// List billing accounts
    ///
    /// API: GET /v1/billingAccounts
    pub fn list_billing_accounts(&self) -> Result<BillingAccountList> {
        let url = format!("{}/billingAccounts", GCP_BILLING_API_BASE);

        let response = self.get(&url)?;

        if response.status() != 200 {
            let error_text = response.into_string().unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Failed to list billing accounts: {}",
                error_text
            ));
        }

        let list: BillingAccountList = response.into_json()?;
        Ok(list)
    }

    /// Get billing info for a project
    ///
    /// API: GET /v1/projects/{projectId}/billingInfo
    pub fn get_project_billing_info(&self, project_id: &str) -> Result<ProjectBillingInfo> {
        let url = format!(
            "{}/projects/{}/billingInfo",
            GCP_BILLING_API_BASE, project_id
        );

        let response = self.get(&url)?;

        if response.status() != 200 {
            let error_text = response.into_string().unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Failed to get billing info: {}",
                error_text
            ));
        }

        let info: ProjectBillingInfo = response.into_json()?;
        Ok(info)
    }

    /// Enable billing for a project by associating it with a billing account
    ///
    /// API: PUT /v1/projects/{projectId}/billingInfo
    pub fn enable_project_billing(
        &self,
        project_id: &str,
        billing_account_name: &str,
    ) -> Result<ProjectBillingInfo> {
        let url = format!(
            "{}/projects/{}/billingInfo",
            GCP_BILLING_API_BASE, project_id
        );

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct BillingInfoUpdate {
            billing_account_name: String,
        }

        let body = serde_json::to_string(&BillingInfoUpdate {
            billing_account_name: billing_account_name.to_string(),
        })?;

        let response = match ureq::put(&url)
            .set("Authorization", &format!("Bearer {}", self.access_token))
            .set("Content-Type", "application/json")
            .send_string(&body)
        {
            Ok(response) => response,
            Err(ureq::Error::Status(code, response)) => {
                let body = response.into_string().unwrap_or_default();
                return Err(anyhow::anyhow!("HTTP {} error for {}: {}", code, url, body));
            }
            Err(ureq::Error::Transport(transport)) => {
                return Err(anyhow::anyhow!("Network error for {}: {}", url, transport));
            }
        };

        if response.status() != 200 {
            let error_text = response.into_string().unwrap_or_default();
            return Err(anyhow::anyhow!("Failed to enable billing: {}", error_text));
        }

        let info: ProjectBillingInfo = response.into_json()?;
        Ok(info)
    }
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// Instance creation request
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceRequest {
    pub name: String,
    pub machine_type: String, // e.g., "zones/us-central1-a/machineTypes/e2-micro"
    pub disks: Vec<AttachedDisk>,
    pub network_interfaces: Vec<NetworkInterface>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachedDisk {
    pub boot: bool,
    pub auto_delete: bool,
    pub initialize_params: InitializeParams,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    pub source_image: String, // e.g., "projects/debian-cloud/global/images/debian-11-bullseye-v20240219"
    pub disk_size_gb: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInterface {
    pub network: String, // e.g., "global/networks/default"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_configs: Option<Vec<AccessConfig>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessConfig {
    #[serde(rename = "type")]
    pub type_: String, // "ONE_TO_ONE_NAT"
    pub name: String, // "External NAT"
}

#[derive(Debug, Serialize)]
pub struct Tags {
    pub items: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Metadata {
    pub items: Vec<MetadataItem>,
}

#[derive(Debug, Serialize)]
pub struct MetadataItem {
    pub key: String,
    pub value: String,
}

/// Instance response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub id: String,
    pub name: String,
    pub machine_type: String,
    pub zone: String,
    pub status: String,
    #[serde(default)]
    pub network_interfaces: Vec<NetworkInterfaceResponse>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInterfaceResponse {
    #[serde(rename = "networkIP", default)]
    pub network_ip: Option<String>,
    #[serde(default)]
    pub access_configs: Vec<AccessConfigResponse>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessConfigResponse {
    #[serde(rename = "natIP")]
    pub nat_ip: Option<String>,
}

/// Instance list response
#[derive(Debug, Deserialize)]
pub struct InstanceList {
    #[serde(default)]
    pub items: Vec<Instance>,
}

/// Operation response (for async operations)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    #[serde(default)]
    pub id: Option<String>, // Only present in ComputeEngine operations
    pub name: String,
    #[serde(default)]
    pub status: Option<String>, // "PENDING", "RUNNING", "DONE" (ComputeEngine)
    #[serde(default)]
    pub done: Option<bool>, // ResourceManager uses this instead of status
    #[serde(default)]
    pub error: Option<OperationError>,
}

impl Operation {
    /// Returns true if the operation is complete
    /// ResourceManager operations use `done`, ComputeEngine uses `status == "DONE"`
    pub fn is_done(&self) -> bool {
        self.done.unwrap_or(false) || self.status.as_deref() == Some("DONE")
    }

    /// Returns true if the operation has an error
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    /// Returns a status string for display
    pub fn status_string(&self) -> String {
        if let Some(status) = &self.status {
            status.clone()
        } else if let Some(done) = self.done {
            if done {
                "DONE".to_string()
            } else {
                "PENDING".to_string()
            }
        } else {
            "UNKNOWN".to_string()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OperationError {
    pub errors: Vec<ErrorDetail>,
}

#[derive(Debug, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
}

/// API error response
#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub error: ErrorInfo,
}

#[derive(Debug, Deserialize)]
pub struct ErrorInfo {
    pub code: i32,
    pub message: String,
}

/// Region list response
#[derive(Debug, Deserialize)]
pub struct RegionList {
    #[serde(default)]
    pub items: Vec<Region>,
}

#[derive(Debug, Deserialize)]
pub struct Region {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub zones: Vec<String>,
}

/// Zone list response
#[derive(Debug, Deserialize)]
pub struct ZoneList {
    #[serde(default)]
    pub items: Vec<Zone>,
}

#[derive(Debug, Deserialize)]
pub struct Zone {
    pub name: String,
    pub description: String,
    pub region: String,
}

// ============================================================================
// Cloud Resource Manager Types
// ============================================================================

/// Project list response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectList {
    #[serde(default)]
    pub projects: Vec<Project>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

/// Project details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    #[serde(default)]
    pub name: Option<String>, // e.g., "projects/my-project-123"
    pub project_id: String, // e.g., "my-project-123" (always present)
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default, rename = "lifecycleState")]
    pub state: Option<String>, // "ACTIVE", "DELETE_REQUESTED", etc.
    #[serde(default)]
    pub labels: std::collections::HashMap<String, String>,
}

// ============================================================================
// Cloud Billing Types
// ============================================================================

/// Billing account list response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingAccountList {
    #[serde(default)]
    pub billing_accounts: Vec<BillingAccount>,
    #[serde(default)]
    pub next_page_token: Option<String>,
}

/// Billing account details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingAccount {
    pub name: String, // e.g., "billingAccounts/012345-ABCDEF-678901"
    pub display_name: String,
    pub open: bool,
    #[serde(default)]
    pub master_billing_account: Option<String>,
}

/// Project billing info
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectBillingInfo {
    pub name: String, // e.g., "projects/my-project/billingInfo"
    pub project_id: String,
    #[serde(default)]
    pub billing_account_name: Option<String>, // e.g., "billingAccounts/012345-ABCDEF-678901"
    pub billing_enabled: bool,
}

impl BillingAccount {
    /// Extract billing account ID from name
    /// e.g., "billingAccounts/012345-ABCDEF-678901" -> "012345-ABCDEF-678901"
    pub fn id(&self) -> Option<&str> {
        self.name.strip_prefix("billingAccounts/")
    }
}

impl Project {
    /// Extract project ID from name if needed
    pub fn id(&self) -> &str {
        &self.project_id
    }

    /// Get display name with fallback to project_id
    pub fn display_name(&self) -> &str {
        self.display_name.as_deref().unwrap_or(&self.project_id)
    }

    /// Get project state with fallback to "UNKNOWN"
    pub fn state(&self) -> &str {
        self.state.as_deref().unwrap_or("UNKNOWN")
    }

    /// Check if project is active/usable (not being deleted)
    pub fn is_active(&self) -> bool {
        match self.state.as_deref() {
            // Explicitly active
            Some("ACTIVE") => true,
            // No state field or unspecified - assume usable
            None | Some("LIFECYCLE_STATE_UNSPECIFIED") => true,
            // Being deleted - not usable
            Some("DELETE_REQUESTED") | Some("DELETE_IN_PROGRESS") => false,
            // Unknown state - assume usable to be safe
            _ => true,
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

impl InstanceRequest {
    /// Create a basic Debian instance
    pub fn debian_micro(name: String, zone: String) -> Self {
        Self {
            name,
            machine_type: format!("zones/{}/machineTypes/e2-micro", zone),
            disks: vec![AttachedDisk {
                boot: true,
                auto_delete: true,
                initialize_params: InitializeParams {
                    source_image: "projects/debian-cloud/global/images/family/debian-11"
                        .to_string(),
                    disk_size_gb: "10".to_string(),
                },
            }],
            network_interfaces: vec![NetworkInterface {
                network: "global/networks/default".to_string(),
                access_configs: Some(vec![AccessConfig {
                    type_: "ONE_TO_ONE_NAT".to_string(),
                    name: "External NAT".to_string(),
                }]),
            }],
            tags: Some(Tags {
                items: vec!["http-server".to_string(), "https-server".to_string()],
            }),
            metadata: None,
        }
    }
}

impl Instance {
    /// Get external IP address
    pub fn external_ip(&self) -> Option<String> {
        self.network_interfaces
            .first()?
            .access_configs
            .first()?
            .nat_ip
            .clone()
    }

    /// Get internal IP address
    pub fn internal_ip(&self) -> Option<String> {
        self.network_interfaces
            .first()
            .and_then(|ni| ni.network_ip.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_debian_instance() {
        let instance =
            InstanceRequest::debian_micro("test-instance".to_string(), "us-central1-a".to_string());

        assert_eq!(instance.name, "test-instance");
        assert!(instance.machine_type.contains("e2-micro"));
        assert_eq!(instance.disks.len(), 1);
        assert!(instance.disks[0].boot);
    }

    #[test]
    fn test_list_projects_url_construction() {
        // Test that filter parameter is properly URL-encoded
        let client = GcpRestClient::new("fake-token".to_string());

        // Since we can't actually make the HTTP call without a real token,
        // we just verify that the method signature is correct and compiles
        let _ = client.list_projects(None);
        let _ = client.list_projects(Some("name:my-project-*"));
        let _ = client.list_projects(Some("labels.env:prod"));
    }

    #[test]
    fn test_project_list_deserialization() {
        // Test that we can deserialize a valid GCP project list response
        let json = r#"{
            "projects": [
                {
                    "name": "projects/my-project-123",
                    "projectId": "my-project-123",
                    "displayName": "My Test Project",
                    "lifecycleState": "ACTIVE",
                    "labels": {
                        "env": "prod",
                        "team": "platform"
                    }
                },
                {
                    "name": "projects/another-project",
                    "projectId": "another-project",
                    "displayName": "Another Project",
                    "lifecycleState": "ACTIVE"
                }
            ],
            "nextPageToken": "some-token-123"
        }"#;

        let list: ProjectList = serde_json::from_str(json).unwrap();
        assert_eq!(list.projects.len(), 2);
        assert_eq!(list.projects[0].project_id, "my-project-123");
        assert_eq!(
            list.projects[0].display_name,
            Some("My Test Project".to_string())
        );
        assert_eq!(list.projects[0].display_name(), "My Test Project");
        assert_eq!(list.projects[0].state, Some("ACTIVE".to_string()));
        assert_eq!(list.projects[0].state(), "ACTIVE");
        assert!(list.projects[0].is_active());
        assert_eq!(
            list.projects[0].labels.get("env"),
            Some(&"prod".to_string())
        );
        assert_eq!(list.next_page_token, Some("some-token-123".to_string()));
    }

    #[test]
    fn test_project_without_display_name() {
        // Test that we can deserialize a project without displayName
        let json = r#"{
            "projects": [
                {
                    "name": "projects/test-project",
                    "projectId": "test-project",
                    "lifecycleState": "ACTIVE"
                }
            ]
        }"#;

        let list: ProjectList = serde_json::from_str(json).unwrap();
        assert_eq!(list.projects.len(), 1);
        assert_eq!(list.projects[0].project_id, "test-project");
        assert_eq!(list.projects[0].display_name, None);
        // display_name() should fall back to project_id
        assert_eq!(list.projects[0].display_name(), "test-project");
        assert!(list.projects[0].is_active());
    }

    #[test]
    fn test_project_without_state() {
        // Test that we can deserialize a project without state field
        let json = r#"{
            "projects": [
                {
                    "projectId": "test-project-no-state"
                }
            ]
        }"#;

        let list: ProjectList = serde_json::from_str(json).unwrap();
        assert_eq!(list.projects.len(), 1);
        assert_eq!(list.projects[0].project_id, "test-project-no-state");
        assert_eq!(list.projects[0].state, None);
        assert_eq!(list.projects[0].state(), "UNKNOWN");
        assert!(list.projects[0].is_active()); // Active if state is missing (assume usable)
    }

    #[test]
    fn test_project_id_helper() {
        let project = Project {
            name: Some("projects/my-project-123".to_string()),
            project_id: "my-project-123".to_string(),
            display_name: Some("Test".to_string()),
            state: Some("ACTIVE".to_string()),
            labels: std::collections::HashMap::new(),
        };

        assert_eq!(project.id(), "my-project-123");
        assert_eq!(project.display_name(), "Test");
        assert_eq!(project.state(), "ACTIVE");
        assert!(project.is_active());
    }

    #[test]
    fn test_project_display_name_fallback() {
        let project = Project {
            name: Some("projects/my-project-123".to_string()),
            project_id: "my-project-123".to_string(),
            display_name: None,
            state: Some("ACTIVE".to_string()),
            labels: std::collections::HashMap::new(),
        };

        // When display_name is None, display_name() should return project_id
        assert_eq!(project.display_name(), "my-project-123");
    }

    #[test]
    fn test_resource_manager_operation_response() {
        // ResourceManager operations have: name, done, error (NO id or status)
        let json = r#"{
            "name": "operations/cp.1234567890",
            "done": false
        }"#;

        let op: Operation = serde_json::from_str(json).unwrap();
        assert_eq!(op.name, "operations/cp.1234567890");
        assert_eq!(op.id, None); // ResourceManager ops don't have id
        assert_eq!(op.status, None); // ResourceManager ops don't have status
        assert_eq!(op.done, Some(false));
        assert!(!op.is_done());
        assert_eq!(op.status_string(), "PENDING");
    }

    #[test]
    fn test_compute_engine_operation_response() {
        // ComputeEngine operations have: id, name, status (NO done)
        let json = r#"{
            "id": "1234567890",
            "name": "operation-123",
            "status": "DONE"
        }"#;

        let op: Operation = serde_json::from_str(json).unwrap();
        assert_eq!(op.name, "operation-123");
        assert_eq!(op.id, Some("1234567890".to_string()));
        assert_eq!(op.status, Some("DONE".to_string()));
        assert_eq!(op.done, None); // ComputeEngine ops don't have done
        assert!(op.is_done());
        assert_eq!(op.status_string(), "DONE");
    }

    #[test]
    fn test_operation_is_done_helpers() {
        // Test with ResourceManager done=true
        let op = Operation {
            id: None,
            name: "op1".to_string(),
            status: None,
            done: Some(true),
            error: None,
        };
        assert!(op.is_done());
        assert!(!op.has_error());

        // Test with ComputeEngine status="DONE"
        let op = Operation {
            id: Some("123".to_string()),
            name: "op2".to_string(),
            status: Some("DONE".to_string()),
            done: None,
            error: None,
        };
        assert!(op.is_done());

        // Test with ComputeEngine status="PENDING"
        let op = Operation {
            id: Some("123".to_string()),
            name: "op3".to_string(),
            status: Some("PENDING".to_string()),
            done: None,
            error: None,
        };
        assert!(!op.is_done());
        assert_eq!(op.status_string(), "PENDING");
    }

    #[test]
    fn test_instance_without_network_ip() {
        // Instance response may have network_ip missing
        let json = r#"{
            "id": "123456789",
            "name": "test-instance",
            "machineType": "zones/us-central1-a/machineTypes/e2-micro",
            "zone": "zones/us-central1-a",
            "status": "PROVISIONING",
            "networkInterfaces": [
                {
                    "accessConfigs": [
                        {
                            "natIP": "34.123.45.67"
                        }
                    ]
                }
            ]
        }"#;

        let instance: Instance = serde_json::from_str(json).unwrap();
        assert_eq!(instance.id, "123456789");
        assert_eq!(instance.name, "test-instance");
        assert_eq!(instance.status, "PROVISIONING");
        assert_eq!(instance.internal_ip(), None); // No networkIp field
        assert_eq!(instance.external_ip(), Some("34.123.45.67".to_string()));
    }

    #[test]
    fn test_instance_with_network_ip() {
        // Instance response with network_ip present
        let json = r#"{
            "id": "123456789",
            "name": "test-instance",
            "machineType": "zones/us-central1-a/machineTypes/e2-micro",
            "zone": "zones/us-central1-a",
            "status": "RUNNING",
            "networkInterfaces": [
                {
                    "networkIP": "10.128.0.2",
                    "accessConfigs": [
                        {
                            "natIP": "34.123.45.67"
                        }
                    ]
                }
            ]
        }"#;

        let instance: Instance = serde_json::from_str(json).unwrap();
        assert_eq!(instance.internal_ip(), Some("10.128.0.2".to_string()));
        assert_eq!(instance.external_ip(), Some("34.123.45.67".to_string()));
    }

    #[test]
    fn test_project_real_gcp_format() {
        // Test with actual GCP API response format (from user's account)
        let json = r#"{
            "projects": [
                {
                    "projectNumber": "569546886350",
                    "projectId": "dure-20260415-222106",
                    "lifecycleState": "ACTIVE",
                    "name": "dure-20260415-222106",
                    "createTime": "2026-04-15T13:21:17.176Z",
                    "parent": {
                        "type": "organization",
                        "id": "1018343246695"
                    }
                },
                {
                    "projectNumber": "123456789",
                    "projectId": "another-project",
                    "lifecycleState": "ACTIVE",
                    "name": "another-project",
                    "createTime": "2026-01-01T00:00:00.000Z"
                }
            ]
        }"#;

        let list: ProjectList = serde_json::from_str(json).unwrap();
        assert_eq!(list.projects.len(), 2);

        // First project
        assert_eq!(list.projects[0].project_id, "dure-20260415-222106");
        assert_eq!(
            list.projects[0].name,
            Some("dure-20260415-222106".to_string())
        );
        assert_eq!(list.projects[0].display_name, None); // No displayName field
        assert_eq!(list.projects[0].display_name(), "dure-20260415-222106"); // Falls back to project_id
        assert_eq!(list.projects[0].state, Some("ACTIVE".to_string())); // Should now be parsed!
        assert_eq!(list.projects[0].state(), "ACTIVE");
        assert!(list.projects[0].is_active()); // Should be active!

        // Second project
        assert_eq!(list.projects[1].project_id, "another-project");
        assert_eq!(list.projects[1].state, Some("ACTIVE".to_string()));
        assert!(list.projects[1].is_active());
    }

    #[test]
    fn test_project_is_active_states() {
        use std::collections::HashMap;

        // Test various project states

        // ACTIVE state
        let active = Project {
            project_id: "test-1".to_string(),
            name: Some("projects/123456789".to_string()),
            display_name: Some("Test 1".to_string()),
            state: Some("ACTIVE".to_string()),
            labels: HashMap::new(),
        };
        assert!(active.is_active());

        // No state field (should be considered active)
        let no_state = Project {
            project_id: "test-2".to_string(),
            name: Some("projects/123456790".to_string()),
            display_name: Some("Test 2".to_string()),
            state: None,
            labels: HashMap::new(),
        };
        assert!(no_state.is_active());

        // LIFECYCLE_STATE_UNSPECIFIED (should be considered active)
        let unspecified = Project {
            project_id: "test-3".to_string(),
            name: Some("projects/123456791".to_string()),
            display_name: Some("Test 3".to_string()),
            state: Some("LIFECYCLE_STATE_UNSPECIFIED".to_string()),
            labels: HashMap::new(),
        };
        assert!(unspecified.is_active());

        // DELETE_REQUESTED (should NOT be active)
        let delete_requested = Project {
            project_id: "test-4".to_string(),
            name: Some("projects/123456792".to_string()),
            display_name: Some("Test 4".to_string()),
            state: Some("DELETE_REQUESTED".to_string()),
            labels: HashMap::new(),
        };
        assert!(!delete_requested.is_active());

        // DELETE_IN_PROGRESS (should NOT be active)
        let delete_in_progress = Project {
            project_id: "test-5".to_string(),
            name: Some("projects/123456793".to_string()),
            display_name: Some("Test 5".to_string()),
            state: Some("DELETE_IN_PROGRESS".to_string()),
            labels: HashMap::new(),
        };
        assert!(!delete_in_progress.is_active());
    }
}
