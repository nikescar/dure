//! SSH tab - SSH host configuration and management

use eframe::egui;
use egui_material3::spreadsheet::{text_column, MaterialSpreadsheet};
use egui_material3::MaterialButton;

use crate::calc::audit;
use crate::calc::ssh;
use crate::config::{AppConfig, SshHostConfig};

/// SSH tab state
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SshTab {
    selected_row: Option<usize>,
    /// Cached SSH host rows (host, port, auth type, status)
    #[cfg_attr(feature = "serde", serde(skip))]
    rows: Vec<[String; 4]>,
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
    add_host: String,
    #[cfg_attr(feature = "serde", serde(skip))]
    add_password: String,
    #[cfg_attr(feature = "serde", serde(skip))]
    add_private_key_path: String,
    #[cfg_attr(feature = "serde", serde(skip))]
    add_port: String,
    #[cfg_attr(feature = "serde", serde(skip))]
    add_use_password: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    add_use_private_key: bool,

    // Init progress state
    #[cfg_attr(feature = "serde", serde(skip))]
    init_in_progress: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    init_host: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip))]
    init_progress_log: Vec<String>,
}

impl Default for SshTab {
    fn default() -> Self {
        let spreadsheet = {
            let columns = vec![
                text_column("Host", 250.0),
                text_column("Port", 80.0),
                text_column("Auth", 150.0),
                text_column("Status", 150.0),
            ];

            // Create spreadsheet with theme-aware settings
            MaterialSpreadsheet::new("ssh_spreadsheet", columns)
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
            add_host: String::new(),
            add_password: String::new(),
            add_private_key_path: String::new(),
            add_port: "22".to_string(),
            add_use_password: false,
            add_use_private_key: false,
            init_in_progress: false,
            init_host: None,
            init_progress_log: Vec::new(),
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

impl SshTab {
    /// Render the SSH tab UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("SSH Hosts");
        ui.add_space(4.0);
        ui.label("Manage SSH hosts for remote server deployment and management.");
        ui.add_space(8.0);

        // Get selected row for action buttons
        let selected_row_idx = self.spreadsheet.as_ref().and_then(|s| s.get_selected_row());
        let has_selection = selected_row_idx.is_some();

        // Action buttons
        ui.horizontal(|ui| {
            if ui.add(MaterialButton::filled("Add Host")).clicked() {
                self.show_add_dialog = true;
                self.add_host.clear();
                self.add_password.clear();
                self.add_private_key_path.clear();
                self.add_port = "22".to_string();
                self.add_use_password = false;
                self.add_use_private_key = false;
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
                        let host = self.rows[idx][0].clone();
                        self.execute_delete_host(host);
                    }
                }
            }

            // Initialize button - enabled only when a row is selected
            let init_button = MaterialButton::outlined("Initialize");
            let init_button = if has_selection && !self.init_in_progress {
                init_button
            } else {
                init_button.enabled(false)
            };

            if ui.add(init_button).clicked() {
                if let Some(idx) = selected_row_idx {
                    if idx < self.rows.len() {
                        let host = self.rows[idx][0].clone();
                        self.execute_init_host(host);
                    }
                }
            }

            if ui.add(MaterialButton::outlined("Refresh")).clicked() {
                self.loaded = false;
                self.load_error = None;
            }

            // Show selected host info
            if let Some(idx) = selected_row_idx {
                if idx < self.rows.len() {
                    ui.label(format!("│ Selected: {}", self.rows[idx][0]));
                }
            }
        });
        ui.add_space(8.0);

        // Lazy-load from config on first render or after refresh
        if !self.loaded {
            self.load_rows();
            self.loaded = true;
        }

        if let Some(err) = &self.load_error {
            ui.colored_label(egui::Color32::RED, format!("⚠ {err}"));
            ui.add_space(4.0);
        }

        // SSH hosts spreadsheet - fill remaining space
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

        // Add host dialog
        if self.show_add_dialog {
            self.render_add_dialog(ui.ctx());
        }

        // Init progress display
        if self.init_in_progress {
            self.render_init_progress(ui);
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

                    for host_config in &app_config.ssh_hosts {
                        let auth_type = if host_config.private_key_path.is_some() {
                            "Private Key"
                        } else if host_config.password.is_some() {
                            "Password"
                        } else {
                            "SSH Agent"
                        };

                        let status = if host_config.initialized {
                            "Initialized"
                        } else {
                            "Not Initialized"
                        };

                        self.rows.push([
                            host_config.host.clone(),
                            host_config.port.to_string(),
                            auth_type.to_string(),
                            status.to_string(),
                        ]);

                        data_rows.push(vec![
                            host_config.host.clone(),
                            host_config.port.to_string(),
                            auth_type.to_string(),
                            status.to_string(),
                        ]);
                    }

                    // Clear and update spreadsheet with fresh data
                    if let Some(spreadsheet) = &mut self.spreadsheet {
                        // Recreate spreadsheet with fresh data to avoid duplicates
                        let columns = vec![
                            text_column("Host", 250.0),
                            text_column("Port", 80.0),
                            text_column("Auth", 150.0),
                            text_column("Status", 150.0),
                        ];

                        match MaterialSpreadsheet::new("ssh_spreadsheet", columns) {
                            Ok(mut new_spreadsheet) => {
                                new_spreadsheet.set_striped(true);
                                new_spreadsheet.set_row_selection_enabled(true);
                                new_spreadsheet.set_allow_selection(true);
                                new_spreadsheet.init_with_data(data_rows);
                                *spreadsheet = new_spreadsheet;
                            }
                            Err(e) => {
                                self.load_error =
                                    Some(format!("Failed to create spreadsheet: {e}"));
                            }
                        }
                    }
                }
                Err(e) => {
                    self.load_error = Some(format!("Failed to load config: {e}"));
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            self.load_error = Some("SSH management not available on WASM".to_string());
        }
    }

