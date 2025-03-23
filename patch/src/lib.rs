use lv2_atom::{
    atom_prelude::{AtomReadError, AtomWriteError},
    atoms::object::{ObjectHeaderWriter, ObjectReader, ObjectWriter},
    prelude::*,
    space::{AtomSpace, AtomWriter, SpaceWriter},
    AtomHandle,
};
use urid::{Map, URIDCollection, UriBound, URID};

/// The `patch:Set` atom message.
#[derive(URIDCollection)]
pub struct PatchURIDCollection {
    set: URID<PatchSet>,
    get: URID<PatchGet>,
    property: URID<PatchProperty>,
    value: URID<PatchValue>,
    message: URID<String>,
}

pub struct PatchSet;
unsafe impl UriBound for PatchSet {
    const URI: &'static [u8] = lv2_sys::LV2_PATCH__Set;
}

pub struct PatchGet;
unsafe impl UriBound for PatchGet {
    const URI: &'static [u8] = lv2_sys::LV2_PATCH__Get;
}

pub struct PatchProperty;
unsafe impl UriBound for PatchProperty {
    const URI: &'static [u8] = lv2_sys::LV2_PATCH__property;
}

pub struct PatchSubject;
unsafe impl UriBound for PatchSubject {
    const URI: &'static [u8] = lv2_sys::LV2_PATCH__subject;
}

pub struct PatchValue;
unsafe impl UriBound for PatchValue {
    const URI: &'static [u8] = lv2_sys::LV2_PATCH__value;
}

pub struct ObjectReaderHandle;
impl<'a> AtomHandle<'a> for ObjectReaderHandle {
    type Handle = (ObjectHeader, ObjectReader<'a>);
}

pub struct ObjectWriterHandle;
impl<'a> AtomHandle<'a> for ObjectWriterHandle {
    type Handle = ObjectHeaderWriter<'a>;
}

impl Atom for PatchSet {
    type ReadHandle = ObjectReaderHandle;
    type WriteHandle = ObjectWriterHandle;

    unsafe fn read(body: &AtomSpace) -> Result<Self::ReadHandle, AtomReadError> {
        let mut reader = body.read();
        let header: &lv2_sys::LV2_Atom_Object_Body = reader.next_value()?;

        
        Ok(())
    }

    fn write(frame: AtomWriter) -> Result<WriteHandle, AtomWriteError> {
        frame.ob
        Ok(())
    }
}
