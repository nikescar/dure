//! Platform tab - Platform configuration and management

use eframe::egui;
use egui_material3::spreadsheet::{text_column, MaterialSpreadsheet};
use egui_material3::MaterialButton;

use crate::calc::audit;
use crate::config::{AppConfig, CloudPlatformConfig};

#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
use crate::ui_dlg::platform_gcp::GcpWizard;

/// Platform tab state
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PlatformTab {
    selected_row: Option<usize>,
    /// Cached platform rows (name, type, details)
    #[cfg_attr(feature = "serde", serde(skip))]
    rows: Vec<[String; 3]>,
    #[cfg_attr(feature = "serde", serde(skip))]
    loaded: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    load_error: Option<String>,

    // Spreadsheet
    #[cfg_attr(feature = "serde", serde(skip))]
    spreadsheet: Option<MaterialSpreadsheet>,
    row_selection_enabled: bool,

    // Add dialog state
    #[cfg_attr(feature = "serde", serde(skip))]
    show_add_dialog: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    add_platform_name: String,
    #[cfg_attr(feature = "serde", serde(skip))]
    add_platform_type: String,

    // Init progress state
    #[cfg_attr(feature = "serde", serde(skip))]
    init_in_progress: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    init_platform_name: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip))]
    init_progress_log: Vec<String>,

    // GCP wizard
    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    #[cfg_attr(feature = "serde", serde(skip))]
    gcp_wizard: Option<GcpWizard>,
}

impl Default for PlatformTab {
    fn default() -> Self {
        let spreadsheet = {
            let columns = vec![
                text_column("Name", 200.0),
                text_column("Type", 120.0),
                text_column("Details", 300.0),
            ];

            // Create spreadsheet with theme-aware settings
            MaterialSpreadsheet::new("platform_spreadsheet", columns)
                .ok()
                .map(|mut s| {
                    // Enable striped rows for Material Design theme colors
                    s.set_striped(true);
                    // Enable row selection
                    s.set_row_selection_enabled(true);
                    // Enable selection for better visual feedback
                    s.set_allow_selection(true);
                    s
                })
        };

        Self {
            selected_row: None,
            rows: Vec::new(),
            loaded: false,
            load_error: None,
            spreadsheet,
            row_selection_enabled: true,
            show_add_dialog: false,
            add_platform_name: String::new(),
            add_platform_type: "gcp".to_string(),
            init_in_progress: false,
            init_platform_name: None,
            init_progress_log: Vec::new(),
            #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
            gcp_wizard: None,
        }
    }
}

/// Get config file path
#[cfg(not(target_arch = "wasm32"))]
fn get_config_path() -> Result<std::path::PathBuf, String> {
    let proj_dirs = directories::ProjectDirs::from("pe", "nikescar", "dure")
        .ok_or_else(|| "Failed to get project directories".to_string())?;
    Ok(proj_dirs.config_dir().join("config.yml"))
}

/// Load application config
#[cfg(not(target_arch = "wasm32"))]
fn load_config() -> Result<(AppConfig, std::path::PathBuf), String> {
    let config_path = get_config_path()?;
    let app_config = AppConfig::load_or_default(&config_path);
    Ok((app_config, config_path))
}

impl PlatformTab {
    /// Render the platform tab UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Cloud Platforms");
        ui.add_space(4.0);
        ui.label(
            "Manage cloud service platforms (GCP, Firebase, Supabase) for deployment and hosting.",
        );
        ui.add_space(8.0);

        // Get selected row for delete button state
        let selected_row_idx = self.spreadsheet.as_ref().and_then(|s| s.get_selected_row());
        let has_selection = selected_row_idx.is_some();

        // Action buttons
        ui.horizontal(|ui| {
            if ui.add(MaterialButton::filled("Add Platform")).clicked() {
                self.show_add_dialog = true;
                self.add_platform_name.clear();
                self.add_platform_type = "gcp".to_string();
            }

            // Init button - enabled only when a GCP platform is selected
            #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
            {
                let can_init = if let Some(idx) = selected_row_idx {
                    idx < self.rows.len() && self.rows[idx][1] == "gcp"
                } else {
                    false
                };

                let init_button = MaterialButton::outlined("Init Server");
                let init_button = if can_init {
                    init_button
                } else {
                    init_button.enabled(false)
                };

                if ui.add(init_button).clicked() {
                    if let Some(idx) = selected_row_idx {
                        if idx < self.rows.len() {
                            let platform_name = self.rows[idx][0].clone();
                            self.show_gcp_wizard(platform_name);
                        }
                    }
                }
            }

            // Delete button - enabled only when a row is selected
            let delete_button = MaterialButton::outlined("Delete Selected");
            let delete_button = if has_selection {
                delete_button
            } else {
                delete_button.enabled(false)
            };

            if ui.add(delete_button).clicked() {
                if let Some(idx) = selected_row_idx {
                    if idx < self.rows.len() {
                        let platform_name = self.rows[idx][0].clone();
                        self.execute_delete_platform(platform_name);
                    }
                }
            }

            if ui.add(MaterialButton::outlined("Refresh")).clicked() {
                self.loaded = false;
                self.load_error = None;
            }

            // Show selected platform info
            if let Some(idx) = selected_row_idx {
                if idx < self.rows.len() {
                    ui.label(format!(
                        "│ Selected: {} ({})",
                        self.rows[idx][0], self.rows[idx][1]
                    ));
                }
            }
        });
        ui.add_space(8.0);

