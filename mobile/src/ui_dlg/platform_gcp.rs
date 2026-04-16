//! GCP Platform Wizard Dialog
//!
//! Multi-step wizard for GCP Compute Engine setup:
//! 1. Connect Account (OAuth)
//! 2. Configure Project (enter project ID)
//! 3. Configure Server (region, machine type, etc.)
//! 4. Create Server (VM instance creation)
//! 5. Complete (show connection info)

use eframe::egui;
use egui_material3::MaterialButton;
use poll_promise::Promise;
use serde::{Deserialize, Serialize};

use crate::api::gcp_oauth::{OAuthHandler, OAuthResult};
use crate::calc::gcp::{Instance, MachineType, Region, get_common_machine_types};
use crate::calc::gcp_rest::{GcpRestClient, InstanceRequest};
use crate::calc::keyring;
use crate::config::{AppConfig, CloudPlatformConfig};

/// GCP wizard state machine
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum WizardState {
    ConnectAccount,
    SelectProject,
    ConfigureServer,
    CreatingServer,
    Complete,
    Error(String),
}

/// GCP Platform Wizard
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GcpWizard {
    /// Current wizard state
    state: WizardState,

    /// Platform name (from parent)
    platform_name: String,

    /// OAuth result
    #[cfg_attr(feature = "serde", serde(skip))]
    oauth_result: Option<OAuthResult>,

    /// Selected project ID (user input, not loaded from API)
    selected_project_id: String,

    /// Selected region
    selected_region: String,

    /// Selected zone
    selected_zone: String,

    /// Selected machine type
    selected_machine_type: String,

    /// Instance name
    instance_name: String,

    /// Created instance
    #[cfg_attr(feature = "serde", serde(skip))]
    created_instance: Option<Instance>,

    /// Available regions (cached)
    #[cfg_attr(feature = "serde", serde(skip))]
    available_regions: Vec<Region>,

    /// Available machine types (cached)
    #[cfg_attr(feature = "serde", serde(skip))]
    available_machine_types: Vec<MachineType>,

    /// Available projects (loaded from GCP API)
    #[cfg_attr(feature = "serde", serde(skip))]
    available_projects: Vec<crate::calc::gcp_rest::Project>,

    /// Project loading state
    #[cfg_attr(feature = "serde", serde(skip))]
    projects_loaded: bool,

    /// Project loading error
    #[cfg_attr(feature = "serde", serde(skip))]
    projects_load_error: Option<String>,

    /// New project name for creation (when no projects exist)
    new_project_name: String,

    /// Whether user chose "Create New Project" option in combo box
    create_new_project_selected: bool,

    /// OAuth promise (for async OAuth flow)
    #[cfg_attr(feature = "serde", serde(skip))]
    oauth_promise: Option<Promise<Result<OAuthResult, String>>>,

    /// Create promise (for async VM creation)
    #[cfg_attr(feature = "serde", serde(skip))]
    create_promise: Option<Promise<Result<Instance, String>>>,

    /// Progress messages
    #[cfg_attr(feature = "serde", serde(skip))]
    progress_log: Vec<String>,

    /// Show wizard dialog
    show: bool,
}

impl Default for GcpWizard {
    fn default() -> Self {
        // Generate default project ID with timestamp
        let default_project_id = format!("dure-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S"));

        Self {
            state: WizardState::ConnectAccount,
            platform_name: String::new(),
            oauth_result: None,
            selected_project_id: default_project_id,
            selected_region: "us-central1".to_string(),
            selected_zone: "us-central1-a".to_string(),
            selected_machine_type: "e2-micro".to_string(),
            instance_name: "dure-server".to_string(),
            created_instance: None,
            available_regions: Vec::new(),
            available_machine_types: get_common_machine_types(),
            available_projects: Vec::new(),
            projects_loaded: false,
            projects_load_error: None,
            new_project_name: "Dure Server".to_string(),
            create_new_project_selected: false,
            oauth_promise: None,
            create_promise: None,
            progress_log: Vec::new(),
            show: false,
        }
    }
}

impl GcpWizard {
    /// Create new wizard for a platform
    pub fn new(platform_name: String) -> Self {
        Self {
            platform_name: platform_name.clone(),
            instance_name: format!("{}-server", platform_name.to_lowercase().replace(' ', "-")),
            ..Default::default()
        }
    }

    /// Load OAuth from config if exists
    pub fn load_oauth_from_config(&mut self, config: &AppConfig) {
        // Find GCP platform config
        if let Some(gcp_config) = config.platforms.iter().find(|p| p.platform_type == "gcp") {
            if let (Some(access_token), Some(refresh_token)) = (
                &gcp_config.gcp_oauth_access_token,
                &gcp_config.gcp_oauth_refresh_token,
            ) {
                // Check if token is expired
                let now = chrono::Utc::now().timestamp();
                let expired = gcp_config
                    .gcp_oauth_token_expiry
                    .map(|expiry| now >= expiry)
                    .unwrap_or(true);

                if !expired {
                    // Token still valid, use it
                    self.oauth_result = Some(OAuthResult {
                        access_token: access_token.clone(),
                        refresh_token: refresh_token.clone(),
                        expires_at: gcp_config
                            .gcp_oauth_token_expiry
                            .map(|exp| exp as u64)
                            .unwrap_or(chrono::Utc::now().timestamp() as u64 + 3600),
                    });
                    // Skip to project selection
                    self.state = WizardState::SelectProject;
                }
            }
        }
    }

