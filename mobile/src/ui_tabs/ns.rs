//! NS tab - Domain and DNS record management

use eframe::egui;
use egui_material3::spreadsheet::{text_column, MaterialSpreadsheet};
use egui_material3::MaterialButton;

#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
use crate::calc::audit;
#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
use crate::calc::ns::{apply_record, NsConfig, RecordType};
use directories::ProjectDirs;
use std::path::PathBuf;

/// NS tab state
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct NsTab {
    #[cfg_attr(feature = "serde", serde(skip))]
    selected_row: Option<usize>,

    /// Cached domain/record rows (domain, provider, record count)
    #[cfg_attr(feature = "serde", serde(skip))]
    domain_rows: Vec<[String; 3]>,

    /// Cached records for selected domain (type, value)
    #[cfg_attr(feature = "serde", serde(skip))]
    record_rows: Vec<[String; 2]>,

    #[cfg_attr(feature = "serde", serde(skip))]
    loaded: bool,

    #[cfg_attr(feature = "serde", serde(skip))]
    load_error: Option<String>,

    // Spreadsheets
    #[cfg_attr(feature = "serde", serde(skip))]
    domain_spreadsheet: Option<MaterialSpreadsheet>,

    #[cfg_attr(feature = "serde", serde(skip))]
    record_spreadsheet: Option<MaterialSpreadsheet>,

    row_selection_enabled: bool,

    // Selected domain for record view
    #[cfg_attr(feature = "serde", serde(skip))]
    selected_domain: Option<String>,

    // Add domain dialog
    #[cfg_attr(feature = "serde", serde(skip))]
    show_add_domain_dialog: bool,

    #[cfg_attr(feature = "serde", serde(skip))]
    add_domain: String,

    #[cfg_attr(feature = "serde", serde(skip))]
    add_provider: String,

    #[cfg_attr(feature = "serde", serde(skip))]
    add_token: String,

    // Add record dialog
    #[cfg_attr(feature = "serde", serde(skip))]
    show_add_record_dialog: bool,

    #[cfg_attr(feature = "serde", serde(skip))]
    add_record_type: String,

    #[cfg_attr(feature = "serde", serde(skip))]
    add_record_value: String,

    #[cfg_attr(feature = "serde", serde(skip))]
    add_record_apply: bool,

    // Progress/status
    #[cfg_attr(feature = "serde", serde(skip))]
    progress_log: Vec<String>,
}

impl Default for NsTab {
    fn default() -> Self {
        let domain_spreadsheet = {
            let columns = vec![
                text_column("Domain", 250.0),
                text_column("Provider", 120.0),
                text_column("Records", 80.0),
            ];

            MaterialSpreadsheet::new("ns_domain_spreadsheet", columns)
                .ok()
                .map(|mut s| {
                    s.set_striped(true);
                    s.set_row_selection_enabled(true);
                    s.set_allow_selection(true);
                    s
                })
        };

        let record_spreadsheet = {
            let columns = vec![text_column("Type", 100.0), text_column("Value", 300.0)];

            MaterialSpreadsheet::new("ns_record_spreadsheet", columns)
                .ok()
                .map(|mut s| {
                    s.set_striped(true);
                    s.set_row_selection_enabled(true);
                    s.set_allow_selection(true);
                    s
                })
        };

        Self {
            selected_row: None,
            domain_rows: Vec::new(),
            record_rows: Vec::new(),
            loaded: false,
            load_error: None,
            domain_spreadsheet,
            record_spreadsheet,
            row_selection_enabled: true,
            selected_domain: None,
            show_add_domain_dialog: false,
            add_domain: String::new(),
            add_provider: "cloudflare".to_string(),
            add_token: String::new(),
            show_add_record_dialog: false,
            add_record_type: "a".to_string(),
            add_record_value: String::new(),
            add_record_apply: true,
            progress_log: Vec::new(),
        }
    }
}

/// Get config file path
#[cfg(not(target_arch = "wasm32"))]
fn get_config_path() -> Result<PathBuf, String> {
    let proj_dirs = ProjectDirs::from("com", "dure", "dure")
        .ok_or_else(|| "Failed to get project directories".to_string())?;
    Ok(proj_dirs.config_dir().join("config.yml"))
}

