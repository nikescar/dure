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
    /// Cached SSH host rows (host, port, domain, auth type, status)
    #[cfg_attr(feature = "serde", serde(skip))]
    rows: Vec<[String; 5]>,
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
    #[cfg_attr(feature = "serde", serde(skip))]
    init_promise: Option<poll_promise::Promise<Result<Vec<String>, String>>>,

    // Connection test state
    #[cfg_attr(feature = "serde", serde(skip))]
    test_in_progress: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    test_promise: Option<poll_promise::Promise<Result<ssh::SshConnectionResult, String>>>,
    #[cfg_attr(feature = "serde", serde(skip))]
    test_result: Option<Result<String, String>>,

    // Domain selection dialog state
    #[cfg_attr(feature = "serde", serde(skip))]
    show_domain_dialog: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    available_domains: Vec<String>,
    #[cfg_attr(feature = "serde", serde(skip))]
    selected_domain_for_edit: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip))]
    editing_host_index: Option<usize>,

    // Check server state
    #[cfg_attr(feature = "serde", serde(skip))]
    check_server_in_progress: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    check_server_promise: Option<poll_promise::Promise<Result<(String, String), String>>>,
    #[cfg_attr(feature = "serde", serde(skip))]
    check_server_result: Option<Result<(String, String), String>>,

    // Install server state
    #[cfg_attr(feature = "serde", serde(skip))]
    install_server_in_progress: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    install_server_promise: Option<poll_promise::Promise<Result<Vec<String>, String>>>,
    #[cfg_attr(feature = "serde", serde(skip))]
    install_server_result: Option<Result<Vec<String>, String>>,
}