    /// Save OAuth to config
    pub fn save_oauth_to_config(&self, config: &mut AppConfig) -> anyhow::Result<()> {
        if let Some(oauth) = &self.oauth_result {
            // Find or create GCP platform config
            let gcp_config = config
                .platforms
                .iter_mut()
                .find(|p| p.platform_type == "gcp")
                .cloned();

            let mut platform_config = gcp_config.unwrap_or_else(|| CloudPlatformConfig {
                name: "GCP".to_string(),
                platform_type: "gcp".to_string(),
                ..Default::default()
            });

            // Update OAuth fields
            platform_config.gcp_oauth_access_token = Some(oauth.access_token.clone());
            platform_config.gcp_oauth_refresh_token = Some(oauth.refresh_token.clone());
            platform_config.gcp_oauth_token_expiry = Some(oauth.expires_at as i64);

            // Update or add to platforms list
            if let Some(existing) = config
                .platforms
                .iter_mut()
                .find(|p| p.platform_type == "gcp")
            {
                *existing = platform_config;
            } else {
                config.platforms.push(platform_config);
            }
        }
        Ok(())
    }

    /// Clear OAuth from config
    pub fn clear_oauth_from_config(&self, config: &mut AppConfig) {
        if let Some(gcp_config) = config
            .platforms
            .iter_mut()
            .find(|p| p.platform_type == "gcp")
        {
            gcp_config.gcp_oauth_access_token = None;
            gcp_config.gcp_oauth_refresh_token = None;
            gcp_config.gcp_oauth_token_expiry = None;
        }
    }

    /// Show the wizard
    pub fn show(&mut self) {
        self.show = true;
        self.state = WizardState::ConnectAccount;
        self.progress_log.clear();
    }

    /// Hide the wizard
    pub fn hide(&mut self) {
        self.show = false;
    }

    /// Check if wizard is visible
    pub fn is_visible(&self) -> bool {
        self.show
    }

    /// Render the wizard UI
    pub fn ui(&mut self, ctx: &egui::Context) {
        if !self.show {
            return;
        }

        let mut open = true;

        egui::Window::new("GCP Server Setup")
            .open(&mut open)
            .resizable(true)
            .default_width(600.0)
            .default_height(500.0)
            .collapsible(false)
            .show(ctx, |ui| {
                // Progress indicator
                self.render_progress_indicator(ui);

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);

                // Render current step
                match self.state.clone() {
                    WizardState::ConnectAccount => self.render_connect_account(ui),
                    WizardState::SelectProject => self.render_select_project(ui),
                    WizardState::ConfigureServer => self.render_configure_server(ui),
                    WizardState::CreatingServer => self.render_creating_server(ui),
                    WizardState::Complete => self.render_complete(ui),
                    WizardState::Error(err) => self.render_error(ui, &err),
                }
            });