        // Lazy-load from DB on first render or after refresh
        if !self.loaded {
            self.load_rows();
            self.loaded = true;
        }

        if let Some(err) = &self.load_error {
            ui.colored_label(egui::Color32::RED, format!("⚠ {err}"));
            ui.add_space(4.0);
        }

        // Platform spreadsheet - fill remaining space
        if let Some(spreadsheet) = &mut self.spreadsheet {
            let available_height = ui.available_height();

            ui.group(|ui| {
                // Set the group to fill available space
                ui.set_min_height(available_height - 20.0); // Leave some padding
                ui.set_width(ui.available_width());

                egui::ScrollArea::vertical()
                    .max_height(available_height - 20.0)
                    .show(ui, |ui| {
                        spreadsheet.show(ui);
                    });
            });
        }

        // Add platform dialog
        if self.show_add_dialog {
            self.render_add_dialog(ui.ctx());
        }

        // Init progress display
        if self.init_in_progress {
            self.render_init_progress(ui);
        }

        // GCP wizard dialog
        #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
        if let Some(wizard) = &mut self.gcp_wizard {
            wizard.ui(ui.ctx());
        }
    }

    fn load_rows(&mut self) {
        self.rows.clear();
        self.load_error = None;

        #[cfg(not(target_arch = "wasm32"))]
        {
            match load_config() {
                Ok((app_config, _)) => {
                    let mut data_rows = Vec::new();

                    for platform in &app_config.platforms {
                        let details = match platform.platform_type.as_str() {
                            "gcp" => {
                                let mut parts = Vec::new();
                                if let Some(project) = &platform.gcp_project_id {
                                    parts.push(format!("Project: {}", project));
                                }
                                if let Some(region) = &platform.gcp_region {
                                    parts.push(format!("Region: {}", region));
                                }
                                parts.join(", ")
                            }
                            "firebase" => {
                                if let Some(project) = &platform.firebase_project_id {
                                    format!("Project: {}", project)
                                } else {
                                    String::new()
                                }
                            }
                            "supabase" => {
                                if let Some(url) = &platform.supabase_api_url {
                                    format!("URL: {}", url)
                                } else {
                                    String::new()
                                }
                            }
                            _ => String::new(),
                        };

                        self.rows.push([
                            platform.name.clone(),
                            platform.platform_type.clone(),
                            details.clone(),
                        ]);

                        data_rows.push(vec![
                            platform.name.clone(),
                            platform.platform_type.clone(),
                            details,
                        ]);
                    }

                    // Clear and update spreadsheet with fresh data
                    if let Some(spreadsheet) = &mut self.spreadsheet {
                        // Recreate spreadsheet with fresh data to avoid duplicates
                        let columns = vec![
                            text_column("Name", 200.0),
                            text_column("Type", 120.0),
                            text_column("Details", 300.0),
                        ];

                        if let Ok(mut new_spreadsheet) =
                            MaterialSpreadsheet::new("platform_spreadsheet", columns)
                        {
                            // Apply theme-aware settings
                            new_spreadsheet.set_striped(true); // Enable Material Design theme colors
                            new_spreadsheet.set_row_selection_enabled(self.row_selection_enabled);
                            new_spreadsheet.set_allow_selection(true);
                            new_spreadsheet.init_with_data(data_rows);
                            *spreadsheet = new_spreadsheet;
                        }
                    }
                }
                Err(e) => {
                    self.load_error = Some(format!("Failed to load config: {e}"));
                }
            }
        }
    }

    fn render_add_dialog(&mut self, ctx: &egui::Context) {
        let mut open = self.show_add_dialog;

        egui::Window::new("Add Platform")
            .open(&mut open)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.label("Configure a new cloud platform:");
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut self.add_platform_name);
                });

                ui.horizontal(|ui| {
                    ui.label("Type:");
                    egui::ComboBox::from_id_salt("platform_type_combo")
                        .selected_text(&self.add_platform_type)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.add_platform_type,
                                "gcp".to_string(),
                                "GCP (Google Cloud Platform)",
                            );
                            // TODO: Re-enable when Firebase/Supabase support is implemented
                            // ui.selectable_value(&mut self.add_platform_type, "firebase".to_string(), "Firebase");
                            // ui.selectable_value(&mut self.add_platform_type, "supabase".to_string(), "Supabase");
                        });
                });

                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.show_add_dialog = false;
                    }

                    if ui.button("Add").clicked() {
                        if self.add_platform_name.is_empty() {
                            // Show error
                            self.load_error = Some("Platform name is required".to_string());
                        } else {
                            self.execute_add_platform();
                            self.show_add_dialog = false;
                        }
                    }
                });
            });

        if !open {
            self.show_add_dialog = false;
        }
    }

    fn execute_add_platform(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            match load_config() {
                Ok((mut app_config, config_path)) => {
                    // Check if platform already exists
                    if app_config
                        .platforms
                        .iter()
                        .any(|p| p.name == self.add_platform_name)
                    {
                        self.load_error = Some(format!(
                            "Platform '{}' already exists",
                            self.add_platform_name
                        ));
                        return;
                    }

                    // Create new platform
                    let platform = CloudPlatformConfig {
                        name: self.add_platform_name.clone(),
                        platform_type: self.add_platform_type.clone(),
                        gcp_project_id: None,
                        gcp_billing_account: None,
                        gcp_region: None,
                        gcp_oauth_client_id: None,
                        gcp_oauth_client_secret: None,
                        gcp_oauth_access_token: None,
                        gcp_oauth_refresh_token: None,
                        gcp_oauth_token_expiry: None,
                        firebase_project_id: None,
                        firebase_api_key: None,
                        supabase_project_ref: None,
                        supabase_api_url: None,
                        supabase_anon_key: None,
                        api_token: None,
                        service_account_json: None,
                    };

                    // Add to config
                    app_config.platforms.push(platform);

                    // Save config
                    match app_config.save(&config_path) {
                        Ok(_) => {
                            // Record audit event
                            match audit::push_gui(
                                "system",
                                "desktop",
                                "platform add",
                                &self.add_platform_name,
                            ) {
                                Ok(audit_id) => {
                                    eprintln!("✓ Audit record created: ID {}", audit_id);
                                }
                                Err(e) => {
                                    eprintln!("⚠ Failed to record audit event: {}", e);
                                    self.load_error = Some(format!("Audit tracking failed: {}", e));
                                }
                            }

                            self.loaded = false; // Trigger reload
                            self.load_error = None;
                        }
                        Err(e) => {
                            self.load_error = Some(format!("Failed to save config: {e}"));
                        }
                    }
                }
                Err(e) => {
                    self.load_error = Some(format!("Failed to load config: {e}"));
                }
            }
        }
    }

    fn execute_delete_platform(&mut self, name: String) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            match load_config() {
                Ok((mut app_config, config_path)) => {
                    // Find and remove platform
                    if let Some(idx) = app_config.platforms.iter().position(|p| p.name == name) {
                        app_config.platforms.remove(idx);

                        // Save config
                        match app_config.save(&config_path) {
                            Ok(_) => {
                                // Record audit event
                                match audit::push_gui("system", "desktop", "platform del", &name) {
                                    Ok(audit_id) => {
                                        eprintln!("✓ Audit record created: ID {}", audit_id);
                                    }
                                    Err(e) => {
                                        eprintln!("⚠ Failed to record audit event: {}", e);
                                    }
                                }

                                self.loaded = false; // Trigger reload
                                self.selected_row = None;
                                self.load_error = None;
                            }
                            Err(e) => {
                                self.load_error = Some(format!("Failed to save config: {e}"));
                            }
                        }
                    } else {
                        self.load_error = Some(format!("Platform '{}' not found", name));
                    }
                }
                Err(e) => {
                    self.load_error = Some(format!("Failed to load config: {e}"));
                }
            }
        }
    }

    fn render_init_progress(&mut self, ui: &mut egui::Ui) {
        ui.add_space(12.0);
        ui.separator();
        ui.heading("Initialization Progress");

        if let Some(name) = &self.init_platform_name {
            ui.label(format!("Platform: {}", name));
        }

        ui.add_space(8.0);

        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                for log in &self.init_progress_log {
                    ui.label(log);
                }
            });
    }

    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    fn show_gcp_wizard(&mut self, platform_name: String) {
        let mut wizard = GcpWizard::new(platform_name);

        // Load OAuth from config if exists
        if let Ok((app_config, _)) = load_config() {
            wizard.load_oauth_from_config(&app_config);
        }

        wizard.show();
        self.gcp_wizard = Some(wizard);
    }
}