impl Default for SshTab {
    fn default() -> Self {
        let spreadsheet = {
            let columns = vec![
                text_column("Host", 250.0),
                text_column("Port", 80.0),
                text_column("Domain", 150.0),
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
            init_promise: None,
            test_in_progress: false,
            test_promise: None,
            test_result: None,
            show_domain_dialog: false,
            available_domains: Vec::new(),
            selected_domain_for_edit: None,
            editing_host_index: None,
            check_server_in_progress: false,
            check_server_promise: None,
            check_server_result: None,
            install_server_in_progress: false,
            install_server_promise: None,
            install_server_result: None,
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

            // Check Connection button - enabled only when a row is selected
            let check_button = MaterialButton::outlined("Ping");
            let check_button = if has_selection && !self.test_in_progress {
                check_button
            } else {
                check_button.enabled(false)
            };

            if ui.add(check_button).clicked() {
                if let Some(idx) = selected_row_idx {
                    if idx < self.rows.len() {
                        let host = self.rows[idx][0].clone();
                        self.execute_test_connection(host);
                    }
                }
            }

            // Domain button - enabled only when a row is selected
            let domain_button = MaterialButton::outlined("Domain");
            let domain_button = if has_selection {
                domain_button
            } else {
                domain_button.enabled(false)
            };

            if ui.add(domain_button).clicked() {
                if let Some(idx) = selected_row_idx {
                    if idx < self.rows.len() {
                        self.show_domain_selection_dialog(idx);
                    }
                }
            }

            // Check Server button - enabled only when a row is selected
            let check_server_button = MaterialButton::outlined("Check Server");
            let check_server_button = if has_selection && !self.check_server_in_progress {
                check_server_button
            } else {
                check_server_button.enabled(false)
            };

            if ui.add(check_server_button).clicked() {
                if let Some(idx) = selected_row_idx {
                    if idx < self.rows.len() {
                        let host = self.rows[idx][0].clone();
                        self.execute_check_server(host);
                    }
                }
            }

            // Install Dure Server button - enabled only when a row is selected with domain
            let install_button = MaterialButton::filled("Install Dure Server");
            let install_button = if has_selection && !self.install_server_in_progress {
                install_button
            } else {
                install_button.enabled(false)
            };

            if ui.add(install_button).clicked() {
                if let Some(idx) = selected_row_idx {
                    if idx < self.rows.len() {
                        let host = self.rows[idx][0].clone();
                        let domain = self.rows[idx][2].clone();
                        if domain == "-" || domain.is_empty() {
                            self.load_error = Some("Please set a domain for this host first".to_string());
                        } else {
                            self.execute_install_server(host);
                        }
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

        // Poll for connection test completion
        self.poll_connection_test();

        // Show connection test result
        if let Some(result) = self.test_result.clone() {
            self.render_test_result(ui.ctx(), &result);
        }

        // Domain selection dialog
        if self.show_domain_dialog {
            self.render_domain_dialog(ui.ctx());
        }

        // Poll for check server completion
        self.poll_check_server();

        // Show check server result
        if let Some(result) = self.check_server_result.clone() {
            self.render_check_server_result(ui.ctx(), &result);
        }

        // Poll for install server completion
        self.poll_install_server();

        // Install progress display
        if self.install_server_in_progress {
            self.render_install_progress(ui);
        }

        // Show install server result
        if let Some(result) = self.install_server_result.clone() {
            self.render_install_server_result(ui.ctx(), &result);
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

                        let domain = host_config.domain.as_deref().unwrap_or("-");

                        self.rows.push([
                            host_config.host.clone(),
                            host_config.port.to_string(),
                            domain.to_string(),
                            auth_type.to_string(),
                            status.to_string(),
                        ]);

                        data_rows.push(vec![
                            host_config.host.clone(),
                            host_config.port.to_string(),
                            domain.to_string(),
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
                            text_column("Domain", 150.0),
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
                        keyring_domain: None,
                        port,
                        domain: None,
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

                            eprintln!("✓ SSH host added, refreshing spreadsheet");
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

                                eprintln!("✓ SSH host deleted, refreshing spreadsheet");
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

            // Load config and get host config
            let host_config_clone = match load_config() {
                Ok((app_config, _)) => {
                    app_config
                        .ssh_hosts
                        .iter()
                        .find(|h| h.host == host)
                        .cloned()
                }
                Err(e) => {
                    self.init_progress_log
                        .push(format!("✗ Failed to load config: {e}"));
                    self.init_in_progress = false;
                    return;
                }
            };

            let Some(host_config) = host_config_clone else {
                self.init_progress_log
                    .push(format!("✗ SSH host '{}' not found", host));
                self.init_in_progress = false;
                return;
            };

            // Spawn initialization in background thread
            let promise = poll_promise::Promise::spawn_thread("ssh_init", move || {
                ssh::initialize_host(&host_config)
                    .map_err(|e| format!("{}", e))
            });

            self.init_promise = Some(promise);
        }
    }

    fn execute_test_connection(&mut self, host: String) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.test_in_progress = true;
            self.test_result = None;

            // Load config and get host config
            let host_config_clone = match load_config() {
                Ok((app_config, _)) => {
                    app_config
                        .ssh_hosts
                        .iter()
                        .find(|h| h.host == host)
                        .cloned()
                }
                Err(e) => {
                    self.test_result = Some(Err(format!("Failed to load config: {e}")));
                    self.test_in_progress = false;
                    return;
                }
            };

            let Some(host_config) = host_config_clone else {
                self.test_result = Some(Err(format!("SSH host '{}' not found", host)));
                self.test_in_progress = false;
                return;
            };

            // Spawn connection test in background thread
            let promise = poll_promise::Promise::spawn_thread("ssh_test", move || {
                ssh::test_connection(&host_config)
                    .map_err(|e| format!("{}", e))
            });

            self.test_promise = Some(promise);
        }
    }

    fn poll_connection_test(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(promise) = &self.test_promise {
            if let Some(result) = promise.ready() {
                match result {
                    Ok(conn_result) => {
                        self.test_result = Some(Ok(conn_result.message.clone()));
                    }
                    Err(e) => {
                        self.test_result = Some(Err(e.clone()));
                    }
                }

                self.test_promise = None;
                self.test_in_progress = false;
            }
        }
    }

    fn render_test_result(&mut self, ctx: &egui::Context, result: &Result<String, String>) {
        let mut open = true;

        egui::Window::new("Connection Test Result")
            .open(&mut open)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                match result {
                    Ok(msg) => {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("✓").color(egui::Color32::GREEN).size(20.0));
                            ui.label(egui::RichText::new("Connection successful").strong());
                        });
                        ui.add_space(8.0);
                        ui.label(msg);
                    }
                    Err(msg) => {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("✗").color(egui::Color32::RED).size(20.0));
                            ui.label(egui::RichText::new("Connection failed").strong());
                        });
                        ui.add_space(8.0);
                        ui.colored_label(egui::Color32::RED, msg);
                    }
                }

