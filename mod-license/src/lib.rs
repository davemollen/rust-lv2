//! An interface for LV2 plugins to handle licensing and copy-protection.
//!
//! To use this LV2 extension you need to enable the "mod_license" feature.
//! ```toml
//! [dependencies]
//! lv2 = { git = "https://github.com/davemollen/rust-lv2.git", branch = "master", features = [
//!     "mod_license"
//! ] }
//! ```
//!
//! You also need to link the static MOD License API library. See the [`mod_license_api_linker`] docs for detailed instructions.
//!
//! # Example
//! ```
//! use lv2_core::prelude::*;
//! use urid::*;
//! use mod_license::*;
//!
//! #[uri("http://lv2plug.in/plugins.rs/plugin_with_licensing")]
//! struct PluginWithLicensing {
//!     mod_license: ModLicenseApi,
//! }
//!
//! #[derive(PortCollection)]
//! struct Ports {
//!     input: InputPort<Audio>,
//!     output: OutputPort<Audio>,
//! }
//!
//! impl Plugin for PluginWithLicensing {
//!     type Ports = Ports;
//!     type InitFeatures = ();
//!     type AudioFeatures = ();
//!
//!     fn new(_plugin_info: &PluginInfo, _features: &mut Self::InitFeatures) -> Option<Self> {
//!         Some(Self {
//!             mod_license: ModLicenseApi::new()
//!         })
//!     }
//!
//!     fn run(&mut self, ports: &mut Ports, _features: &mut Self::AudioFeatures, run_count: u32) {
//!         self.mod_license.run_begin(run_count);
//!         let input = ports.input.iter();
//!         let output = ports.output.iter_mut();
//!
//!         for (input_sample, output_sample) in input.zip(output) {
//!             *output_sample = *input_sample * 2.0;
//!         }
//!         self.mod_license.run_silence([ports.output.as_mut_ptr()], run_count);
//!     }
//! }
//!
//! lv2_descriptors!(PluginWithLicensing);
//! ```
extern crate lv2_core as core;
extern crate lv2_sys as sys;

mod feature;
pub use feature::*;
#[cfg_attr(feature = "mod_license", path = "supported.rs")]
mod unsupported;
pub use unsupported::*;
