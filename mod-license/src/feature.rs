use lv2_core::{feature::Feature, prelude::ThreadingClass};
use std::ffi::{c_void, CStr};
use urid::*;

/// Feature allowing to manage MOD Audio license
#[repr(transparent)]
pub struct ModLicense<'a> {
    internal: &'a lv2_sys::MOD_License_Feature,
}

unsafe impl<'a> UriBound for ModLicense<'a> {
    const URI: &'static [u8] = b"http://moddevices.com/ns/ext/license#feature\0";
}

unsafe impl<'a> Feature for ModLicense<'a> {
    unsafe fn from_feature_ptr(feature: *const c_void, _class: ThreadingClass) -> Option<Self> {
        (feature as *const lv2_sys::MOD_License_Feature)
            .as_ref()
            .map(|internal| Self { internal })
    }
}

impl<'a> ModLicense<'a> {
    /// Ask the host about a license file for a specific uri (can be the plugin uri or a collection).
    ///
    /// The host will return the contents of the file, signed and encrypted, or NULL if no license exists.
    pub fn licensee(&self, plugin_uri: &Uri) -> Option<String> {
        unsafe {
            let license_fn = self.internal.license?;
            let free_fn = self.internal.free?;
            let license_ptr = license_fn(self.internal.handle, plugin_uri.as_ptr());

            let license = if license_ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(license_ptr).to_string_lossy().into_owned())
            };

            free_fn(self.internal.handle, license_ptr);

            license
        }
    }
}
