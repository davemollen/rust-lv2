use crate::ControlInputPortChangeRequestErr;
use lv2_core::{
    feature::Feature,
    port::{InputPort, PortHandle, PortType},
    prelude::ThreadingClass,
};
use std::ffi::c_void;
use urid::*;

/// Feature that enables plugins to request changes on their own control input ports.
#[repr(transparent)]
pub struct ControlInputPortChangeRequest<'a> {
    internal: &'a sys::LV2_ControlInputPort_Change_Request,
}

unsafe impl<'a> UriBound for ControlInputPortChangeRequest<'a> {
    const URI: &'static [u8] = sys::LV2_CONTROL_INPUT_PORT_CHANGE_REQUEST_URI;
}

unsafe impl<'a> Feature for ControlInputPortChangeRequest<'a> {
    unsafe fn from_feature_ptr(feature: *const c_void, _class: ThreadingClass) -> Option<Self> {
        (feature as *const sys::LV2_ControlInputPort_Change_Request)
            .as_ref()
            .map(|internal| Self { internal })
    }
}

impl<'a> ControlInputPortChangeRequest<'a> {
    /// Ask the host to change a plugin's control input port value.
    ///
    /// Pass a mutable reference of the port to update.
    /// And pass the desired new value of the control port.
    ///
    /// Returns status of the request.
    /// The host may decline this request, if e.g. it is currently automating this port.
    ///
    /// The plugin MUST call this function during run().
    pub fn request_change<T: PortType>(
        &self,
        port: &InputPort<T>,
        value: f32,
    ) -> Result<(), ControlInputPortChangeRequestErr> {
        let request_change_fn = self
            .internal
            .request_change
            .ok_or(ControlInputPortChangeRequestErr::Unknown)?;

        let request_change_status =
            unsafe { request_change_fn(self.internal.handle, port.get_index(), value) };

        ControlInputPortChangeRequestErr::from(request_change_status)
    }
}
