// Autogenerated by Thrift Compiler (0.11.0)
// DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_extern_crates)]
#![cfg_attr(feature = "cargo-clippy", allow(too_many_arguments, type_complexity))]
#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate ordered_float;
extern crate thrift;
extern crate try_from;

use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::From;
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use try_from::TryFrom;

use thrift::{ApplicationError, ApplicationErrorKind, ProtocolError, ProtocolErrorKind, TThriftClient};
use thrift::protocol::{TFieldIdentifier, TListIdentifier, TMapIdentifier, TMessageIdentifier, TMessageType, TInputProtocol, TOutputProtocol, TSetIdentifier, TStructIdentifier, TType};
use thrift::protocol::field_id;
use thrift::protocol::verify_expected_message_type;
use thrift::protocol::verify_expected_sequence_number;
use thrift::protocol::verify_expected_service_call;
use thrift::protocol::verify_required_field_exists;
use thrift::server::TProcessor;

/// A subset of thrift base types, except BYTES.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AnnotationType {
  /// Set to 0x01 when key is CLIENT_ADDR or SERVER_ADDR
  BOOL = 0,
  /// No encoding, or type is unknown.
  BYTES = 1,
  I16 = 2,
  I32 = 3,
  I64 = 4,
  DOUBLE = 5,
  /// the only type zipkin v1 supports search against.
  STRING = 6,
}

impl AnnotationType {
  pub fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    o_prot.write_i32(*self as i32)
  }
  pub fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<AnnotationType> {
    let enum_value = i_prot.read_i32()?;
    AnnotationType::try_from(enum_value)  }
}

impl TryFrom<i32> for AnnotationType {
  type Err = thrift::Error;  fn try_from(i: i32) -> Result<Self, Self::Err> {
    match i {
      0 => Ok(AnnotationType::BOOL),
      1 => Ok(AnnotationType::BYTES),
      2 => Ok(AnnotationType::I16),
      3 => Ok(AnnotationType::I32),
      4 => Ok(AnnotationType::I64),
      5 => Ok(AnnotationType::DOUBLE),
      6 => Ok(AnnotationType::STRING),
      _ => {
        Err(
          thrift::Error::Protocol(
            ProtocolError::new(
              ProtocolErrorKind::InvalidData,
              format!("cannot convert enum constant {} to AnnotationType", i)
            )
          )
        )
      },
    }
  }
}

//
// Endpoint
//

/// Indicates the network context of a service recording an annotation with two
/// exceptions.
/// 
/// When a BinaryAnnotation, and key is CLIENT_ADDR or SERVER_ADDR,
/// the endpoint indicates the source or destination of an RPC. This exception
/// allows zipkin to display network context of uninstrumented services, or
/// clients such as web browsers.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Endpoint {
  /// IPv4 host address packed into 4 bytes.
  /// 
  /// Ex for the ip 1.2.3.4, it would be (1 << 24) | (2 << 16) | (3 << 8) | 4
  pub ipv4: Option<i32>,
  /// IPv4 port or 0, if unknown.
  /// 
  /// Note: this is to be treated as an unsigned integer, so watch for negatives.
  pub port: Option<i16>,
  /// Classifier of a source or destination in lowercase, such as "zipkin-web".
  /// 
  /// This is the primary parameter for trace lookup, so should be intuitive as
  /// possible, for example, matching names in service discovery.
  /// 
  /// Conventionally, when the service name isn't known, service_name = "unknown".
  /// However, it is also permissible to set service_name = "" (empty string).
  /// The difference in the latter usage is that the span will not be queryable
  /// by service name unless more information is added to the span with non-empty
  /// service name, e.g. an additional annotation from the server.
  /// 
  /// Particularly clients may not have a reliable service name at ingest. One
  /// approach is to set service_name to "" at ingest, and later assign a
  /// better label based on binary annotations, such as user agent.
  pub service_name: Option<String>,
  /// IPv6 host address packed into 16 bytes. Ex Inet6Address.getBytes()
  pub ipv6: Option<Vec<u8>>,
}

