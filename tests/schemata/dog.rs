// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Dog {
    // message fields
    breed: ::protobuf::SingularField<::std::string::String>,
    age: ::std::option::Option<i32>,
    temperament: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Dog {}

impl Dog {
    pub fn new() -> Dog {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Dog {
        static mut instance: ::protobuf::lazy::Lazy<Dog> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Dog,
        };
        unsafe {
            instance.get(Dog::new)
        }
    }

    // required string breed = 1;

    pub fn clear_breed(&mut self) {
        self.breed.clear();
    }

    pub fn has_breed(&self) -> bool {
        self.breed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_breed(&mut self, v: ::std::string::String) {
        self.breed = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_breed(&mut self) -> &mut ::std::string::String {
        if self.breed.is_none() {
            self.breed.set_default();
        };
        self.breed.as_mut().unwrap()
    }

    // Take field
    pub fn take_breed(&mut self) -> ::std::string::String {
        self.breed.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_breed(&self) -> &str {
        match self.breed.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_breed_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.breed
    }

    fn mut_breed_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.breed
    }

    // required int32 age = 2;

    pub fn clear_age(&mut self) {
        self.age = ::std::option::Option::None;
    }

    pub fn has_age(&self) -> bool {
        self.age.is_some()
    }

    // Param is passed by value, moved
    pub fn set_age(&mut self, v: i32) {
        self.age = ::std::option::Option::Some(v);
    }

    pub fn get_age(&self) -> i32 {
        self.age.unwrap_or(0)
    }

    fn get_age_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.age
    }

    fn mut_age_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.age
    }

    // required string temperament = 3;

    pub fn clear_temperament(&mut self) {
        self.temperament.clear();
    }

    pub fn has_temperament(&self) -> bool {
        self.temperament.is_some()
    }

    // Param is passed by value, moved
    pub fn set_temperament(&mut self, v: ::std::string::String) {
        self.temperament = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_temperament(&mut self) -> &mut ::std::string::String {
        if self.temperament.is_none() {
            self.temperament.set_default();
        };
        self.temperament.as_mut().unwrap()
    }

    // Take field
    pub fn take_temperament(&mut self) -> ::std::string::String {
        self.temperament.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_temperament(&self) -> &str {
        match self.temperament.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_temperament_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.temperament
    }

    fn mut_temperament_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.temperament
    }
}

impl ::protobuf::Message for Dog {
    fn is_initialized(&self) -> bool {
        if self.breed.is_none() {
            return false;
        };
        if self.age.is_none() {
            return false;
        };
        if self.temperament.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.breed)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.age = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.temperament)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.breed.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.age {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.temperament.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.breed.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.age {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.temperament.as_ref() {
            os.write_string(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Dog {
    fn new() -> Dog {
        Dog::new()
    }

    fn descriptor_static(_: ::std::option::Option<Dog>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "breed",
                    Dog::get_breed_for_reflect,
                    Dog::mut_breed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "age",
                    Dog::get_age_for_reflect,
                    Dog::mut_age_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "temperament",
                    Dog::get_temperament_for_reflect,
                    Dog::mut_temperament_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Dog>(
                    "Dog",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Dog {
    fn clear(&mut self) {
        self.clear_breed();
        self.clear_age();
        self.clear_temperament();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Dog {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Dog {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x09, 0x64, 0x6f, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0f, 0x63, 0x6f, 0x6d,
    0x2e, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2e, 0x64, 0x6f, 0x67, 0x22, 0x36, 0x0a, 0x03,
    0x44, 0x6f, 0x67, 0x12, 0x0d, 0x0a, 0x05, 0x62, 0x72, 0x65, 0x65, 0x64, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x09, 0x12, 0x0b, 0x0a, 0x03, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x05, 0x12,
    0x13, 0x0a, 0x0b, 0x74, 0x65, 0x6d, 0x70, 0x65, 0x72, 0x61, 0x6d, 0x65, 0x6e, 0x74, 0x18, 0x03,
    0x20, 0x02, 0x28, 0x09, 0x4a, 0xf9, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x06, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x02, 0x00, 0x06, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08,
    0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x02, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x03, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x03, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x03, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x04, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x04, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x04, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04, 0x11, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x05, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x05, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x05,
    0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x05, 0x20, 0x21,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
