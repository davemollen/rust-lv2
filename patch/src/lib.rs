//! LV2 specification for patch, a protocol for accessing and manipulating properties.
//!
//! The original [specification](https://lv2plug.in/ns/ext/patch) contains means to access and manipulate properties with messages.
//!
//! # Example
//! ```
//! use lv2_atom::prelude::*;
//! use lv2_core::prelude::*;
//! use lv2_patch::*;
//! use lv2_units::prelude::*;
//! use lv2_urid::LV2Map;
//! use urid::*;
//! use std::string::String;
//!
//! #[derive(PortCollection)]
//! struct Ports {
//!   control: InputPort<AtomPort>,
//!   notify: OutputPort<AtomPort>,
//! }
//!
//! #[derive(FeatureCollection)]
//! struct InitFeatures<'a> {
//!   map: LV2Map<'a>,
//! }
//!
//! #[uri("http://lv2plug.in/plugins.rs/patch_example#sample")]
//! struct Sample;
//!
//! #[derive(URIDCollection)]
//! struct URIDs {
//!   atom: AtomURIDCollection,
//!   unit: UnitURIDCollection,
//!   patch: PatchURIDCollection,
//!   sample: URID<Sample>,
//! }
//!
//! #[uri("http://lv2plug.in/plugins.rs/patch_example")]
//! struct PatchExample {
//!   urids: URIDs,
//!   file_path: String,
//! }
//!
//! impl Plugin for PatchExample {
//!   type Ports = Ports;
//!   type InitFeatures = InitFeatures<'static>;
//!   type AudioFeatures = ();
//!
//!   fn new(_plugin_info: &PluginInfo, features: &mut Self::InitFeatures) -> Option<Self> {
//!     Some(Self {
//!       urids: features.map.populate_collection()?,
//!       file_path: "".to_string(),
//!     })
//!   }
//!
//!   fn run(&mut self, ports: &mut Ports, _features: &mut Self::AudioFeatures, _sample_count: u32) {
//!     let control_sequence = match ports
//!       .control
//!       .read(self.urids.atom.sequence, self.urids.unit.beat)
//!     {
//!       Some(sequence_iter) => sequence_iter,
//!       None => return,
//!     };
//!
//!     for (time_stamp, atom) in control_sequence {
//!       // Handle patch get events
//!       let (object_header, object_reader) = match atom.read(self.urids.atom.object, ()) {
//!         Some(object) => object,
//!         None => return,
//!       };
//!
//!       if object_header.otype == self.urids.patch.get_class {
//!         let mut notify_sequence = match ports.notify.init(
//!           self.urids.atom.sequence,
//!           TimeStampURID::Frames(self.urids.unit.frame),
//!         ) {
//!           Some(sequence_iter) => sequence_iter,
//!           None => return,
//!         };
//!
//!         let mut object_writer = notify_sequence
//!           .init(
//!             TimeStamp::Frames(time_stamp.as_frames().unwrap_or(0)),
//!             self.urids.atom.object,
//!             ObjectHeader {
//!               id: None,
//!               otype: self.urids.patch.set_class.into_general(),
//!             },
//!           )
//!           .unwrap();
//!         object_writer
//!           .init(
//!             self.urids.patch.property,
//!             self.urids.atom.urid,
//!             self.urids.sample.into_general(),
//!           )
//!           .unwrap();
//!         let mut path_value_writer = object_writer
//!           .init(self.urids.patch.value, self.urids.atom.path, ())
//!           .unwrap();
//!         path_value_writer.append(&self.file_path).unwrap();
//!       }
//!
//!       // Handle patch set events
//!       if object_header.otype == self.urids.patch.set_class {
//!         for (property_header, property) in object_reader {
//!           if property_header.key == self.urids.patch.value {
//!             self.file_path = property
//!               .read(self.urids.atom.path, ())
//!               .map(|path| path.to_string())
//!               .unwrap();
//!           }
//!         }
//!       };
//!     }
//!   }
//! }
//!
//! lv2_descriptors!(PatchExample);
//! ```

extern crate lv2_sys as sys;
use urid::*;

/// All patch URI bounds.
///
/// All Struct suffixed by `Class` are patch Classes, others are patch properties.
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