    fn render_add_dialog(&mut self, ctx: &egui::Context) {
        let mut open = true;

        egui::Window::new("Add SSH Host")
            .open(&mut open)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.label("Configure a new SSH host:");
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label("Host:");
                    ui.text_edit_singleline(&mut self.add_host)
                        .on_hover_text("Format: username@hostname (e.g., root@dure.com)");
                });

                ui.horizontal(|ui| {
                    ui.label("Port:");
                    ui.text_edit_singleline(&mut self.add_port);
                });

                ui.add_space(8.0);
                ui.label("Authentication:");

                ui.checkbox(&mut self.add_use_password, "Use password");
                if self.add_use_password {
                    ui.horizontal(|ui| {
                        ui.label("Password:");
                        ui.add(egui::TextEdit::singleline(&mut self.add_password).password(true));
                    });
                }

                ui.checkbox(&mut self.add_use_private_key, "Use private key");
                if self.add_use_private_key {
                    ui.horizontal(|ui| {
                        ui.label("Key path:");
                        ui.text_edit_singleline(&mut self.add_private_key_path)
                            .on_hover_text("Path to private key file (e.g., ~/.ssh/id_rsa)");
                    });
                }

                if !self.add_use_password && !self.add_use_private_key {
                    ui.label(
                        egui::RichText::new("Will use SSH agent if no auth method selected")
                            .color(ui.visuals().weak_text_color()),
                    );
                }

                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.show_add_dialog = false;
                    }

                    if ui.button("Add").clicked() {
                        if self.add_host.is_empty() {
                            self.load_error =
                                Some("Host is required (format: username@hostname)".to_string());
                        } else if !self.add_host.contains('@') {
                            self.load_error =
                                Some("Invalid host format. Use: username@hostname".to_string());
                        } else {
                            self.execute_add_host();
                            self.show_add_dialog = false;
                        }
                    }
                });
            });

        if !open {
            self.show_add_dialog = false;
        }
    }

    fn execute_add_host(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            match load_config() {
                Ok((mut app_config, config_path)) => {
                    // Check if host already exists
                    if app_config.ssh_hosts.iter().any(|h| h.host == self.add_host) {
                        self.load_error =
                            Some(format!("SSH host '{}' already exists", self.add_host));
                        return;
                    }

                    // Parse port
                    let port = match self.add_port.parse::<u16>() {
                        Ok(p) => p,
                        Err(_) => {
                            self.load_error = Some("Invalid port number".to_string());
                            return;
                        }
                    };

                    // Create new SSH host config
                    let ssh_host = SshHostConfig {
                        host: self.add_host.clone(),
                        password: if self.add_use_password && !self.add_password.is_empty() {
                            Some(self.add_password.clone())
                        } else {
                            None
                        },
                        private_key_path: if self.add_use_private_key
                            && !self.add_private_key_path.is_empty()
                        {
                            Some(shellexpand::tilde(&self.add_private_key_path).to_string())
                        } else {
                            None
                        },
                        port,
                        initialized: false,
                        last_status: None,
                    };

                    // Add to config
                    app_config.ssh_hosts.push(ssh_host);

                    // Save config
                    match app_config.save(&config_path) {
                        Ok(_) => {
                            // Record audit event
                            let _ = audit::push_gui("system", "desktop", "ssh add", &self.add_host);

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

    fn execute_delete_host(&mut self, host: String) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            match load_config() {
                Ok((mut app_config, config_path)) => {
                    // Find and remove host
                    if let Some(idx) = app_config.ssh_hosts.iter().position(|h| h.host == host) {
                        app_config.ssh_hosts.remove(idx);

                        // Save config
                        match app_config.save(&config_path) {
                            Ok(_) => {
                                // Record audit event
                                let _ = audit::push_gui("system", "desktop", "ssh del", &host);

                                self.loaded = false; // Trigger reload
                                self.selected_row = None;
                                self.load_error = None;
                            }
                            Err(e) => {
                                self.load_error = Some(format!("Failed to save config: {e}"));
                            }
                        }
                    } else {
                        self.load_error = Some(format!("SSH host '{}' not found", host));
                    }
                }
                Err(e) => {
                    self.load_error = Some(format!("Failed to load config: {e}"));
                }
            }
        }
    }

    fn execute_init_host(&mut self, host: String) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.init_in_progress = true;
            self.init_host = Some(host.clone());
            self.init_progress_log.clear();
            self.init_progress_log
                .push(format!("Initializing SSH host: {}", host));

            match load_config() {
                Ok((mut app_config, config_path)) => {
                    // Find host config
                    if let Some(host_config) =
                        app_config.ssh_hosts.iter_mut().find(|h| h.host == host)
                    {
                        // Run initialization
                        match ssh::initialize_host(host_config) {
                            Ok(progress_log) => {
                                self.init_progress_log.extend(progress_log);
                                host_config.initialized = true;

                                // Save config
                                match app_config.save(&config_path) {
                                    Ok(_) => {
                                        self.loaded = false; // Trigger reload
                                        self.init_progress_log
                                            .push("✓ Configuration saved".to_string());
                                    }
                                    Err(e) => {
                                        self.init_progress_log
                                            .push(format!("⚠ Failed to save config: {e}"));
                                    }
                                }
                            }
                            Err(e) => {
                                self.init_progress_log
                                    .push(format!("✗ Initialization failed: {e}"));
                            }
                        }
                    } else {
                        self.init_progress_log
                            .push(format!("✗ SSH host '{}' not found", host));
                    }
                }
                Err(e) => {
                    self.init_progress_log
                        .push(format!("✗ Failed to load config: {e}"));
                }
            }
        }
    }

    fn render_init_progress(&mut self, ui: &mut egui::Ui) {
        ui.add_space(12.0);
        ui.separator();
        ui.heading("Initialization Progress");

        if let Some(host) = &self.init_host {
            ui.label(format!("Host: {}", host));
        }

        ui.add_space(8.0);

        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                for log in &self.init_progress_log {
                    ui.label(log);
                }
            });

        ui.add_space(8.0);

        if ui.button("Close").clicked() {
            self.init_in_progress = false;
            self.init_host = None;
            self.init_progress_log.clear();
        }
    }
}
