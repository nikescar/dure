//! Tab modules for the main application UI

pub mod channel;
pub mod client;
pub mod dm;
pub mod email;
pub mod members;
#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
pub mod ns;
pub mod orders;
#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
pub mod platform;
pub mod products;
pub mod roles;
pub mod site;
#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
pub mod ssh;

/// Enum representing all available tabs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Tab {
    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    Platform,
    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    Ssh,
    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    Ns,
    Site,
    Roles,
    Members,
    Channel,
    DM,
    Products,
    Orders,
    Email,
    Client,
}

impl Tab {
    /// Get the display name for the tab
    pub fn name(&self) -> &'static str {
        match self {
            #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
            Tab::Platform => "Platform",
            #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
            Tab::Ssh => "SSH",
            #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
            Tab::Ns => "Nameserver",
            Tab::Site => "Site",
            Tab::Roles => "Roles",
            Tab::Members => "Members",
            Tab::Channel => "Channel",
            Tab::DM => "DM",
            Tab::Products => "Products",
            Tab::Orders => "Orders",
            Tab::Email => "Email",
            Tab::Client => "Client",
        }
    }

    /// Get all tabs in order
    pub fn all() -> Vec<Tab> {
        vec![
            #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
            Tab::Platform,
            #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
            Tab::Ns,
            #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
            Tab::Ssh,
            Tab::Site,
            Tab::Roles,
            Tab::Members,
            Tab::Channel,
            Tab::DM,
            Tab::Products,
            Tab::Orders,
            Tab::Email,
            Tab::Client,
        ]
    }
}
