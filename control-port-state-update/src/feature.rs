use crate::{ControlPortState, ControlPortStateUpdateErr};
use lv2_core::{feature::Feature, port::PortHandle, prelude::ThreadingClass};
use std::ffi::c_void;
use urid::*;

/// Feature that enables plugins to update the state of their own control ports.
#[repr(transparent)]
pub struct ControlPortStateUpdate<'a> {
    internal: &'a sys::LV2_Control_Port_State_Update,
}

unsafe impl<'a> UriBound for ControlPortStateUpdate<'a> {
    const URI: &'static [u8] = sys::LV2_CONTROL_PORT_STATE_UPDATE_URI;
}

unsafe impl<'a> Feature for ControlPortStateUpdate<'a> {
    unsafe fn from_feature_ptr(feature: *const c_void, _class: ThreadingClass) -> Option<Self> {
        (feature as *const sys::LV2_Control_Port_State_Update)
            .as_ref()
            .map(|internal| Self { internal })
    }
}

impl<'a> ControlPortStateUpdate<'a> {
    /// Ask the host to change a plugin's control port's state.
    ///
    /// Pass a mutable reference of the port to update.
    /// And pass the desired new state of the control port.
    ///
    /// Returns status of the update.
    ///
    /// The plugin MUST call this function during run().
    pub fn update_state<T: PortHandle>(
        &self,
        port: &T,
        state: ControlPortState,
    ) -> Result<(), ControlPortStateUpdateErr> {
        let update_state_fn = self
            .internal
            .update_state
            .ok_or(ControlPortStateUpdateErr::Unknown)?;

        let update_state_status = unsafe {
            update_state_fn(
                self.internal.handle,
                port.get_index(),
                ControlPortState::into(state),
            )
        };

        ControlPortStateUpdateErr::from(update_state_status)
    }
}