impl Endpoint {
  pub fn new<F1, F2, F3, F4>(ipv4: F1, port: F2, service_name: F3, ipv6: F4) -> Endpoint where F1: Into<Option<i32>>, F2: Into<Option<i16>>, F3: Into<Option<String>>, F4: Into<Option<Vec<u8>>> {
    Endpoint {
      ipv4: ipv4.into(),
      port: port.into(),
      service_name: service_name.into(),
      ipv6: ipv6.into(),
    }
  }
  pub fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<Endpoint> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<i32> = Some(0);
    let mut f_2: Option<i16> = Some(0);
    let mut f_3: Option<String> = Some("".to_owned());
    let mut f_4: Option<Vec<u8>> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_i32()?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_i16()?;
          f_2 = Some(val);
        },
        3 => {
          let val = i_prot.read_string()?;
          f_3 = Some(val);
        },
        4 => {
          let val = i_prot.read_bytes()?;
          f_4 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = Endpoint {
      ipv4: f_1,
      port: f_2,
      service_name: f_3,
      ipv6: f_4,
    };
    Ok(ret)
  }
  pub fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("Endpoint");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(fld_var) = self.ipv4 {
      o_prot.write_field_begin(&TFieldIdentifier::new("ipv4", TType::I32, 1))?;
      o_prot.write_i32(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(fld_var) = self.port {
      o_prot.write_field_begin(&TFieldIdentifier::new("port", TType::I16, 2))?;
      o_prot.write_i16(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(ref fld_var) = self.service_name {
      o_prot.write_field_begin(&TFieldIdentifier::new("service_name", TType::String, 3))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(ref fld_var) = self.ipv6 {
      o_prot.write_field_begin(&TFieldIdentifier::new("ipv6", TType::String, 4))?;
      o_prot.write_bytes(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

impl Default for Endpoint {
  fn default() -> Self {
    Endpoint{
      ipv4: Some(0),
      port: Some(0),
      service_name: Some("".to_owned()),
      ipv6: Some(Vec::new()),
    }
  }
}

//
// Annotation
//

/// Associates an event that explains latency with a timestamp.
/// 
/// Unlike log statements, annotations are often codes: for example "sr".
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Annotation {
  /// Microseconds from epoch.
  /// 
  /// This value should use the most precise value possible. For example,
  /// gettimeofday or multiplying currentTimeMillis by 1000.
  pub timestamp: Option<i64>,
  /// Usually a short tag indicating an event, like "sr" or "finagle.retry".
  pub value: Option<String>,
  /// The host that recorded the value, primarily for query by service name.
  pub host: Option<Endpoint>,
}

impl Annotation {
  pub fn new<F1, F2, F3>(timestamp: F1, value: F2, host: F3) -> Annotation where F1: Into<Option<i64>>, F2: Into<Option<String>>, F3: Into<Option<Endpoint>> {
    Annotation {
      timestamp: timestamp.into(),
      value: value.into(),
      host: host.into(),
    }
  }
  pub fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<Annotation> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<i64> = Some(0);
    let mut f_2: Option<String> = Some("".to_owned());
    let mut f_3: Option<Endpoint> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_i64()?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_string()?;
          f_2 = Some(val);
        },
        3 => {
          let val = Endpoint::read_from_in_protocol(i_prot)?;
          f_3 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = Annotation {
      timestamp: f_1,
      value: f_2,
      host: f_3,
    };
    Ok(ret)
  }
  pub fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("Annotation");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(fld_var) = self.timestamp {
      o_prot.write_field_begin(&TFieldIdentifier::new("timestamp", TType::I64, 1))?;
      o_prot.write_i64(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(ref fld_var) = self.value {
      o_prot.write_field_begin(&TFieldIdentifier::new("value", TType::String, 2))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(ref fld_var) = self.host {
      o_prot.write_field_begin(&TFieldIdentifier::new("host", TType::Struct, 3))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

impl Default for Annotation {
  fn default() -> Self {
    Annotation{
      timestamp: Some(0),
      value: Some("".to_owned()),
      host: None,
    }
  }
}

//
// BinaryAnnotation
//

/// Binary annotations are tags applied to a Span to give it context. For
/// example, a binary annotation of HTTP_PATH ("http.path") could the path
/// to a resource in a RPC call.
/// 
/// Binary annotations of type STRING are always queryable, though more a
/// historical implementation detail than a structural concern.
/// 
/// Binary annotations can repeat, and vary on the host. Similar to Annotation,
/// the host indicates who logged the event. This allows you to tell the
/// difference between the client and server side of the same key. For example,
/// the key "http.path" might be different on the client and server side due to
/// rewriting, like "/api/v1/myresource" vs "/myresource. Via the host field,
/// you can see the different points of view, which often help in debugging.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BinaryAnnotation {
  /// Name used to lookup spans, such as "http.path" or "finagle.version".
  pub key: Option<String>,
  /// Serialized thrift bytes, in TBinaryProtocol format.
  /// 
  /// For legacy reasons, byte order is big-endian. See THRIFT-3217.
  pub value: Option<Vec<u8>>,
  /// The thrift type of value, most often STRING.
  /// 
  /// annotation_type shouldn't vary for the same key.
  pub annotation_type: Option<AnnotationType>,
  /// The host that recorded value, allowing query by service name or address.
  /// 
  /// There are two exceptions: when key is "ca" or "sa", this is the source or
  /// destination of an RPC. This exception allows zipkin to display network
  /// context of uninstrumented services, such as browsers or databases.
  pub host: Option<Endpoint>,
}

impl BinaryAnnotation {
  pub fn new<F1, F2, F3, F4>(key: F1, value: F2, annotation_type: F3, host: F4) -> BinaryAnnotation where F1: Into<Option<String>>, F2: Into<Option<Vec<u8>>>, F3: Into<Option<AnnotationType>>, F4: Into<Option<Endpoint>> {
    BinaryAnnotation {
      key: key.into(),
      value: value.into(),
      annotation_type: annotation_type.into(),
      host: host.into(),
    }
  }
  pub fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<BinaryAnnotation> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<String> = Some("".to_owned());
    let mut f_2: Option<Vec<u8>> = Some(Vec::new());
    let mut f_3: Option<AnnotationType> = None;
    let mut f_4: Option<Endpoint> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_string()?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_bytes()?;
          f_2 = Some(val);
        },
        3 => {
          let val = AnnotationType::read_from_in_protocol(i_prot)?;
          f_3 = Some(val);
        },
        4 => {
          let val = Endpoint::read_from_in_protocol(i_prot)?;
          f_4 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = BinaryAnnotation {
      key: f_1,
      value: f_2,
      annotation_type: f_3,
      host: f_4,
    };
    Ok(ret)
  }
  pub fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("BinaryAnnotation");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.key {
      o_prot.write_field_begin(&TFieldIdentifier::new("key", TType::String, 1))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(ref fld_var) = self.value {
      o_prot.write_field_begin(&TFieldIdentifier::new("value", TType::String, 2))?;
      o_prot.write_bytes(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(ref fld_var) = self.annotation_type {
      o_prot.write_field_begin(&TFieldIdentifier::new("annotation_type", TType::I32, 3))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(ref fld_var) = self.host {
      o_prot.write_field_begin(&TFieldIdentifier::new("host", TType::Struct, 4))?;
      fld_var.write_to_out_protocol(o_prot)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

impl Default for BinaryAnnotation {
  fn default() -> Self {
    BinaryAnnotation{
      key: Some("".to_owned()),
      value: Some(Vec::new()),
      annotation_type: None,
      host: None,
    }
  }
}

//
// Span
//

/// A trace is a series of spans (often RPC calls) which form a latency tree.
/// 
/// Spans are usually created by instrumentation in RPC clients or servers, but
/// can also represent in-process activity. Annotations in spans are similar to
/// log statements, and are sometimes created directly by application developers
/// to indicate events of interest, such as a cache miss.
/// 
/// The root span is where parent_id = Nil; it usually has the longest duration
/// in the trace.
/// 
/// Span identifiers are packed into i64s, but should be treated opaquely.
/// String encoding is fixed-width lower-hex, to avoid signed interpretation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Span {
  /// Unique 8-byte identifier for a trace, set on all spans within it.
  pub trace_id: Option<i64>,
  /// Span name in lowercase, rpc method for example. Conventionally, when the
  /// span name isn't known, name = "unknown".
  pub name: Option<String>,
  /// Unique 8-byte identifier of this span within a trace. A span is uniquely
  /// identified in storage by (trace_id, id).
  pub id: Option<i64>,
  /// The parent's Span.id; absent if this the root span in a trace.
  pub parent_id: Option<i64>,
  /// Associates events that explain latency with a timestamp. Unlike log
  /// statements, annotations are often codes: for example SERVER_RECV("sr").
  /// Annotations are sorted ascending by timestamp.
  pub annotations: Option<Vec<Annotation>>,
  /// Tags a span with context, usually to support query or aggregation. For
  /// example, a binary annotation key could be "http.path".
  pub binary_annotations: Option<Vec<BinaryAnnotation>>,
  /// True is a request to store this span even if it overrides sampling policy.
  pub debug: Option<bool>,
  /// Epoch microseconds of the start of this span, absent if this an incomplete
  /// span.
  /// 
  /// This value should be set directly by instrumentation, using the most
  /// precise value possible. For example, gettimeofday or syncing nanoTime
  /// against a tick of currentTimeMillis.
  /// 
  /// For compatibility with instrumentation that precede this field, collectors
  /// or span stores can derive this via Annotation.timestamp.
  /// For example, SERVER_RECV.timestamp or CLIENT_SEND.timestamp.
  /// 
  /// Timestamp is nullable for input only. Spans without a timestamp cannot be
  /// presented in a timeline: Span stores should not output spans missing a
  /// timestamp.
  /// 
  /// There are two known edge-cases where this could be absent: both cases
  /// exist when a collector receives a span in parts and a binary annotation
  /// precedes a timestamp. This is possible when..
  ///  - The span is in-flight (ex not yet received a timestamp)
  ///  - The span's start event was lost
  pub timestamp: Option<i64>,
  /// Measurement in microseconds of the critical path, if known. Durations of
  /// less than one microsecond must be rounded up to 1 microsecond.
  /// 
  /// This value should be set directly, as opposed to implicitly via annotation
  /// timestamps. Doing so encourages precision decoupled from problems of
  /// clocks, such as skew or NTP updates causing time to move backwards.
  /// 
  /// For compatibility with instrumentation that precede this field, collectors
  /// or span stores can derive this by subtracting Annotation.timestamp.
  /// For example, SERVER_SEND.timestamp - SERVER_RECV.timestamp.
  /// 
  /// If this field is persisted as unset, zipkin will continue to work, except
  /// duration query support will be implementation-specific. Similarly, setting
  /// this field non-atomically is implementation-specific.
  /// 
  /// This field is i64 vs i32 to support spans longer than 35 minutes.
  pub duration: Option<i64>,
  /// Optional unique 8-byte additional identifier for a trace. If non zero, this
  /// means the trace uses 128 bit traceIds instead of 64 bit.
  pub trace_id_high: Option<i64>,
}

impl Span {
  pub fn new<F1, F3, F4, F5, F6, F8, F9, F10, F11, F12>(trace_id: F1, name: F3, id: F4, parent_id: F5, annotations: F6, binary_annotations: F8, debug: F9, timestamp: F10, duration: F11, trace_id_high: F12) -> Span where F1: Into<Option<i64>>, F3: Into<Option<String>>, F4: Into<Option<i64>>, F5: Into<Option<i64>>, F6: Into<Option<Vec<Annotation>>>, F8: Into<Option<Vec<BinaryAnnotation>>>, F9: Into<Option<bool>>, F10: Into<Option<i64>>, F11: Into<Option<i64>>, F12: Into<Option<i64>> {
    Span {
      trace_id: trace_id.into(),
      name: name.into(),
      id: id.into(),
      parent_id: parent_id.into(),
      annotations: annotations.into(),
      binary_annotations: binary_annotations.into(),
      debug: debug.into(),
      timestamp: timestamp.into(),
      duration: duration.into(),
      trace_id_high: trace_id_high.into(),
    }
  }
  pub fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<Span> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<i64> = Some(0);
    let mut f_3: Option<String> = Some("".to_owned());
    let mut f_4: Option<i64> = Some(0);
    let mut f_5: Option<i64> = None;
    let mut f_6: Option<Vec<Annotation>> = Some(Vec::new());
    let mut f_8: Option<Vec<BinaryAnnotation>> = Some(Vec::new());
    let mut f_9: Option<bool> = None;
    let mut f_10: Option<i64> = None;
    let mut f_11: Option<i64> = None;
    let mut f_12: Option<i64> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_i64()?;
          f_1 = Some(val);
        },
        3 => {
          let val = i_prot.read_string()?;
          f_3 = Some(val);
        },
        4 => {
          let val = i_prot.read_i64()?;
          f_4 = Some(val);
        },
        5 => {
          let val = i_prot.read_i64()?;
          f_5 = Some(val);
        },
        6 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<Annotation> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_0 = Annotation::read_from_in_protocol(i_prot)?;
            val.push(list_elem_0);
          }
          i_prot.read_list_end()?;
          f_6 = Some(val);
        },
        8 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<BinaryAnnotation> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_1 = BinaryAnnotation::read_from_in_protocol(i_prot)?;
            val.push(list_elem_1);
          }
          i_prot.read_list_end()?;
          f_8 = Some(val);
        },
        9 => {
          let val = i_prot.read_bool()?;
          f_9 = Some(val);
        },
        10 => {
          let val = i_prot.read_i64()?;
          f_10 = Some(val);
        },
        11 => {
          let val = i_prot.read_i64()?;
          f_11 = Some(val);
        },
        12 => {
          let val = i_prot.read_i64()?;
          f_12 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = Span {
      trace_id: f_1,
      name: f_3,
      id: f_4,
      parent_id: f_5,
      annotations: f_6,
      binary_annotations: f_8,
      debug: f_9,
      timestamp: f_10,
      duration: f_11,
      trace_id_high: f_12,
    };
    Ok(ret)
  }
  pub fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("Span");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(fld_var) = self.trace_id {
      o_prot.write_field_begin(&TFieldIdentifier::new("trace_id", TType::I64, 1))?;
      o_prot.write_i64(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(ref fld_var) = self.name {
      o_prot.write_field_begin(&TFieldIdentifier::new("name", TType::String, 3))?;
      o_prot.write_string(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(fld_var) = self.id {
      o_prot.write_field_begin(&TFieldIdentifier::new("id", TType::I64, 4))?;
      o_prot.write_i64(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(fld_var) = self.parent_id {
      o_prot.write_field_begin(&TFieldIdentifier::new("parent_id", TType::I64, 5))?;
      o_prot.write_i64(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(ref fld_var) = self.annotations {
      o_prot.write_field_begin(&TFieldIdentifier::new("annotations", TType::List, 6))?;
      o_prot.write_list_begin(&TListIdentifier::new(TType::Struct, fld_var.len() as i32))?;
      for e in fld_var {
        e.write_to_out_protocol(o_prot)?;
        o_prot.write_list_end()?;
      }
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(ref fld_var) = self.binary_annotations {
      o_prot.write_field_begin(&TFieldIdentifier::new("binary_annotations", TType::List, 8))?;
      o_prot.write_list_begin(&TListIdentifier::new(TType::Struct, fld_var.len() as i32))?;
      for e in fld_var {
        e.write_to_out_protocol(o_prot)?;
        o_prot.write_list_end()?;
      }
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(fld_var) = self.debug {
      o_prot.write_field_begin(&TFieldIdentifier::new("debug", TType::Bool, 9))?;
      o_prot.write_bool(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(fld_var) = self.timestamp {
      o_prot.write_field_begin(&TFieldIdentifier::new("timestamp", TType::I64, 10))?;
      o_prot.write_i64(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(fld_var) = self.duration {
      o_prot.write_field_begin(&TFieldIdentifier::new("duration", TType::I64, 11))?;
      o_prot.write_i64(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    if let Some(fld_var) = self.trace_id_high {
      o_prot.write_field_begin(&TFieldIdentifier::new("trace_id_high", TType::I64, 12))?;
      o_prot.write_i64(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

impl Default for Span {
  fn default() -> Self {
    Span{
      trace_id: Some(0),
      name: Some("".to_owned()),
      id: Some(0),
      parent_id: Some(0),
      annotations: Some(Vec::new()),
      binary_annotations: Some(Vec::new()),
      debug: Some(false),
      timestamp: Some(0),
      duration: Some(0),
      trace_id_high: Some(0),
    }
  }
}

pub const C_L_I_E_N_T_S_E_N_D: &str = "cs";

pub const C_L_I_E_N_T_R_E_C_V: &str = "cr";

pub const S_E_R_V_E_R_S_E_N_D: &str = "ss";

pub const S_E_R_V_E_R_R_E_C_V: &str = "sr";

pub const M_E_S_S_A_G_E_S_E_N_D: &str = "ms";

pub const M_E_S_S_A_G_E_R_E_C_V: &str = "mr";

pub const W_I_R_E_S_E_N_D: &str = "ws";

pub const W_I_R_E_R_E_C_V: &str = "wr";

pub const C_L_I_E_N_T_S_E_N_D_F_R_A_G_M_E_N_T: &str = "csf";

pub const C_L_I_E_N_T_R_E_C_V_F_R_A_G_M_E_N_T: &str = "crf";

pub const S_E_R_V_E_R_S_E_N_D_F_R_A_G_M_E_N_T: &str = "ssf";

pub const S_E_R_V_E_R_R_E_C_V_F_R_A_G_M_E_N_T: &str = "srf";

pub const H_T_T_P_H_O_S_T: &str = "http.host";

pub const H_T_T_P_M_E_T_H_O_D: &str = "http.method";

pub const H_T_T_P_P_A_T_H: &str = "http.path";

pub const H_T_T_P_U_R_L: &str = "http.url";

pub const H_T_T_P_S_T_A_T_U_S_C_O_D_E: &str = "http.status_code";

pub const H_T_T_P_R_E_Q_U_E_S_T_S_I_Z_E: &str = "http.request.size";

pub const H_T_T_P_R_E_S_P_O_N_S_E_S_I_Z_E: &str = "http.response.size";

pub const L_O_C_A_L_C_O_M_P_O_N_E_N_T: &str = "lc";

pub const E_R_R_O_R: &str = "error";

pub const C_L_I_E_N_T_A_D_D_R: &str = "ca";

pub const S_E_R_V_E_R_A_D_D_R: &str = "sa";

pub const M_E_S_S_A_G_E_A_D_D_R: &str = "ma";