                ui.add_space(12.0);

                if ui.button("Close").clicked() {
                    self.test_result = None;
                }
            });

        if !open {
            self.test_result = None;
        }
    }

    fn render_init_progress(&mut self, ui: &mut egui::Ui) {
        // Poll for completion
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(promise) = &self.init_promise {
            if let Some(result) = promise.ready() {
                match result {
                    Ok(progress_log) => {
                        self.init_progress_log.extend(progress_log.clone());

                        // Mark host as initialized and save config
                        if let Some(host) = &self.init_host {
                            if let Ok((mut app_config, config_path)) = load_config() {
                                if let Some(host_config) =
                                    app_config.ssh_hosts.iter_mut().find(|h| &h.host == host)
                                {
                                    host_config.initialized = true;

                                    match app_config.save(&config_path) {
                                        Ok(_) => {
                                            eprintln!("✓ SSH host initialized, refreshing spreadsheet");
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
                            }
                        }
                    }
                    Err(e) => {
                        self.init_progress_log
                            .push(format!("✗ Initialization failed: {}", e));
                    }
                }

                self.init_promise = None;
            }
        }

        ui.add_space(12.0);
        ui.separator();
        ui.heading("Initialization Progress");

        if let Some(host) = &self.init_host {
            ui.label(format!("Host: {}", host));
        }

        ui.add_space(8.0);

        // Show spinner if still in progress
        #[cfg(not(target_arch = "wasm32"))]
        if self.init_promise.is_some() {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label("Initialization in progress...");
            });
            ui.add_space(8.0);
        }

        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                for log in &self.init_progress_log {
                    ui.label(log);
                }
            });

        ui.add_space(8.0);

        let can_close = self.init_promise.is_none();
        if ui.add_enabled(can_close, egui::Button::new("Close")).clicked() {
            self.init_in_progress = false;
            self.init_host = None;
            self.init_progress_log.clear();
        }

        if !can_close {
            ui.colored_label(
                egui::Color32::GRAY,
                "Please wait for initialization to complete",
            );
        }
    }

    fn show_domain_selection_dialog(&mut self, host_idx: usize) {
        self.editing_host_index = Some(host_idx);
        self.show_domain_dialog = true;
        self.available_domains.clear();
        self.selected_domain_for_edit = None;

        // Load available domains from ns config
        #[cfg(not(target_arch = "wasm32"))]
        {
            use crate::calc::ns::NsConfig;
            if let Ok(ns_config) = NsConfig::load_from_config_file() {
                // Collect domains from all providers
                for provider_config in ns_config.providers.values() {
                    for domain_entry in &provider_config.domains {
                        self.available_domains.push(domain_entry.domain.clone());
                    }
                }
                // Collect domains from GCP accounts
                for gcp_account in &ns_config.gcp_accounts {
                    for domain_entry in &gcp_account.domains {
                        self.available_domains.push(domain_entry.domain.clone());
                    }
                }
                self.available_domains.sort();
                self.available_domains.dedup();
            }
        }

        // Set current domain if exists
        if host_idx < self.rows.len() {
            let current_domain = &self.rows[host_idx][2];
            if current_domain != "-" {
                self.selected_domain_for_edit = Some(current_domain.clone());
            }
        }
    }

    fn render_domain_dialog(&mut self, ctx: &egui::Context) {
        let mut open = true;

        egui::Window::new("Select Domain")
            .open(&mut open)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.label("Select a domain to associate with this SSH host:");
                ui.add_space(8.0);

                if self.available_domains.is_empty() {
                    ui.colored_label(
                        egui::Color32::YELLOW,
                        "No domains found. Add domains in the Domains tab first.",
                    );
                } else {
                    // Domain selection combo box
                    let current_text = self
                        .selected_domain_for_edit
                        .as_deref()
                        .unwrap_or("(Select a domain)");

                    egui::ComboBox::from_id_salt("domain_selection")
                        .selected_text(current_text)
                        .show_ui(ui, |ui| {
                            for domain in &self.available_domains {
                                ui.selectable_value(
                                    &mut self.selected_domain_for_edit,
                                    Some(domain.clone()),
                                    domain,
                                );
                            }
                        });
                }

                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.show_domain_dialog = false;
                        self.editing_host_index = None;
                        self.selected_domain_for_edit = None;
                    }

                    if ui.button("Clear").clicked() {
                        self.execute_set_domain(None);
                        self.show_domain_dialog = false;
                    }

                    if ui.button("Set").clicked() {
                        if let Some(domain) = &self.selected_domain_for_edit {
                            self.execute_set_domain(Some(domain.clone()));
                        }
                        self.show_domain_dialog = false;
                    }
                });
            });

        if !open {
            self.show_domain_dialog = false;
            self.editing_host_index = None;
            self.selected_domain_for_edit = None;
        }
    }

    fn execute_set_domain(&mut self, domain: Option<String>) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(host_idx) = self.editing_host_index {
                if host_idx < self.rows.len() {
                    let host = self.rows[host_idx][0].clone();

                    match load_config() {
                        Ok((mut app_config, config_path)) => {
                            // Find and update host
                            if let Some(host_config) =
                                app_config.ssh_hosts.iter_mut().find(|h| h.host == host)
                            {
                                host_config.domain = domain.clone();

                                // Save config
                                match app_config.save(&config_path) {
                                    Ok(_) => {
                                        eprintln!("✓ SSH host domain updated, refreshing spreadsheet");
                                        self.loaded = false; // Trigger reload
                                        self.editing_host_index = None;
                                        self.selected_domain_for_edit = None;
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
        }
    }

    fn execute_check_server(&mut self, host: String) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.check_server_in_progress = true;
            self.check_server_result = None;

            // Load config and get host config
            let host_config_clone = match load_config() {
                Ok((app_config, _)) => {
                    app_config
                        .ssh_hosts
                        .iter()
                        .find(|h| h.host == host)
                        .cloned()
                }
                Err(e) => {
                    self.check_server_result = Some(Err(format!("Failed to load config: {e}")));
                    self.check_server_in_progress = false;
                    return;
                }
            };

            let Some(host_config) = host_config_clone else {
                self.check_server_result = Some(Err(format!("SSH host '{}' not found", host)));
                self.check_server_in_progress = false;
                return;
            };

            // Spawn check server in background thread
            let promise = poll_promise::Promise::spawn_thread("check_server", move || {
                // Check nftables
                let nft_result = ssh::execute_ssh_command(&host_config, "nft list ruleset 2>&1")
                    .unwrap_or_else(|e| format!("Error checking nftables: {}", e));

                // Check network connections
                let ss_result = ssh::execute_ssh_command(&host_config, "ss -nltup 2>&1")
                    .unwrap_or_else(|e| format!("Error checking network connections: {}", e));

                Ok((nft_result, ss_result))
            });

            self.check_server_promise = Some(promise);
        }
    }

    fn poll_check_server(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(promise) = &self.check_server_promise {
            if let Some(result) = promise.ready() {
                self.check_server_result = Some(result.clone());
                self.check_server_promise = None;
                self.check_server_in_progress = false;
            }
        }
    }

    fn render_check_server_result(
        &mut self,
        ctx: &egui::Context,
        result: &Result<(String, String), String>,
    ) {
        let mut open = true;

        egui::Window::new("Server Check Results")
            .open(&mut open)
            .resizable(true)
            .default_width(700.0)
            .default_height(500.0)
            .collapsible(false)
            .show(ctx, |ui| {
                match result {
                    Ok((nft_output, ss_output)) => {
                        ui.heading("NFTables Status");
                        ui.add_space(8.0);

                        egui::ScrollArea::vertical()
                            .id_salt("check_server_nft_scroll")
                            .max_height(200.0)
                            .show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(&mut nft_output.as_str())
                                        .code_editor()
                                        .desired_width(f32::INFINITY),
                                );
                            });

                        ui.add_space(12.0);
                        ui.separator();
                        ui.add_space(12.0);

                        ui.heading("Network Connections (ss -nltup)");
                        ui.add_space(8.0);

                        egui::ScrollArea::vertical()
                            .id_salt("check_server_ss_scroll")
                            .max_height(200.0)
                            .show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(&mut ss_output.as_str())
                                        .code_editor()
                                        .desired_width(f32::INFINITY),
                                );
                            });
                    }
                    Err(err) => {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("✗").color(egui::Color32::RED).size(20.0));
                            ui.label(egui::RichText::new("Check failed").strong());
                        });
                        ui.add_space(8.0);
                        ui.colored_label(egui::Color32::RED, err);
                    }
                }

                ui.add_space(12.0);

                if ui.button("Close").clicked() {
                    self.check_server_result = None;
                }
            });

        if !open {
            self.check_server_result = None;
        }
    }

    fn execute_install_server(&mut self, host: String) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.install_server_in_progress = true;
            self.install_server_result = None;

            // Load config and get host config
            let host_config_clone = match load_config() {
                Ok((app_config, _)) => {
                    app_config
                        .ssh_hosts
                        .iter()
                        .find(|h| h.host == host)
                        .cloned()
                }
                Err(e) => {
                    self.install_server_result = Some(Err(format!("Failed to load config: {}", e)));
                    self.install_server_in_progress = false;
                    return;
                }
            };

            let Some(host_config) = host_config_clone else {
                self.install_server_result = Some(Err(format!("SSH host '{}' not found", host)));
                self.install_server_in_progress = false;
                return;
            };

            let domain = match &host_config.domain {
                Some(d) => d.clone(),
                None => {
                    self.install_server_result = Some(Err("No domain configured for this host".to_string()));
                    self.install_server_in_progress = false;
                    return;
                }
            };

            // Spawn install server in background thread
            let promise = poll_promise::Promise::spawn_thread("install_server", move || {
                let mut log = Vec::new();

                // Step 1: Download GitHub zip
                log.push("1. Downloading dure-wasm from GitHub...".to_string());
                let download_cmd = "curl -L -o /tmp/dure-wasm-main.zip https://github.com/nikescar/dure-wasm/archive/refs/heads/main.zip 2>&1";
                match ssh::execute_ssh_command(&host_config, download_cmd) {
                    Ok(output) => log.push(format!("   ✓ Downloaded: {}", output.trim())),
                    Err(e) => {
                        log.push(format!("   ✗ Download failed: {}", e));
                        return Err(format!("Download failed: {}", e));
                    }
                }

                // Step 2: Extract to /tmp/dure-master
                log.push("2. Extracting zip to /tmp/dure-wasm-main...".to_string());
                let extract_cmd = "cd /tmp && unzip -o dure-wasm-main.zip 2>&1";
                match ssh::execute_ssh_command(&host_config, extract_cmd) {
                    Ok(output) => log.push(format!("   ✓ Extracted: {}", output.lines().take(3).collect::<Vec<_>>().join(", "))),
                    Err(e) => {
                        log.push(format!("   ✗ Extract failed: {}", e));
                        return Err(format!("Extract failed: {}", e));
                    }
                }

                // Step 3: Extract dure.tar.gz to /opt/dure
                log.push("3. Extracting dure.tar.gz to /opt/dure...".to_string());
                let extract_tar_cmd = "mkdir -p /opt/dure && tar -xzf /tmp/dure-wasm-main/dure.tar.gz -C /opt/dure 2>&1";
                match ssh::execute_ssh_command(&host_config, extract_tar_cmd) {
                    Ok(output) => log.push(format!("   ✓ Extracted tar.gz: {}", output.trim())),
                    Err(e) => {
                        log.push(format!("   ✗ Extract tar.gz failed: {}", e));
                        return Err(format!("Extract tar.gz failed: {}", e));
                    }
                }

                // Step 4: Remove tar.gz
                log.push("4. Cleaning up tar.gz...".to_string());
                let cleanup_tar_cmd = "rm -f /tmp/dure-wasm-main/dure.tar.gz 2>&1";
                match ssh::execute_ssh_command(&host_config, cleanup_tar_cmd) {
                    Ok(_) => log.push("   ✓ Removed tar.gz".to_string()),
                    Err(e) => {
                        log.push(format!("   ⚠ Cleanup warning: {}", e));
                    }
                }

                // Step 5: Copy files to /opt/dure/serv
                log.push("5. Copying files to /opt/dure/serv...".to_string());
                let copy_cmd = "mkdir -p /opt/dure/serv && cp -r /tmp/dure-wasm-main/* /opt/dure/serv/ 2>&1";
                match ssh::execute_ssh_command(&host_config, copy_cmd) {
                    Ok(output) => log.push(format!("   ✓ Copied files: {}", output.trim())),
                    Err(e) => {
                        log.push(format!("   ✗ Copy failed: {}", e));
                        return Err(format!("Copy failed: {}", e));
                    }
                }

                // Step 6: Run dure wss server
                log.push(format!("6. Starting dure server for domain '{}'...", domain));
                let run_cmd = format!("cd /opt/dure && nohup ./dure wss server {} > /var/log/dure-server.log 2>&1 &", domain);
                match ssh::execute_ssh_command(&host_config, &run_cmd) {
                    Ok(output) => log.push(format!("   ✓ Server started: {}", output.trim())),
                    Err(e) => {
                        log.push(format!("   ✗ Server start failed: {}", e));
                        return Err(format!("Server start failed: {}", e));
                    }
                }

                log.push("".to_string());
                log.push("✓ Dure server installation complete!".to_string());
                log.push(format!("  Server logs: /var/log/dure-server.log"));
                log.push(format!("  Domain: {}", domain));

                Ok(log)
            });

            self.install_server_promise = Some(promise);
        }
    }

    fn poll_install_server(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(promise) = &self.install_server_promise {
            if let Some(result) = promise.ready() {
                self.install_server_result = Some(result.clone());
                self.install_server_promise = None;
                self.install_server_in_progress = false;
            }
        }
    }

    fn render_install_server_result(
        &mut self,
        ctx: &egui::Context,
        result: &Result<Vec<String>, String>,
    ) {
        let mut open = true;

        egui::Window::new("Install Dure Server Result")
            .open(&mut open)
            .resizable(true)
            .default_width(600.0)
            .default_height(400.0)
            .collapsible(false)
            .show(ctx, |ui| {
                match result {
                    Ok(log) => {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("✓").color(egui::Color32::GREEN).size(20.0));
                            ui.label(egui::RichText::new("Installation completed").strong());
                        });
                        ui.add_space(8.0);

                        egui::ScrollArea::vertical()
                            .id_salt("install_server_log_scroll")
                            .max_height(300.0)
                            .show(ui, |ui| {
                                for line in log {
                                    if line.starts_with("   ✓") {
                                        ui.colored_label(egui::Color32::GREEN, line);
                                    } else if line.starts_with("   ✗") {
                                        ui.colored_label(egui::Color32::RED, line);
                                    } else if line.starts_with("   ⚠") {
                                        ui.colored_label(egui::Color32::YELLOW, line);
                                    } else {
                                        ui.label(line);
                                    }
                                }
                            });
                    }
                    Err(err) => {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("✗").color(egui::Color32::RED).size(20.0));
                            ui.label(egui::RichText::new("Installation failed").strong());
                        });
                        ui.add_space(8.0);
                        ui.colored_label(egui::Color32::RED, err);
                    }
                }

                ui.add_space(12.0);

                if ui.button("Close").clicked() {
                    self.install_server_result = None;
                }
            });

        if !open {
            self.install_server_result = None;
        }
    }
}
