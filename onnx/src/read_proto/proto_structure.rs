use core::fmt::Debug;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::str::FromStr;

/*
This structure enumerates the possible kind of annotations an attribute of a Proto message could assume
accordingly to Protocol Buffers v2(proto2) documentation (https://protobuf.dev/programming-guides/proto2/).
  - Optional: means that the attribute could be not present in the Attribute structure assuming its default value.
              Note: in proto3 each attribute without explicit annotation its considered as marked optional by default.
  - Repeated: means that the attribute could be present [0..N] times
  - Required: means that the message struct cannot be considered well-formed if this attribute is not present;
             currently this annotation is no more used but is maintained for backward compatibility
  - Map: means that a certain scalar value has been encoded as "packed" (this is done by default in proto3, while must be specified
         in proto2). e.g. Map<string, i32> shows an i32 value which is packed as a string encoding (with a certain LEN).
 */
#[repr(C)]
#[derive(Default, Debug, PartialEq, Clone)]
pub enum ProtoAnnotation{
  #[default]
  Optional,
  Repeated,
  Required,
  Map
}
#[derive(Debug, PartialEq, Eq)]
pub struct ParseProtoAnnotationError;

impl FromStr for ProtoAnnotation {
  type Err = ParseProtoAnnotationError;
  //this allows to automatically parse() from a string (red from .proto file) into a ProtoAnnotation Type
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "optional" => Ok(ProtoAnnotation::Optional),
      "repeated" => Ok(ProtoAnnotation::Repeated),
      "required" => Ok(ProtoAnnotation::Required),
      "map" => Ok(ProtoAnnotation::Map),
      _ => Err(ParseProtoAnnotationError)
    }
  }
}

/*
This structure contains an Attribute of a Message struct in a .proto file. (e.g. optional string name = 1;)
  - annotation: this annotation specifies a modifier for the attribute(i.e. optional). This is only present in proto2 version, while it could be omitted in proto3 version
  - attribute_name: the name of the attribute (i.e. name)
  - attribute_type: the type of the attribute (i.e. string)
  - tag: this is the number which identifies the attribute (i.e. 1)
 */
#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct ProtoAttribute {
  pub annotation: ProtoAnnotation,
  pub attribute_name: String,
  pub attribute_type: String
}
impl ProtoAttribute {
  pub(crate) fn new() -> Self {
    Self {
      annotation: Default::default(),
      attribute_name: Default::default(),
      attribute_type: Default::default()
    }
  }
}

/*
This enum allows a certain Proto structure to be distinguished between a "message" or a "one of" or a "enum"
 */
#[repr(C)]
#[derive(Default, Debug, PartialEq, Clone)]
pub enum KindOf{
  #[default]
  Message,
  OneOf,
  Enum
}

/*
This structure contains a "message" structure or a "one of" structure contained in a .proto file.
Since this structure will be used while parsing .onnx files in order to understand its content,
all the message/one-of contained in the .proto file are stored in a HashMap (which allows O(1) searches).
Specifically, let's make an example:
  .proto file ->
            message Person {
              oneof Address {
                string city = 3;
                int32 number = 5;
              }
              optional string email = 1;
            };

  runtime ->
            proto_map: HashMap<String, Proto> = [person, proto1];

              proto1: Proto = {
                kind_of: KindOf::Message,
                attributes: HashMap<i32, ProtoAttribute>[(1, protoAttribute1)],
                contents: HashMap<String, proto2>[(address, proto2)]
              };

                protoAttribute1: ProtoAttribute = {
                  annotation: ProtoAnnotation::optional,
                  attribute_name: "email",
                  attribute_type: "string"
                };
                proto2: Proto = {
                  kind_of: KindOf::OneOf,
                  attributes: HashMap<i32, ProtoAttribute>[(3, protoAttribute2), (5, protoAttribute3)],
                  contents: HashMap<String, proto2>[]
                };

                  protoAttribute2: ProtoAttribute = {
                    annotation: ProtoAnnotation::default(),
                    attribute_name: "city",
                    attribute_type: "string"
                  };
                  protoAttribute3: ProtoAttribute = {
                    annotation: ProtoAnnotation::default(),
                    attribute_name: "number",
                    attribute_type: "int32"
                  };

  - kind_of: represents the type of the structure(Message or OneOf or Enum)
  - attributes: this HashMap contains the list of attributes. Each attribute is represented by a ProtoAttribute. The HashMap allows to
              execute O(1) searches once having the Tag(i32) key to search.
  - contents: this HashMap allows to contain other "message"/"oneof" structures recursively, preserving the O(1) access time
*/
#[repr(C)]
#[derive(Default, Clone)]
pub struct Proto {
  pub kind_of: KindOf, //one value between [Message, OneOf, Enum]
  pub attributes: HashMap<i32, ProtoAttribute>, //<tag, ProtoAttribute>
  pub contents: HashMap<String, Proto> //<name, Proto>, since a message could contain itself others messages/one-of
}
impl Proto{
  pub(crate) fn new(kind_of: KindOf) -> Self {
    Self {
      kind_of,
      attributes: HashMap::new(),
      contents: HashMap::new()
    }
  }
}
impl Debug for Proto{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{\n\t{:?} \n\tattributes: {:?} \n\tcontents: {:?}\n}}", self.kind_of, self.attributes, self.contents)
  }
}

