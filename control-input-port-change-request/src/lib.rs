//! LV2 extension for allowing plugins to request changes on their own control input ports.
//!
//! # Example
//! ```
//! use lv2_core::prelude::*;
//! use urid::*;
//! use control_input_port_change_request::*;
//!
//! #[uri("http://lv2plug.in/plugins.rs/simple_amp")]
//! struct SimpleAmp;
//!
//! #[derive(PortCollection)]
//! struct Ports {
//!   gain: InputPort<Control>,
//!   gain_enabled: InputPort<Control>,
//!   input: InputPort<Audio>,
//!   output: OutputPort<Audio>,
//! }
//!
//! #[derive(FeatureCollection)]
//! struct AudioFeatures<'a> {
//!   control_input_port_change_request: ControlInputPortChangeRequest<'a>,
//! }
//!
//! impl Plugin for SimpleAmp {
//!   type Ports = Ports;
//!   type InitFeatures = ();
//!   type AudioFeatures = AudioFeatures<'static>;
//!
//!   fn new(_plugin_info: &PluginInfo, _features: &mut Self::InitFeatures) -> Option<Self> {
//!     Some(Self)
//!   }
//!
//!   fn run(&mut self, ports: &mut Ports, features: &mut Self::AudioFeatures, _run_count: u32) {
//!     let gain = *ports.gain;
//!     let gain_enabled = *ports.gain_enabled == 1.;
//!     let input = ports.input.iter();
//!     let output = ports.output.iter_mut();
//!
//!     if gain_enabled {
//!       features
//!         .control_input_port_change_request
//!         .request_change(&ports.gain, 0.)
//!         .ok();
//!     } else {
//!       features
//!         .control_input_port_change_request
//!         .request_change(&ports.gain, 1.)
//!         .ok();
//!     }
//!
//!     for (input_sample, output_sample) in input.zip(output) {
//!       *output_sample = *input_sample * gain;
//!     }
//!   }
//! }
//!
//! lv2_descriptors!(SimpleAmp);
//! ```

extern crate lv2_sys as sys;
mod feature;
pub use feature::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlInputPortChangeRequestErr {
    /// Unknown error.
    Unknown,
    /// Failed due to invalid port index.
    InvalidIndex,
}

impl ControlInputPortChangeRequestErr {
    /// Convert a raw status flag to a result or possible error value.
    pub fn from(
        value: sys::LV2_ControlInputPort_Change_Status,
    ) -> Result<(), ControlInputPortChangeRequestErr> {
        match value {
            sys::LV2_ControlInputPort_Change_Status_LV2_CONTROL_INPUT_PORT_CHANGE_SUCCESS => Ok(()),
            sys::LV2_ControlInputPort_Change_Status_LV2_CONTROL_INPUT_PORT_CHANGE_ERR_INVALID_INDEX => Err(ControlInputPortChangeRequestErr::InvalidIndex),
            sys::LV2_ControlInputPort_Change_Status_LV2_CONTROL_INPUT_PORT_CHANGE_ERR_UNKNOWN => Err(ControlInputPortChangeRequestErr::Unknown),
            _ => Err(ControlInputPortChangeRequestErr::Unknown),
        }
    }

    /// Convert a result to a raw status flag.
    pub fn into(
        result: Result<(), ControlInputPortChangeRequestErr>,
    ) -> sys::LV2_ControlInputPort_Change_Status {
        match result {
            Ok(()) => sys::LV2_ControlInputPort_Change_Status_LV2_CONTROL_INPUT_PORT_CHANGE_SUCCESS,
            Err(ControlInputPortChangeRequestErr::InvalidIndex) => {
                sys::LV2_ControlInputPort_Change_Status_LV2_CONTROL_INPUT_PORT_CHANGE_ERR_INVALID_INDEX
            }
            Err(ControlInputPortChangeRequestErr::Unknown) => sys::LV2_ControlInputPort_Change_Status_LV2_CONTROL_INPUT_PORT_CHANGE_ERR_UNKNOWN,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ControlInputPortChangeRequestErr;

    #[test]
    fn test_control_port_state_err_conversion() {
        assert_eq!(
            Ok(()),
            ControlInputPortChangeRequestErr::from(
                sys::LV2_ControlInputPort_Change_Status_LV2_CONTROL_INPUT_PORT_CHANGE_SUCCESS
            )
        );
        assert_eq!(
            Err(ControlInputPortChangeRequestErr::InvalidIndex),
            ControlInputPortChangeRequestErr::from(sys::LV2_ControlInputPort_Change_Status_LV2_CONTROL_INPUT_PORT_CHANGE_ERR_INVALID_INDEX),
        );
        assert_eq!(
            Err(ControlInputPortChangeRequestErr::Unknown),
            ControlInputPortChangeRequestErr::from(
                sys::LV2_ControlInputPort_Change_Status_LV2_CONTROL_INPUT_PORT_CHANGE_ERR_UNKNOWN
            ),
        );

        assert_eq!(
            ControlInputPortChangeRequestErr::into(Ok(())),
            sys::LV2_ControlInputPort_Change_Status_LV2_CONTROL_INPUT_PORT_CHANGE_SUCCESS
        );
        assert_eq!(
            ControlInputPortChangeRequestErr::into(Err(
                ControlInputPortChangeRequestErr::InvalidIndex
            )),
            sys::LV2_ControlInputPort_Change_Status_LV2_CONTROL_INPUT_PORT_CHANGE_ERR_INVALID_INDEX
        );
        assert_eq!(
            ControlInputPortChangeRequestErr::into(Err(ControlInputPortChangeRequestErr::Unknown)),
            sys::LV2_ControlInputPort_Change_Status_LV2_CONTROL_INPUT_PORT_CHANGE_ERR_UNKNOWN
        );
    }
}
