//! An interface for LV2 plugins to handle licensing and copy-protection, see <http://moddevices.com/ns/ext/license> for details.
//!
//! To use this LV2 extension you need to link the static MOD License API library first.
//! Make sure you have rust-lv2 as a dependency and build-dependency in your Cargo.toml and enable the "mod_license" feature on both.
//! ```toml
//! [dependencies]
//! lv2 = { git = "https://github.com/davemollen/rust-lv2.git", branch = "master", features = [
//! "mod_license"
//! ] }
//!
//! [build-dependencies]
//! lv2 = { git = "https://github.com/davemollen/rust-lv2.git", branch = "master", features = [
//!     "mod_license"
//! ] }
//! ```
//!
//! Use this function within build.rs to link the MOD License API library:
//! ```
//! use mod_license::*;
//! mod_license_api_linker::link_library();
//! ```
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