/// Load NS config from YAML
#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
fn load_ns_config() -> Result<NsConfig, String> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        return Ok(NsConfig::default());
    }

    let yaml = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config: {}", e))?;

    #[derive(serde::Deserialize)]
    struct FullConfig {
        #[serde(default)]
        ns: NsConfig,
    }

    match serde_yaml::from_str::<FullConfig>(&yaml) {
        Ok(full_config) => Ok(full_config.ns),
        Err(_) => Ok(NsConfig::default()),
    }
}

/// Save NS config to YAML
#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
fn save_ns_config(ns_config: &NsConfig) -> Result<(), String> {
    let config_path = get_config_path()?;

    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config dir: {}", e))?;
    }

    let mut full_config = if config_path.exists() {
        let yaml = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config: {}", e))?;
        serde_yaml::from_str::<serde_yaml::Value>(&yaml)
            .unwrap_or_else(|_| serde_yaml::Value::Mapping(Default::default()))
    } else {
        serde_yaml::Value::Mapping(Default::default())
    };

    let ns_value = serde_yaml::to_value(ns_config)
        .map_err(|e| format!("Failed to serialize NS config: {}", e))?;

    if let serde_yaml::Value::Mapping(ref mut map) = full_config {
        map.insert(serde_yaml::Value::String("ns".to_string()), ns_value);
    }

    let yaml = serde_yaml::to_string(&full_config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    std::fs::write(&config_path, yaml).map_err(|e| format!("Failed to write config: {}", e))?;

    Ok(())
}

impl NsTab {
    /// Render the NS tab UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Nameserver Management");
        ui.add_space(4.0);
        ui.label("Manage domains and DNS records (A, AAAA, TXT, SSHFP) for Cloudflare, Google Cloud DNS, DuckDNS, and Porkbun.");
        ui.add_space(8.0);

        // Load data on first render
        if !self.loaded {
            self.load_data();
        }

        // Show error if any
        if let Some(ref error) = self.load_error {
            ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
            ui.add_space(8.0);
        }

        // Domain management section
        ui.horizontal(|ui| {
            ui.label("Domains:");
            ui.add_space(8.0);

            if ui.add(MaterialButton::filled("Add Domain")).clicked() {
                self.show_add_domain_dialog = true;
                self.add_domain.clear();
                self.add_token.clear();
            }

            let domain_selected = self
                .domain_spreadsheet
                .as_ref()
                .and_then(|s| s.get_selected_row())
                .is_some();

            let delete_button = MaterialButton::outlined("Delete Domain");
            let delete_button = if domain_selected {
                delete_button
            } else {
                delete_button.enabled(false)
            };

            if ui.add(delete_button).clicked() {
                if let Some(idx) = self
                    .domain_spreadsheet
                    .as_ref()
                    .and_then(|s| s.get_selected_row())
                {
                    if idx < self.domain_rows.len() {
                        let domain = self.domain_rows[idx][0].clone();
                        self.execute_delete_domain(&domain);
                    }
                }
            }

            if ui.add(MaterialButton::text("Refresh")).clicked() {
                self.load_data();
            }
        });

        ui.add_space(8.0);

        // Domain spreadsheet
        if let Some(ref mut spreadsheet) = self.domain_spreadsheet {
            ui.push_id("domain_spreadsheet_container", |ui| {
                spreadsheet.show(ui);
            });

            // Check for selection change
            if let Some(selected_idx) = spreadsheet.get_selected_row() {
                if selected_idx < self.domain_rows.len() {
                    let domain = self.domain_rows[selected_idx][0].clone();
                    if self.selected_domain.as_ref() != Some(&domain) {
                        self.selected_domain = Some(domain);
                        self.load_records();
                    }
                }
            }
        }

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        // Records section
        if let Some(domain) = self.selected_domain.clone() {
            ui.horizontal(|ui| {
                ui.label(format!("Records for {}:", domain));
                ui.add_space(8.0);

                if ui.add(MaterialButton::filled("Add Record")).clicked() {
                    self.show_add_record_dialog = true;
                    self.add_record_value.clear();
                }

                let record_selected = self
                    .record_spreadsheet
                    .as_ref()
                    .and_then(|s| s.get_selected_row())
                    .is_some();

                let delete_record_button = MaterialButton::outlined("Delete Record");
                let delete_record_button = if record_selected {
                    delete_record_button
                } else {
                    delete_record_button.enabled(false)
                };

                if ui.add(delete_record_button).clicked() {
                    if let Some(idx) = self
                        .record_spreadsheet
                        .as_ref()
                        .and_then(|s| s.get_selected_row())
                    {
                        if idx < self.record_rows.len() {
                            let record_type = self.record_rows[idx][0].clone();
                            let value = self.record_rows[idx][1].clone();
                            self.execute_delete_record(&record_type, &value);
                        }
                    }
                }
            });

            ui.add_space(8.0);

            if let Some(ref mut spreadsheet) = self.record_spreadsheet {
                ui.push_id("record_spreadsheet_container", |ui| {
                    spreadsheet.show(ui);
                });
            }
        } else {
            ui.label("Select a domain to view and manage its records");
        }

