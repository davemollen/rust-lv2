//! LV2 specification for patch, a protocol for accessing and manipulating
//! properties.
//!
//! The original [specification](https://lv2plug.in/ns/ext/patch) contains means to access and manipulate properties with messages.
extern crate lv2_sys as sys;

use urid::*;

/// All patch URI bounds
///
/// All Struct suffixed by `Class` are patch Classes, others are patch
/// properties.
pub mod patch {
    use urid::UriBound;

    pub struct MessageClass;
    unsafe impl UriBound for MessageClass {
        const URI: &'static [u8] = sys::LV2_PATCH__Message;
    }

    pub struct RequestClass;
    unsafe impl UriBound for RequestClass {
        const URI: &'static [u8] = sys::LV2_PATCH__Request;
    }

    pub struct CopyClass;
    unsafe impl UriBound for CopyClass {
        const URI: &'static [u8] = sys::LV2_PATCH__Copy;
    }

    pub struct DeleteClass;
    unsafe impl UriBound for DeleteClass {
        const URI: &'static [u8] = sys::LV2_PATCH__Delete;
    }

    pub struct GetClass;
    unsafe impl UriBound for GetClass {
        const URI: &'static [u8] = sys::LV2_PATCH__Get;
    }

    pub struct MoveClass;
    unsafe impl UriBound for MoveClass {
        const URI: &'static [u8] = sys::LV2_PATCH__Move;
    }

    pub struct PatchClass;
    unsafe impl UriBound for PatchClass {
        const URI: &'static [u8] = sys::LV2_PATCH__Patch;
    }

    pub struct PutClass;
    unsafe impl UriBound for PutClass {
        const URI: &'static [u8] = sys::LV2_PATCH__Put;
    }

    pub struct SetClass;
    unsafe impl UriBound for SetClass {
        const URI: &'static [u8] = sys::LV2_PATCH__Set;
    }

    pub struct ResponseClass;
    unsafe impl UriBound for ResponseClass {
        const URI: &'static [u8] = sys::LV2_PATCH__Response;
    }

    pub struct AckClass;
    unsafe impl UriBound for AckClass {
        const URI: &'static [u8] = sys::LV2_PATCH__Ack;
    }

    pub struct ErrorClass;
    unsafe impl UriBound for ErrorClass {
        const URI: &'static [u8] = sys::LV2_PATCH__Error;
    }

    pub struct Accept;
    unsafe impl UriBound for Accept {
        const URI: &'static [u8] = sys::LV2_PATCH__accept;
    }

    pub struct Add;
    unsafe impl UriBound for Add {
        const URI: &'static [u8] = sys::LV2_PATCH__add;
    }

    pub struct Body;
    unsafe impl UriBound for Body {
        const URI: &'static [u8] = sys::LV2_PATCH__body;
    }

    pub struct Context;
    unsafe impl UriBound for Context {
        const URI: &'static [u8] = sys::LV2_PATCH__context;
    }

    pub struct Destination;
    unsafe impl UriBound for Destination {
        const URI: &'static [u8] = sys::LV2_PATCH__destination;
    }

    pub struct Property;
    unsafe impl UriBound for Property {
        const URI: &'static [u8] = sys::LV2_PATCH__property;
    }

    pub struct Readable;
    unsafe impl UriBound for Readable {
        const URI: &'static [u8] = sys::LV2_PATCH__readable;
    }

    pub struct Remove;
    unsafe impl UriBound for Remove {
        const URI: &'static [u8] = sys::LV2_PATCH__remove;
    }

    pub struct Request;
    unsafe impl UriBound for Request {
        const URI: &'static [u8] = sys::LV2_PATCH__request;
    }

    pub struct SequenceNumber;
    unsafe impl UriBound for SequenceNumber {
        const URI: &'static [u8] = sys::LV2_PATCH__sequenceNumber;
    }

    pub struct Subject;
    unsafe impl UriBound for Subject {
        const URI: &'static [u8] = sys::LV2_PATCH__subject;
    }

    pub struct Value;
    unsafe impl UriBound for Value {
        const URI: &'static [u8] = sys::LV2_PATCH__value;
    }

    pub struct Writable;
    unsafe impl UriBound for Writable {
        const URI: &'static [u8] = sys::LV2_PATCH__writable;
    }
}

use patch::*;

/// A URID cache containing all patch properties.
#[derive(URIDCollection)]
pub struct PatchURIDCollection {
    pub message_class: URID<MessageClass>,
    pub request_class: URID<RequestClass>,
    pub copy_class: URID<CopyClass>,
    pub delete_class: URID<DeleteClass>,
    pub get_class: URID<GetClass>,
    pub move_class: URID<MoveClass>,
    pub patch_class: URID<PatchClass>,
    pub put_class: URID<PutClass>,
    pub set_class: URID<SetClass>,
    pub response_class: URID<ResponseClass>,
    pub ack_class: URID<AckClass>,
    pub error_class: URID<ErrorClass>,
    pub accept: URID<Accept>,
    pub add: URID<Add>,
    pub body: URID<Body>,
    pub context: URID<Context>,
    pub destination: URID<Destination>,
    pub property: URID<Property>,
    pub readable: URID<Readable>,
    pub remove: URID<Remove>,
    pub request: URID<Request>,
    pub sequence_number: URID<SequenceNumber>,
    pub subject: URID<Subject>,
    pub value: URID<Value>,
    pub writable: URID<Writable>,
}

/// Prelude of `lv2_patch` for wildcard usage.
pub mod prelude {
    pub use crate::patch::*;
    pub use crate::PatchURIDCollection;
}