        if !open {
            self.hide();
        }
    }

    fn render_progress_indicator(&self, ui: &mut egui::Ui) {
        let steps = [
            ("Connect", WizardState::ConnectAccount),
            ("Project", WizardState::SelectProject),
            ("Configure", WizardState::ConfigureServer),
            ("Create", WizardState::CreatingServer),
            ("Complete", WizardState::Complete),
        ];

        ui.horizontal(|ui| {
            for (i, (label, step_state)) in steps.iter().enumerate() {
                if i > 0 {
                    ui.label("→");
                }

                let is_current =
                    std::mem::discriminant(&self.state) == std::mem::discriminant(step_state);
                let is_past = self.is_past_step(step_state);

                let color = if is_current {
                    egui::Color32::from_rgb(103, 126, 234) // Primary color
                } else if is_past {
                    egui::Color32::from_rgb(72, 187, 120) // Green
                } else {
                    egui::Color32::GRAY
                };

                ui.colored_label(
                    color,
                    if is_current {
                        format!("● {}", label)
                    } else {
                        label.to_string()
                    },
                );
            }
        });
    }

    fn is_past_step(&self, step: &WizardState) -> bool {
        use WizardState::*;
        match (&self.state, step) {
            (SelectProject, ConnectAccount) => true,
            (ConfigureServer, ConnectAccount | SelectProject) => true,
            (CreatingServer, ConnectAccount | SelectProject | ConfigureServer) => true,
            (Complete, ConnectAccount | SelectProject | ConfigureServer | CreatingServer) => true,
            _ => false,
        }
    }

    fn render_connect_account(&mut self, ui: &mut egui::Ui) {
        ui.heading("Connect to Google Cloud");
        ui.add_space(8.0);

        // Check if already connected (OAuth result exists)
        if self.oauth_result.is_some() {
            ui.colored_label(
                egui::Color32::from_rgb(72, 187, 120),
                "✓ Already connected to Google Cloud",
            );
            ui.add_space(8.0);

            ui.label("You can continue to project configuration or disconnect and reconnect.");
            ui.add_space(16.0);

            ui.horizontal(|ui| {
                if ui.add(MaterialButton::outlined("Disconnect")).clicked() {
                    // Clear OAuth from config
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        if let Ok((mut app_config, config_path)) = self.load_config_file() {
                            self.clear_oauth_from_config(&mut app_config);
                            if let Err(e) = app_config.save(&config_path) {
                                self.progress_log
                                    .push(format!("⚠ Failed to save config: {}", e));
                            } else {
                                self.progress_log
                                    .push("✓ OAuth cleared from config".to_string());
                            }
                        }
                    }

                    // Clear OAuth from memory
                    self.oauth_result = None;
                    self.progress_log
                        .push("Disconnected from Google Cloud".to_string());
                }

                ui.add_space(8.0);

                if ui.add(MaterialButton::filled("Next →")).clicked() {
                    self.state = WizardState::SelectProject;
                }
            });

            return;
        }

        ui.label("Click the button below to authorize Dure to access your Google Cloud Platform account.");
        ui.add_space(4.0);
        ui.label("Required permissions:");
        ui.label("  • Compute Engine (create and manage VM instances)");
        ui.label("  • Cloud Platform (project access)");
        ui.add_space(4.0);
        ui.colored_label(
            egui::Color32::GRAY,
            "Note: Only Cloud Resource Manager API needs to be enabled in your OAuth project.",
        );

        ui.add_space(16.0);

        // Check for OAuth promise result
        if let Some(promise) = &self.oauth_promise {
            if let Some(result) = promise.ready() {
                match result {
                    Ok(oauth_result) => {
                        self.oauth_result = Some(oauth_result.clone());
                        self.progress_log.push("✓ OAuth successful".to_string());

                        // Store refresh token in keyring
                        if let Err(e) = self.store_oauth_token(oauth_result) {
                            self.state =
                                WizardState::Error(format!("Failed to store token: {}", e));
                        } else {
                            // Save OAuth to config
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                if let Ok((mut app_config, config_path)) = self.load_config_file() {
                                    if let Err(e) = self.save_oauth_to_config(&mut app_config) {
                                        self.progress_log.push(format!(
                                            "⚠ Failed to save OAuth to config: {}",
                                            e
                                        ));
                                    } else if let Err(e) = app_config.save(&config_path) {
                                        self.progress_log
                                            .push(format!("⚠ Failed to save config: {}", e));
                                    } else {
                                        self.progress_log
                                            .push("✓ OAuth saved to config".to_string());
                                    }
                                }
                            }

                            // Go directly to project configuration (no API calls needed)
                            self.state = WizardState::SelectProject;
                        }

                        self.oauth_promise = None;
                    }
                    Err(e) => {
                        self.state = WizardState::Error(e.clone());
                        self.oauth_promise = None;
                    }
                }
            } else {
                ui.spinner();
                ui.label("Waiting for authorization...");
                ui.label("Please complete the OAuth flow in your browser.");
            }
        } else {
            if ui.add(MaterialButton::filled("Connect Account")).clicked() {
                self.start_oauth();
            }

            ui.add_space(8.0);

            if ui.button("Cancel").clicked() {
                self.hide();
            }
        }
    }

    fn render_select_project(&mut self, ui: &mut egui::Ui) {
        ui.heading("Configure Project");
        ui.add_space(8.0);

        // Load projects on first render
        if !self.projects_loaded {
            self.load_projects();
        }

        // Show error if project loading failed
        if let Some(error) = &self.projects_load_error {
            // Check if it's the API not enabled error
            if error.contains("Cloud Resource Manager API") && error.contains("not been used") {
                ui.colored_label(
                    egui::Color32::from_rgb(255, 152, 0),
                    "⚠ Cloud Resource Manager API Not Enabled",
                );
                ui.add_space(8.0);

                ui.label("The Cloud Resource Manager API needs to be enabled to list projects.");
                ui.add_space(4.0);

                // Extract project number from error
                let project_num = error
                    .split("project ")
                    .nth(1)
                    .and_then(|s| s.split_whitespace().next())
                    .unwrap_or("YOUR_PROJECT");

                let enable_url = format!(
                    "https://console.developers.google.com/apis/api/cloudresourcemanager.googleapis.com/overview?project={}",
                    project_num
                );

                ui.hyperlink_to("→ Click here to enable the API", &enable_url);
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    if ui.button("🔄 Retry Loading Projects").clicked() {
                        self.projects_loaded = false;
                        self.projects_load_error = None;
                    }

                    ui.colored_label(egui::Color32::GRAY, "After enabling, click Retry");
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);

                ui.label("Or proceed without loading (enter project ID manually):");
                ui.add_space(8.0);
            } else {
                ui.colored_label(
                    egui::Color32::from_rgb(245, 101, 101),
                    format!("⚠ {}", error),
                );
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    if ui.button("🔄 Retry").clicked() {
                        self.projects_loaded = false;
                        self.projects_load_error = None;
                    }
                });
                ui.add_space(8.0);
                ui.label("Or proceed by entering project ID manually:");
                ui.add_space(8.0);
            }
        }

        // Only show active projects
        let active_projects: Vec<_> = self
            .available_projects
            .iter()
            .filter(|p| p.is_active())
            .collect();

        // Show simple info about active projects found
        if self.projects_loaded && !active_projects.is_empty() {
            ui.colored_label(
                egui::Color32::GRAY,
                format!(
                    "ℹ Found {} active project{}",
                    active_projects.len(),
                    if active_projects.len() == 1 { "" } else { "s" }
                ),
            );
            ui.add_space(4.0);
        } else if !self.projects_loaded
            && self.oauth_result.is_some()
            && self.projects_load_error.is_none()
        {
            ui.colored_label(egui::Color32::GRAY, "⏳ Loading projects...");
            ui.add_space(4.0);
        }

        // Always show combo box with existing projects + "Create New Project" option
        ui.label("Select a project:");
        ui.add_space(8.0);

        // Build combo box options
        const CREATE_NEW_ID: &str = "__CREATE_NEW_PROJECT__";

        // Determine what to show in combo box
        let combo_display = if self.create_new_project_selected {
            "➕ Create New Project".to_string()
        } else if !self.selected_project_id.is_empty() && self.selected_project_id != CREATE_NEW_ID
        {
            // Find matching project for display
            active_projects
                .iter()
                .find(|p| p.project_id == self.selected_project_id)
                .map(|p| {
                    let display = p.display_name();
                    if display != p.project_id {
                        format!("{} ({})", display, p.project_id)
                    } else {
                        p.project_id.clone()
                    }
                })
                .unwrap_or_else(|| self.selected_project_id.clone())
        } else if !active_projects.is_empty() {
            // Default to first project
            let first = active_projects[0];
            self.selected_project_id = first.project_id.clone();
            self.create_new_project_selected = false;
            let display = first.display_name();
            if display != first.project_id {
                format!("{} ({})", display, first.project_id)
            } else {
                first.project_id.clone()
            }
        } else {
            // No active projects - default to create new
            self.create_new_project_selected = true;
            "➕ Create New Project".to_string()
        };

        ui.horizontal(|ui| {
            ui.label("Project:");

            let combo_width = if !active_projects.is_empty() {
                400.0
            } else {
                300.0
            };

            egui::ComboBox::from_id_source("project_selector")
                .selected_text(&combo_display)
                .width(combo_width)
                .show_ui(ui, |ui| {
                    // Show active projects only
                    for project in &active_projects {
                        let display = project.display_name();
                        let label = if display != project.project_id {
                            format!("{} ({})", display, project.project_id)
                        } else {
                            project.project_id.clone()
                        };

                        if ui
                            .selectable_label(
                                self.selected_project_id == project.project_id
                                    && !self.create_new_project_selected,
                                label,
                            )
                            .clicked()
                        {
                            self.selected_project_id = project.project_id.clone();
                            self.create_new_project_selected = false;
                        }
                    }

                    // Add separator if there are existing projects
                    if !active_projects.is_empty() {
                        ui.separator();
                    }

                    // Always show "Create New Project" option
                    if ui
                        .selectable_label(self.create_new_project_selected, "➕ Create New Project")
                        .clicked()
                    {
                        self.create_new_project_selected = true;
                        // Generate new project ID with timestamp
                        self.selected_project_id =
                            format!("dure-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S"));
                        self.new_project_name = "Dure Server".to_string();
                    }
                });
        });

        ui.add_space(12.0);

        // Compute validation before closures to avoid borrow issues
        let project_id_for_validation = self.selected_project_id.clone();
        let create_new_selected = self.create_new_project_selected;

        // Show input fields when "Create New Project" is selected
        if create_new_selected {
            let is_valid = self.validate_project_id(&project_id_for_validation);
            let is_empty = project_id_for_validation.is_empty();

            ui.group(|ui| {
                ui.set_width(ui.available_width());
                ui.vertical(|ui| {
                    ui.label("New Project Details:");
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        ui.label("Project ID:");
                        ui.text_edit_singleline(&mut self.selected_project_id);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Display Name:");
                        ui.text_edit_singleline(&mut self.new_project_name);
                    });

                    ui.add_space(4.0);
                    ui.colored_label(
                        egui::Color32::GRAY,
                        "💡 Project ID: 6-30 characters, lowercase letters, digits, hyphens",
                    );

                    // Show validation error
                    if !is_valid && !is_empty {
                        ui.colored_label(
                            egui::Color32::from_rgb(245, 101, 101),
                            "⚠ Invalid project ID format",
                        );
                    }
                });
            });
        }

        ui.add_space(16.0);

        // Determine state before entering closure to avoid borrow issues
        let can_proceed = !self.selected_project_id.is_empty()
            && (self.create_new_project_selected || !active_projects.is_empty());
        let has_load_error = self.projects_load_error.is_some();
        let is_new_project = self.create_new_project_selected;

        ui.horizontal(|ui| {
            if ui.button("← Back").clicked() {
                self.state = WizardState::ConnectAccount;
            }

            ui.add_enabled_ui(can_proceed, |ui| {
                if ui.add(MaterialButton::filled("Next →")).clicked() {
                    // If we couldn't load projects due to API error, try to proceed anyway
                    // The project might exist even if we couldn't list it
                    if has_load_error && !is_new_project {
                        // Don't try to create - just proceed and let region loading handle it
                        self.load_regions();
                        self.state = WizardState::ConfigureServer;
                    } else if is_new_project && can_proceed {
                        // It's a new project - create it first
                        self.create_project_and_proceed();
                    } else {
                        // Existing project - just proceed
                        self.load_regions();
                        self.state = WizardState::ConfigureServer;
                    }
                }
            });

            if !can_proceed {
                ui.label("⚠ Select or create a project");
            } else if is_new_project {
                ui.colored_label(
                    egui::Color32::from_rgb(100, 181, 246),
                    "ℹ Will create new project",
                );
            }

            if ui.button("Cancel").clicked() {
                self.hide();
            }
        });
    }

    fn render_configure_server(&mut self, ui: &mut egui::Ui) {
        ui.heading("Configure Server");
        ui.add_space(8.0);

        // Instance name with validation
        ui.horizontal(|ui| {
            ui.label("Instance Name:");
            ui.text_edit_singleline(&mut self.instance_name);
        });

        // Show validation hint
        if !self.instance_name.is_empty() {
            let is_valid = self.validate_instance_name(&self.instance_name);
            if is_valid {
                ui.colored_label(egui::Color32::from_rgb(72, 187, 120), "✓ Valid name");
            } else {
                ui.colored_label(
                    egui::Color32::from_rgb(245, 101, 101),
                    "⚠ Name must start with letter, contain only lowercase letters, numbers, hyphens"
                );
            }
        }

        ui.add_space(8.0);

        // Region selection
        ui.horizontal(|ui| {
            ui.label("Region:");
            egui::ComboBox::from_id_salt("region_combo")
                .selected_text(&self.selected_region)
                .show_ui(ui, |ui| {
                    for region in &self.available_regions {
                        ui.selectable_value(
                            &mut self.selected_region,
                            region.name.clone(),
                            format!("{} ({})", region.name, region.location),
                        );
                    }
                });
        });

        // Zone selection (based on selected region)
        if let Some(region) = self
            .available_regions
            .iter()
            .find(|r| r.name == self.selected_region)
        {
            ui.horizontal(|ui| {
                ui.label("Zone:");
                egui::ComboBox::from_id_salt("zone_combo")
                    .selected_text(&self.selected_zone)
                    .show_ui(ui, |ui| {
                        for zone in &region.zones {
                            ui.selectable_value(&mut self.selected_zone, zone.clone(), zone);
                        }
                    });
            });
        }

        ui.add_space(8.0);

        // Machine type selection
        ui.horizontal(|ui| {
            ui.label("Machine Type:");
            egui::ComboBox::from_id_salt("machine_type_combo")
                .selected_text(&self.selected_machine_type)
                .show_ui(ui, |ui| {
                    for machine_type in &self.available_machine_types {
                        ui.selectable_value(
                            &mut self.selected_machine_type,
                            machine_type.name.clone(),
                            format!("{} - {}", machine_type.name, machine_type.description),
                        );
                    }
                });
        });

        ui.add_space(16.0);

        ui.horizontal(|ui| {
            if ui.button("← Back").clicked() {
                self.state = WizardState::SelectProject;
            }

            let can_create = !self.instance_name.is_empty()
                && self.validate_instance_name(&self.instance_name)
                && !self.selected_region.is_empty()
                && !self.selected_zone.is_empty()
                && !self.selected_machine_type.is_empty();

            let create_button = MaterialButton::filled("Create Server");
            ui.add_enabled_ui(can_create, |ui| {
                if ui.add(create_button).clicked() {
                    self.start_server_creation();
                }
            });

            if !can_create {
                ui.label("⚠ Complete all fields");
            }

            if ui.button("Cancel").clicked() {
                self.hide();
            }
        });
    }

    fn render_creating_server(&mut self, ui: &mut egui::Ui) {
        ui.heading("Creating Server");
        ui.add_space(8.0);

        ui.spinner();
        ui.label("Please wait while we create your GCP Compute Engine instance...");

        ui.add_space(8.0);
        ui.colored_label(egui::Color32::GRAY, "This usually takes 1-2 minutes...");

        ui.add_space(16.0);

        // Show progress log
        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                for log in &self.progress_log {
                    ui.label(log);
                }
            });

        // Check for creation promise result
        if let Some(promise) = &self.create_promise {
            if let Some(result) = promise.ready() {
                match result {
                    Ok(instance) => {
                        self.created_instance = Some(instance.clone());
                        self.progress_log
                            .push("✓ Server created successfully!".to_string());
                        self.state = WizardState::Complete;
                        self.create_promise = None;
                    }
                    Err(e) => {
                        self.state = WizardState::Error(e.clone());
                        self.create_promise = None;
                    }
                }
            }
        }
    }

    fn render_complete(&mut self, ui: &mut egui::Ui) {
        ui.heading("✓ Setup Complete!");
        ui.add_space(8.0);

        if let Some(instance) = &self.created_instance {
            // Check if instance is actually running
            let is_running = instance.status.to_uppercase() == "RUNNING";

            if is_running {
                ui.colored_label(
                    egui::Color32::from_rgb(72, 187, 120),
                    "✓ Instance is RUNNING and ready to use!",
                );
            } else {
                ui.colored_label(
                    egui::Color32::from_rgb(255, 193, 7),
                    format!(
                        "⏳ Instance status: {} (waiting for RUNNING)",
                        instance.status
                    ),
                );
            }

            ui.add_space(12.0);

            ui.group(|ui| {
                ui.set_width(ui.available_width());

                ui.label(format!("📦 Instance Name: {}", instance.name));
                ui.label(format!("🆔 Instance ID: {}", instance.id));
                ui.label(format!("⚙️  Machine Type: {}", instance.machine_type));
                ui.label(format!("📍 Zone: {}", instance.zone));

                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);

                let status_color = if is_running {
                    egui::Color32::from_rgb(72, 187, 120)
                } else {
                    egui::Color32::from_rgb(255, 193, 7)
                };
                ui.colored_label(status_color, format!("📊 Status: {}", instance.status));

                if let Some(ip) = &instance.external_ip {
                    ui.colored_label(
                        egui::Color32::from_rgb(103, 126, 234),
                        format!("🌐 External IP: {}", ip),
                    );

                    // SSH command hint
                    ui.add_space(4.0);
                    ui.label(format!("💻 SSH: ssh user@{}", ip));
                } else {
                    ui.label("🌐 External IP: (assigning...)");
                }

                if let Some(internal_ip) = &instance.internal_ip {
                    ui.label(format!("🔒 Internal IP: {}", internal_ip));
                }
            });

            ui.add_space(12.0);

            ui.label("📋 Next steps:");
            if !is_running {
                ui.label("  1. ⏳ Wait for instance to reach RUNNING status");
            }
            ui.label("  2. 🔑 Configure SSH access (add your public key)");
            ui.label("  3. 🔐 Set up firewall rules if needed");
            ui.label("  4. 📦 Install your application");

            ui.add_space(8.0);
            ui.label("💡 View in GCP Console:");
            ui.hyperlink_to(
                format!(
                    "https://console.cloud.google.com/compute/instances?project={}",
                    self.selected_project_id
                ),
                format!(
                    "https://console.cloud.google.com/compute/instances?project={}",
                    self.selected_project_id
                ),
            );
        } else {
            ui.colored_label(egui::Color32::RED, "⚠ Instance information not available");
            ui.label("The creation may have failed. Check the GCP Console.");
        }

        ui.add_space(16.0);

        if ui.add(MaterialButton::filled("Close")).clicked() {
            self.hide();
        }
    }

    fn render_error(&mut self, ui: &mut egui::Ui, error: &str) {
        ui.heading("❌ Error");
        ui.add_space(8.0);

        // Check for specific API not enabled errors
        let is_compute_api_error =
            error.contains("Compute Engine API") && error.contains("is not enabled");

        if is_compute_api_error {
            // Extract project ID from error message
            let project_id = if let Some(pos) = error.find("project '") {
                let start = pos + 9; // Length of "project '"
                error[start..]
                    .find('\'')
                    .map(|end_pos| &error[start..start + end_pos])
            } else {
                None
            };

            ui.colored_label(
                egui::Color32::from_rgb(255, 152, 0), // Orange
                error.split('\n').next().unwrap_or(error),
            );

            ui.add_space(12.0);

            ui.label("To fix this (one-time setup):");
            ui.add_space(4.0);

            if let Some(proj_id) = project_id {
                let enable_url = format!(
                    "https://console.developers.google.com/apis/api/compute.googleapis.com/overview?project={}",
                    proj_id
                );

                ui.label("1. Open: ");
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    if ui.link(&enable_url).clicked() {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = webbrowser::open(&enable_url);
                        }
                    }
                });
            } else {
                ui.label("1. Open the GCP Console for your project");
            }

            ui.add_space(4.0);
            ui.label("2. Click 'Enable API'");
            ui.add_space(4.0);
            ui.label("3. Wait a few minutes for changes to propagate");
            ui.add_space(4.0);
            ui.label("4. Return here and click 'Create Server' again");

            ui.add_space(12.0);
            ui.colored_label(
                egui::Color32::GRAY,
                "Note: This needs to be done once per GCP project.",
            );

            ui.add_space(16.0);

            ui.horizontal(|ui| {
                if ui
                    .add(MaterialButton::filled("← Back to Configure"))
                    .clicked()
                {
                    self.state = WizardState::ConfigureServer;
                }

                if ui.button("Close").clicked() {
                    self.hide();
                }
            });
        } else {
            // Generic error display
            egui::ScrollArea::vertical()
                .max_height(300.0)
                .show(ui, |ui| {
                    ui.colored_label(egui::Color32::RED, error);
                });

            ui.add_space(16.0);

            ui.horizontal(|ui| {
                if ui.button("← Start Over").clicked() {
                    self.state = WizardState::ConnectAccount;
                    self.progress_log.clear();
                }

                if ui.button("Close").clicked() {
                    self.hide();
                }
            });
        }
    }

    fn start_oauth(&mut self) {
        self.progress_log.push("Starting OAuth flow...".to_string());

        // Load OAuth credentials from config
        let (client_id, client_secret) = match self.load_oauth_credentials() {
            Ok((id, secret)) => (id, secret),
            Err(e) => {
                self.state = WizardState::Error(format!(
                    "Failed to load OAuth credentials: {}\n\n\
                     Please configure gcp_oauth_client_id and gcp_oauth_client_secret \
                     in ~/.config/dure/config.yml or set GCP_CLIENT_ID and GCP_CLIENT_SECRET \
                     environment variables.",
                    e
                ));
                return;
            }
        };

        // Validate credentials
        if client_id.is_empty() || client_secret.is_empty() {
            self.state = WizardState::Error(
                "OAuth credentials not configured.\n\n\
                 Please add to ~/.config/dure/config.yml:\n\
                 platforms:\n\
                   - name: \"your-platform\"\n\
                     platform_type: \"gcp\"\n\
                     gcp_oauth_client_id: \"your-client-id.apps.googleusercontent.com\"\n\
                     gcp_oauth_client_secret: \"GOCSPX-your-secret\""
                    .to_string(),
            );
            return;
        }

        let handler = OAuthHandler::new(client_id, client_secret);

        self.oauth_promise = Some(Promise::spawn_thread("gcp_oauth", move || {
            handler.run_oauth_flow().map_err(|e| e.to_string())
        }));
    }

    /// Load OAuth credentials from config file or environment variables
    fn load_oauth_credentials(&self) -> Result<(String, String), String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Try to load from config file
            if let Ok(config_path) = self.get_config_path() {
                let app_config = AppConfig::load_or_default(&config_path);

                // Find the platform by name
                if let Some(platform) = app_config
                    .platforms
                    .iter()
                    .find(|p| p.name == self.platform_name)
                {
                    // Use config values if available
                    if let (Some(id), Some(secret)) = (
                        &platform.gcp_oauth_client_id,
                        &platform.gcp_oauth_client_secret,
                    ) {
                        return Ok((id.clone(), secret.clone()));
                    }
                }
            }

            // Fallback to environment variables
            let client_id = std::env::var("GCP_CLIENT_ID").unwrap_or_default();
            let client_secret = std::env::var("GCP_CLIENT_SECRET").unwrap_or_default();

            if !client_id.is_empty() && !client_secret.is_empty() {
                return Ok((client_id, client_secret));
            }

            Err("OAuth credentials not found in config or environment".to_string())
        }

        #[cfg(target_arch = "wasm32")]
        {
            Err("OAuth not supported on WASM".to_string())
        }
    }

    /// Get config file path
    #[cfg(not(target_arch = "wasm32"))]
    fn get_config_path(&self) -> Result<std::path::PathBuf, String> {
        directories::ProjectDirs::from("pe", "nikescar", "dure")
            .map(|proj_dirs| proj_dirs.config_dir().join("config.yml"))
            .ok_or_else(|| "Failed to get config directory".to_string())
    }

    /// Load config file
    #[cfg(not(target_arch = "wasm32"))]
    fn load_config_file(&self) -> Result<(AppConfig, std::path::PathBuf), String> {
        let config_path = self.get_config_path()?;
        let app_config = AppConfig::load_or_default(&config_path);
        Ok((app_config, config_path))
    }

    fn store_oauth_token(&self, oauth_result: &OAuthResult) -> Result<(), String> {
        // Store refresh token in keyring (overwrites existing if present)
        let domain = format!("gcp.{}", self.platform_name);
        let username = "oauth";
        let password = &oauth_result.refresh_token;

        // Get keyring paths
        let kdbx_path = keyring::get_default_kdbx_path()
            .map_err(|e| format!("Failed to get kdbx path: {}", e))?;
        let keyfile_path = keyring::get_default_keyfile_path()
            .map_err(|e| format!("Failed to get keyfile path: {}", e))?;

        keyring::update_key(&kdbx_path, Some(&keyfile_path), &domain, username, password)
            .map_err(|e| format!("Failed to store OAuth token: {}", e))?;

        Ok(())
    }

    fn load_regions(&mut self) {
        if let Some(oauth) = &self.oauth_result {
            let client = GcpRestClient::new(oauth.access_token.clone());

            match client.list_regions(&self.selected_project_id) {
                Ok(region_list) => {
                    self.available_regions = region_list
                        .items
                        .into_iter()
                        .map(|r| Region {
                            name: r.name.clone(),
                            location: r.description,
                            zones: r
                                .zones
                                .iter()
                                .filter_map(|z| z.split('/').next_back().map(String::from))
                                .collect(),
                        })
                        .collect();

                    self.progress_log
                        .push(format!("✓ Loaded {} regions", self.available_regions.len()));
                }
                Err(e) => {
                    // Check if it's a "not found" error for a newly created project
                    let error_msg = e.to_string();
                    let is_new_project_not_ready = error_msg.contains("404")
                        || error_msg.contains("NOT_FOUND")
                        || error_msg.contains("was not found");

                    if is_new_project_not_ready {
                        self.progress_log.push(
                            "ℹ Project not fully ready yet - using default regions".to_string(),
                        );
                    } else {
                        self.progress_log
                            .push(format!("⚠ Failed to load regions: {}", e));
                    }

                    // Fallback to static list
                    self.available_regions = vec![
                        Region {
                            name: "us-central1".to_string(),
                            location: "Iowa, USA".to_string(),
                            zones: vec!["us-central1-a".to_string(), "us-central1-b".to_string()],
                        },
                        Region {
                            name: "us-east1".to_string(),
                            location: "South Carolina, USA".to_string(),
                            zones: vec!["us-east1-b".to_string(), "us-east1-c".to_string()],
                        },
                        Region {
                            name: "asia-northeast3".to_string(),
                            location: "Seoul, South Korea".to_string(),
                            zones: vec!["asia-northeast3-a".to_string()],
                        },
                    ];

                    self.progress_log.push(format!(
                        "✓ Using {} default regions",
                        self.available_regions.len()
                    ));
                }
            }
        }
    }

    fn load_projects(&mut self) {
        if self.projects_loaded {
            return;
        }

        if let Some(oauth) = &self.oauth_result {
            let client = GcpRestClient::new(oauth.access_token.clone());

            match client.list_projects(None) {
                Ok(project_list) => {
                    self.available_projects = project_list.projects;
                    self.projects_loaded = true;
                    self.projects_load_error = None;

                    // Log project details for debugging
                    log::info!(
                        "Loaded {} projects from GCP API",
                        self.available_projects.len()
                    );
                    for proj in &self.available_projects {
                        let state_str = proj.state.as_deref().unwrap_or("<no state field>");
                        log::debug!(
                            "  Project: {} ({}), state: {:?}",
                            proj.display_name(),
                            proj.project_id,
                            state_str
                        );
                    }

                    let active_count = self
                        .available_projects
                        .iter()
                        .filter(|p| p.is_active())
                        .count();
                    log::info!(
                        "  {} active/usable projects, {} total",
                        active_count,
                        self.available_projects.len()
                    );

                    // Auto-select first active project if available
                    if let Some(project) = self.available_projects.iter().find(|p| p.is_active()) {
                        self.selected_project_id = project.project_id.clone();
                        log::info!("Auto-selected first active project: {}", project.project_id);
                    } else {
                        log::warn!(
                            "No active projects found among {} total projects",
                            self.available_projects.len()
                        );
                    }
                }
                Err(e) => {
                    self.projects_load_error = Some(format!("Failed to load projects: {}", e));
                    self.projects_loaded = true;
                    log::error!("Failed to load projects: {}", e);
                }
            }
        }
    }

    fn create_project_and_proceed(&mut self) {
        if let Some(oauth) = &self.oauth_result {
            let client = GcpRestClient::new(oauth.access_token.clone());

            let project_id = self.selected_project_id.clone();
            let display_name = self.new_project_name.clone();

            match client.create_project(&project_id, &display_name) {
                Ok(_operation) => {
                    // Project creation initiated successfully
                    // Note: Project creation is async and may take time
                    // For now, we'll proceed immediately and let region loading fail if needed
                    self.progress_log
                        .push(format!("✓ Project '{}' creation initiated", project_id));

                    // Add a small delay to allow project to be created
                    // In production, we should poll the operation status
                    self.progress_log
                        .push("ℹ Waiting for project to be ready...".to_string());

                    // Proceed to next step
                    self.load_regions();
                    self.state = WizardState::ConfigureServer;
                }
                Err(e) => {
                    // Show error but don't transition state
                    self.projects_load_error = Some(format!("Failed to create project: {}", e));
                }
            }
        }
    }

    fn start_server_creation(&mut self) {
        self.state = WizardState::CreatingServer;
        self.progress_log.push("Creating server...".to_string());

        let project_id = self.selected_project_id.clone();
        let zone = self.selected_zone.clone();
        let instance_name = self.instance_name.clone();
        let machine_type = self.selected_machine_type.clone();

        let access_token = self
            .oauth_result
            .as_ref()
            .map(|o| o.access_token.clone())
            .unwrap_or_default();

        self.create_promise = Some(Promise::spawn_thread("gcp_create_vm", move || {
            let client = GcpRestClient::new(access_token);

            // Create instance request (skip project existence check - assumes project exists)
            // If project doesn't exist, VM creation will fail with clear error
            let instance_req = InstanceRequest::debian_micro(instance_name.clone(), zone.clone());

            // Customize machine type if not default
            let mut instance_req = instance_req;
            if machine_type != "e2-micro" {
                instance_req.machine_type = format!("zones/{}/machineTypes/{}", zone, machine_type);
            }

            // Create the instance
            let operation = client
                .create_instance(&project_id, &zone, &instance_req)
                .map_err(|e| format!("Failed to create instance: {}", e))?;

            // Wait for operation to complete (10 minute timeout)
            let result = client
                .wait_for_operation(&project_id, &zone, &operation.name, 600)
                .map_err(|e| format!("Operation failed: {}", e))?;

            if let Some(error) = result.error {
                return Err(format!("Creation failed: {:?}", error.errors));
            }

            // Get instance details
            let instance = client
                .get_instance(&project_id, &zone, &instance_name)
                .map_err(|e| format!("Failed to get instance: {}", e))?;

            // Extract IPs before moving other fields
            let external_ip = instance.external_ip();
            let internal_ip = instance.internal_ip();

            // Convert to our Instance type
            Ok(Instance {
                id: instance.id,
                name: instance.name,
                machine_type: instance.machine_type,
                zone: instance.zone,
                status: instance.status,
                external_ip,
                internal_ip,
                creation_timestamp: String::new(),
            })
        }));
    }

    /// Validate GCP project ID
    /// Rules: 6-30 characters, lowercase letters, digits, hyphens, start with letter
    fn validate_project_id(&self, id: &str) -> bool {
        if id.is_empty() || id.len() < 6 || id.len() > 30 {
            return false;
        }

        // Must start with a lowercase letter
        if !id
            .chars()
            .next()
            .map(|c| c.is_ascii_lowercase())
            .unwrap_or(false)
        {
            return false;
        }

        // Can only contain lowercase letters, digits, and hyphens
        id.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    }

    /// Validate GCP instance name
    /// Rules: 1-63 characters, lowercase letters, numbers, hyphens, start with letter
    fn validate_instance_name(&self, name: &str) -> bool {
        if name.is_empty() || name.len() > 63 {
            return false;
        }

        // Must start with a lowercase letter
        if !name
            .chars()
            .next()
            .map(|c| c.is_ascii_lowercase())
            .unwrap_or(false)
        {
            return false;
        }

        // Can only contain lowercase letters, numbers, and hyphens
        name.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    }
}