        // Progress log
        if !self.progress_log.is_empty() {
            ui.add_space(16.0);
            ui.separator();
            ui.add_space(8.0);
            ui.label("Progress:");
            egui::ScrollArea::vertical()
                .max_height(150.0)
                .show(ui, |ui| {
                    for line in &self.progress_log {
                        ui.label(line);
                    }
                });
        }

        // Dialogs
        self.show_add_domain_dialog(ui.ctx());
        self.show_add_record_dialog(ui.ctx());
    }

    /// Load domain data from config
    fn load_data(&mut self) {
        self.loaded = true;
        self.load_error = None;

        #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
        {
            match load_ns_config() {
                Ok(config) => {
                    self.domain_rows = config
                        .domains
                        .iter()
                        .map(|d| {
                            [
                                d.domain.clone(),
                                d.provider.clone(),
                                d.records.len().to_string(),
                            ]
                        })
                        .collect();

                    // Recreate spreadsheet with fresh data to avoid duplicates
                    let columns = vec![
                        text_column("Domain", 250.0),
                        text_column("Provider", 120.0),
                        text_column("Records", 80.0),
                    ];

                    self.domain_spreadsheet =
                        MaterialSpreadsheet::new("ns_domain_spreadsheet", columns)
                            .ok()
                            .map(|mut s| {
                                s.set_striped(true);
                                s.set_row_selection_enabled(true);
                                s.set_allow_selection(true);
                                s.init_with_data(
                                    self.domain_rows
                                        .iter()
                                        .map(|row| row.iter().cloned().collect())
                                        .collect(),
                                );
                                s
                            });
                }
                Err(e) => {
                    self.load_error = Some(e);
                }
            }
        }

        #[cfg(any(target_os = "android", target_arch = "wasm32"))]
        {
            self.load_error = Some("NS management not supported on this platform".to_string());
        }
    }

    /// Load records for selected domain
    fn load_records(&mut self) {
        #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
        {
            if let Some(ref domain) = self.selected_domain {
                match load_ns_config() {
                    Ok(config) => {
                        if let Some(domain_entry) = config.get_domain(domain) {
                            self.record_rows = domain_entry
                                .records
                                .iter()
                                .map(|r| [r.record_type.as_str().to_uppercase(), r.value.clone()])
                                .collect();

                            // Recreate spreadsheet with fresh data to avoid duplicates
                            let columns =
                                vec![text_column("Type", 100.0), text_column("Value", 300.0)];

                            self.record_spreadsheet =
                                MaterialSpreadsheet::new("ns_record_spreadsheet", columns)
                                    .ok()
                                    .map(|mut s| {
                                        s.set_striped(true);
                                        s.set_row_selection_enabled(true);
                                        s.set_allow_selection(true);
                                        s.init_with_data(
                                            self.record_rows
                                                .iter()
                                                .map(|row| row.iter().cloned().collect())
                                                .collect(),
                                        );
                                        s
                                    });
                        }
                    }
                    Err(e) => {
                        self.add_progress(format!("Error loading records: {}", e));
                    }
                }
            }
        }
    }

    /// Show add domain dialog
    fn show_add_domain_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_add_domain_dialog {
            return;
        }

        let mut open = true;
        egui::Window::new("Add Domain")
            .id(egui::Id::new("ns_add_domain_dialog"))
            .open(&mut open)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Domain name:");
                ui.text_edit_singleline(&mut self.add_domain);

                ui.add_space(8.0);
                ui.label("Provider:");
                egui::ComboBox::from_id_salt("provider_combo")
                    .selected_text(&self.add_provider)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.add_provider,
                            "cloudflare".to_string(),
                            "Cloudflare",
                        );
                        ui.selectable_value(
                            &mut self.add_provider,
                            "gcloud".to_string(),
                            "Google Cloud DNS",
                        );
                        ui.selectable_value(
                            &mut self.add_provider,
                            "duckdns".to_string(),
                            "DuckDNS",
                        );
                        ui.selectable_value(
                            &mut self.add_provider,
                            "porkbun".to_string(),
                            "Porkbun",
                        );
                    });

                ui.add_space(8.0);
                ui.label("API Token:");
                ui.add(egui::TextEdit::singleline(&mut self.add_token).password(true));

                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    if ui.add(MaterialButton::filled("Add")).clicked() {
                        self.execute_add_domain();
                        self.show_add_domain_dialog = false;
                    }

                    if ui.add(MaterialButton::text("Cancel")).clicked() {
                        self.show_add_domain_dialog = false;
                    }
                });
            });

        if !open {
            self.show_add_domain_dialog = false;
        }
    }

    /// Show add record dialog
    fn show_add_record_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_add_record_dialog {
            return;
        }

        let mut open = true;
        egui::Window::new("Add DNS Record")
            .id(egui::Id::new("ns_add_record_dialog"))
            .open(&mut open)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Record type:");
                egui::ComboBox::from_id_salt("record_type_combo")
                    .selected_text(self.add_record_type.to_uppercase())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.add_record_type, "a".to_string(), "A (IPv4)");
                        ui.selectable_value(
                            &mut self.add_record_type,
                            "aaaa".to_string(),
                            "AAAA (IPv6)",
                        );
                        ui.selectable_value(
                            &mut self.add_record_type,
                            "txt".to_string(),
                            "TXT (Text)",
                        );
                        ui.selectable_value(
                            &mut self.add_record_type,
                            "sshfp".to_string(),
                            "SSHFP (SSH Fingerprint)",
                        );
                    });

                ui.add_space(8.0);
                ui.label("Value:");
                ui.text_edit_singleline(&mut self.add_record_value);

                ui.add_space(8.0);
                ui.checkbox(
                    &mut self.add_record_apply,
                    "Apply to DNS provider immediately",
                );

                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    if ui.add(MaterialButton::filled("Add")).clicked() {
                        self.execute_add_record();
                        self.show_add_record_dialog = false;
                    }

                    if ui.add(MaterialButton::text("Cancel")).clicked() {
                        self.show_add_record_dialog = false;
                    }
                });
            });

        if !open {
            self.show_add_record_dialog = false;
        }
    }

    /// Execute add domain
    fn execute_add_domain(&mut self) {
        #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
        {
            let domain = self.add_domain.trim().to_string();
            let provider = self.add_provider.clone();
            let token = self.add_token.clone();

            if domain.is_empty() || token.is_empty() {
                self.add_progress("Error: Domain and token are required".to_string());
                return;
            }

            match load_ns_config() {
                Ok(mut config) => {
                    match config.add_domain(domain.clone(), provider.clone(), token) {
                        Ok(_) => {
                            match save_ns_config(&config) {
                                Ok(_) => {
                                    // Record audit event
                                    let _ = audit::push_gui("system", "desktop", "ns add", &domain);

                                    self.add_progress(format!(
                                        "✓ Added domain: {} ({})",
                                        domain, provider
                                    ));
                                    self.load_data();
                                }
                                Err(e) => {
                                    self.add_progress(format!("Error saving config: {}", e));
                                }
                            }
                        }
                        Err(e) => {
                            self.add_progress(format!("Error: {}", e));
                        }
                    }
                }
                Err(e) => {
                    self.add_progress(format!("Error loading config: {}", e));
                }
            }
        }
    }

    /// Execute delete domain
    fn execute_delete_domain(&mut self, domain: &str) {
        #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
        {
            match load_ns_config() {
                Ok(mut config) => {
                    match config.remove_domain(domain) {
                        Ok(_) => {
                            match save_ns_config(&config) {
                                Ok(_) => {
                                    // Record audit event
                                    let _ = audit::push_gui("system", "desktop", "ns del", domain);

                                    self.add_progress(format!("✓ Deleted domain: {}", domain));
                                    if self.selected_domain.as_ref() == Some(&domain.to_string()) {
                                        self.selected_domain = None;
                                        self.record_rows.clear();
                                    }
                                    self.load_data();
                                }
                                Err(e) => {
                                    self.add_progress(format!("Error saving config: {}", e));
                                }
                            }
                        }
                        Err(e) => {
                            self.add_progress(format!("Error: {}", e));
                        }
                    }
                }
                Err(e) => {
                    self.add_progress(format!("Error loading config: {}", e));
                }
            }
        }
    }

    /// Execute add record
    fn execute_add_record(&mut self) {
        #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
        {
            let value = self.add_record_value.trim().to_string();
            if value.is_empty() {
                self.add_progress("Error: Value is required".to_string());
                return;
            }

            if let Some(domain) = self.selected_domain.clone() {
                let record_type = RecordType::from_str(&self.add_record_type);
                if record_type.is_none() {
                    self.add_progress("Error: Invalid record type".to_string());
                    return;
                }
                let record_type = record_type.unwrap();

                match load_ns_config() {
                    Ok(mut config) => {
                        match config.add_record(&domain, record_type.clone(), value.clone()) {
                            Ok(_) => {
                                match save_ns_config(&config) {
                                    Ok(_) => {
                                        // Record audit event
                                        let record_desc = format!(
                                            "{} {} {}",
                                            domain, self.add_record_type, value
                                        );
                                        let _ = audit::push_gui(
                                            "system",
                                            "desktop",
                                            "ns insert",
                                            &record_desc,
                                        );

                                        self.add_progress(format!(
                                            "✓ Added record: {} {} -> {}",
                                            domain,
                                            self.add_record_type.to_uppercase(),
                                            value
                                        ));

                                        // Apply to DNS provider if requested
                                        if self.add_record_apply {
                                            if let Some(domain_entry) = config.get_domain(&domain) {
                                                if let Some(record) =
                                                    domain_entry.records.iter().find(|r| {
                                                        r.record_type == record_type
                                                            && r.value == value
                                                    })
                                                {
                                                    match apply_record(domain_entry, record) {
                                                        Ok(_) => {
                                                            self.add_progress(
                                                                "✓ Applied to DNS provider"
                                                                    .to_string(),
                                                            );
                                                        }
                                                        Err(e) => {
                                                            self.add_progress(format!(
                                                                "⚠ Failed to apply: {}",
                                                                e
                                                            ));
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        self.load_records();
                                        self.load_data(); // Refresh record count
                                    }
                                    Err(e) => {
                                        self.add_progress(format!("Error saving config: {}", e));
                                    }
                                }
                            }
                            Err(e) => {
                                self.add_progress(format!("Error: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        self.add_progress(format!("Error loading config: {}", e));
                    }
                }
            }
        }
    }

    /// Execute delete record
    fn execute_delete_record(&mut self, record_type: &str, value: &str) {
        #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
        {
            if let Some(ref domain) = self.selected_domain {
                let rec_type = RecordType::from_str(&record_type.to_lowercase());
                if rec_type.is_none() {
                    self.add_progress("Error: Invalid record type".to_string());
                    return;
                }

                match load_ns_config() {
                    Ok(mut config) => {
                        match config.remove_record(domain, rec_type.unwrap(), value) {
                            Ok(_) => {
                                match save_ns_config(&config) {
                                    Ok(_) => {
                                        // Record audit event
                                        let record_desc =
                                            format!("{} {} {}", domain, record_type, value);
                                        let _ = audit::push_gui(
                                            "system",
                                            "desktop",
                                            "ns remove",
                                            &record_desc,
                                        );

                                        self.add_progress(format!(
                                            "✓ Deleted record: {} {} {}",
                                            domain, record_type, value
                                        ));
                                        self.load_records();
                                        self.load_data(); // Refresh record count
                                    }
                                    Err(e) => {
                                        self.add_progress(format!("Error saving config: {}", e));
                                    }
                                }
                            }
                            Err(e) => {
                                self.add_progress(format!("Error: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        self.add_progress(format!("Error loading config: {}", e));
                    }
                }
            }
        }
    }

    fn add_progress(&mut self, message: String) {
        self.progress_log.push(message);
        // Keep only last 20 messages
        if self.progress_log.len() > 20 {
            self.progress_log.remove(0);
        }
    }
}
