#![allow(clippy::needless_doctest_main)]

use std::ffi::CStr;
use sys::*;

/// MOD License API to handle licensing and copy-protection.
#[derive(Default)]
pub struct ModLicenseApi {
    run_count: u32,
}

impl ModLicenseApi {
    /// Create a new ModLicenseApi instance.
    pub fn new() -> Self {
        Self { run_count: 0 }
    }

    /// Begin time calculations for unlicensed silence.
    ///
    /// Must be called at the beginning of each run().
    /// This counts samples (time) to later decide if silence needs to be injected.
    pub fn run_begin(&mut self, sample_count: u32) {
        self.run_count = unsafe { mod_license_run_begin(self.run_count, sample_count) };
    }

    /// Inject silence into output buffers if unlicensed.
    ///
    /// Must be called at the end of each run(), for all audio output buffers.
    pub fn run_silence<const N: usize>(&self, output_buffers: [*mut f32; N], sample_count: u32) {
        output_buffers
            .iter()
            .enumerate()
            .for_each(|(channel, &ptr)| unsafe {
                mod_license_run_silence(self.run_count, ptr, sample_count, channel as u32);
            });
    }

    /// Return the version of the modla library.
    pub fn version() -> Option<String> {
        let version = unsafe { mod_license_version() };
        if version.is_null() {
            None
        } else {
            Some(
                unsafe { CStr::from_ptr(version) }
                    .to_string_lossy()
                    .into_owned(),
            )
        }
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
    const MOD_LICENSE_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

    pub fn link_library() {
        let lib_path = format!("{}/include", MOD_LICENSE_MANIFEST_DIR);
        println!("cargo:rustc-link-search=native={}", lib_path);
        println!("cargo:rustc-link-lib=static=modla");
    }
}
