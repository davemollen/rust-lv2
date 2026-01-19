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

pub mod mod_license_api_linker {
    /// Unsupported without the "mod_license" feature.
    ///
    /// Add the "mod_license" feature to the rust-lv2 dependency in your Cargo.toml to enable it.
    pub fn link_library() {
        // Do nothing in unsupported version
    }
}
