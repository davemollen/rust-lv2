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
