// This file is generated by rust-protobuf 3.0.0-pre. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `doctest_pb.proto`

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(serde, derive(Serialize, Deserialize))]
pub struct MyMessage {
    // special fields
    #[cfg_attr(serde, serde(skip))]
    pub unknown_fields: crate::UnknownFields,
    #[cfg_attr(serde, serde(skip))]
    pub cached_size: crate::rt::CachedSize,
}

impl<'a> ::std::default::Default for &'a MyMessage {
    fn default() -> &'a MyMessage {
        <MyMessage as crate::Message>::default_instance()
    }
}

impl MyMessage {
    pub fn new() -> MyMessage {
        ::std::default::Default::default()
    }
}

impl crate::Message for MyMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut crate::CodedInputStream<'_>) -> crate::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    crate::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += crate::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut crate::CodedOutputStream<'_>) -> crate::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &crate::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut crate::UnknownFields {
        &mut self.unknown_fields
    }

    fn descriptor(&self) -> &'static crate::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> MyMessage {
        MyMessage::new()
    }

    fn descriptor_static() -> &'static crate::reflect::MessageDescriptor {
        static descriptor: crate::rt::LazyV2<crate::reflect::MessageDescriptor> = crate::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            crate::reflect::MessageDescriptor::new::<MyMessage>(
                "MyMessage",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MyMessage {
        static instance: MyMessage = MyMessage {
            unknown_fields: crate::UnknownFields::new(),
            cached_size: crate::rt::CachedSize::new(),
        };
        &instance
    }
}

impl crate::Clear for MyMessage {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MyMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        crate::text_format::fmt(self, f)
    }
}

impl crate::reflect::ProtobufValue for MyMessage {
}

impl crate::reflect::ProtobufValueSized for MyMessage {
    type RuntimeType = crate::reflect::runtime_types::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10doctest_pb.proto\"\x0b\n\tMyMessageJG\n\x06\x12\x04\x02\0\x05\x01\
    \n%\n\x01\x0c\x12\x03\x02\0\x122\x1b\x20Messages\x20used\x20in\x20doctes\
    ts\n\n\n\n\x02\x04\0\x12\x04\x04\0\x05\x01\n\n\n\x03\x04\0\x01\x12\x03\
    \x04\x08\x11b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
pub fn file_descriptor_proto() -> &'static crate::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: crate::rt::LazyV2<crate::descriptor::FileDescriptorProto> = crate::rt::LazyV2::INIT;
    file_descriptor_proto_lazy.get(|| {
        crate::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static crate::reflect::FileDescriptor {
    static file_descriptor_lazy: crate::rt::LazyV2<crate::reflect::FileDescriptor> = crate::rt::LazyV2::INIT;
    file_descriptor_lazy.get(|| {
        let mut deps = ::std::vec::Vec::new();
        let mut messages = ::std::vec::Vec::new();
        messages.push(<MyMessage as crate::Message>::descriptor_static());
        let mut enums = ::std::vec::Vec::new();
        crate::reflect::FileDescriptor::new(
            file_descriptor_proto(),
            deps,
            messages,
            enums,
        )
    })
}
