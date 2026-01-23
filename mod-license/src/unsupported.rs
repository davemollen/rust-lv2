#![allow(clippy::needless_doctest_main)]

/// MOD License API to handle licensing and copy-protection.
///
/// Unsupported without the "mod_license" feature.
///
/// Add the "mod_license" feature to the rust-lv2 dependency in your Cargo.toml to enable it.
pub struct ModLicenseApi;

impl ModLicenseApi {
    /// Unsupported without the "mod_license" feature.
    ///
    /// Add the "mod_license" feature to the rust-lv2 dependency in your Cargo.toml to enable it.
    pub fn new() -> Self {
        Self
    }

    /// Unsupported without the "mod_license" feature.
    ///
    /// Add the "mod_license" feature to the rust-lv2 dependency in your Cargo.toml to enable it.
    pub fn run_begin(&mut self, _sample_count: u32) {
        // Do nothing in unsupported version
    }

    /// Unsupported without the "mod_license" feature.
    ///
    /// Add the "mod_license" feature to the rust-lv2 dependency in your Cargo.toml to enable it.
    pub fn run_silence<const N: usize>(&self, _output_buffers: [*mut f32; N], _sample_count: u32) {
        // Do nothing in unsupported version
    }

    /// Unsupported without the "mod_license" feature.
    ///
    /// Add the "mod_license" feature to the rust-lv2 dependency in your Cargo.toml to enable it.
    pub fn version() -> Option<String> {
        None
    }
}

/// Link the mod license API static library.
///
/// Make sure you have rust-lv2 as a build dependency in your Cargo.toml and enable the "mod_license" feature.
/// ```toml
/// [build-dependencies]
/// lv2 = { git = "https://github.com/davemollen/rust-lv2.git", branch = "master", features = [
///     "mod_license"
/// ] }
/// ```
///
/// Use this function within build.rs to link the MOD License API library on build:
/// ```
/// use mod_license::*;
///
/// fn main() {
///     mod_license_api_linker::link_library();
/// }
/// ```
pub mod mod_license_api_linker {
    /// Unsupported without the "mod_license" feature.
    ///
    /// Add the "mod_license" feature to the rust-lv2 build-dependency in your Cargo.toml to enable it.
    pub fn link_library() {
        // Do nothing in unsupported version
    }
}
