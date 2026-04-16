// internet <-- api --> <-- calc --> ui

pub mod ehttp_cache;

#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
pub mod desktop;

#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
pub mod gcp_oauth;
