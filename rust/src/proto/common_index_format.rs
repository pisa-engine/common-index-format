// This file is generated by rust-protobuf 2.10.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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
//! Generated file from `common-index-format.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_10_1;

#[derive(PartialEq,Clone,Default)]
pub struct Posting {
    // message fields
    pub docid: i32,
    pub tf: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Posting {
    fn default() -> &'a Posting {
        <Posting as ::protobuf::Message>::default_instance()
    }
}

impl Posting {
    pub fn new() -> Posting {
        ::std::default::Default::default()
    }

    // int32 docid = 1;


    pub fn get_docid(&self) -> i32 {
        self.docid
    }
    pub fn clear_docid(&mut self) {
        self.docid = 0;
    }

    // Param is passed by value, moved
    pub fn set_docid(&mut self, v: i32) {
        self.docid = v;
    }

    // int32 tf = 2;


    pub fn get_tf(&self) -> i32 {
        self.tf
    }
    pub fn clear_tf(&mut self) {
        self.tf = 0;
    }

    // Param is passed by value, moved
    pub fn set_tf(&mut self, v: i32) {
        self.tf = v;
    }
}

impl ::protobuf::Message for Posting {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.docid = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.tf = tmp;
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
        if self.docid != 0 {
            my_size += ::protobuf::rt::value_size(1, self.docid, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.tf != 0 {
            my_size += ::protobuf::rt::value_size(2, self.tf, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.docid != 0 {
            os.write_int32(1, self.docid)?;
        }
        if self.tf != 0 {
            os.write_int32(2, self.tf)?;
        }
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Posting {
        Posting::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "docid",
                    |m: &Posting| { &m.docid },
                    |m: &mut Posting| { &mut m.docid },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "tf",
                    |m: &Posting| { &m.tf },
                    |m: &mut Posting| { &mut m.tf },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Posting>(
                    "Posting",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Posting {
        static mut instance: ::protobuf::lazy::Lazy<Posting> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Posting,
        };
        unsafe {
            instance.get(Posting::new)
        }
    }
}

impl ::protobuf::Clear for Posting {
    fn clear(&mut self) {
        self.docid = 0;
        self.tf = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Posting {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Posting {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PostingsList {
    // message fields
    pub term: ::std::string::String,
    pub df: i64,
    pub cf: i64,
    pub posting: ::protobuf::RepeatedField<Posting>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PostingsList {
    fn default() -> &'a PostingsList {
        <PostingsList as ::protobuf::Message>::default_instance()
    }
}

impl PostingsList {
    pub fn new() -> PostingsList {
        ::std::default::Default::default()
    }

    // string term = 1;


    pub fn get_term(&self) -> &str {
        &self.term
    }
    pub fn clear_term(&mut self) {
        self.term.clear();
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: ::std::string::String) {
        self.term = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_term(&mut self) -> &mut ::std::string::String {
        &mut self.term
    }

    // Take field
    pub fn take_term(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.term, ::std::string::String::new())
    }

    // int64 df = 2;


    pub fn get_df(&self) -> i64 {
        self.df
    }
    pub fn clear_df(&mut self) {
        self.df = 0;
    }

    // Param is passed by value, moved
    pub fn set_df(&mut self, v: i64) {
        self.df = v;
    }

    // int64 cf = 3;


    pub fn get_cf(&self) -> i64 {
        self.cf
    }
    pub fn clear_cf(&mut self) {
        self.cf = 0;
    }

    // Param is passed by value, moved
    pub fn set_cf(&mut self, v: i64) {
        self.cf = v;
    }

    // repeated .cif.Posting posting = 4;


    pub fn get_posting(&self) -> &[Posting] {
        &self.posting
    }
    pub fn clear_posting(&mut self) {
        self.posting.clear();
    }

    // Param is passed by value, moved
    pub fn set_posting(&mut self, v: ::protobuf::RepeatedField<Posting>) {
        self.posting = v;
    }

    // Mutable pointer to the field.
    pub fn mut_posting(&mut self) -> &mut ::protobuf::RepeatedField<Posting> {
        &mut self.posting
    }

    // Take field
    pub fn take_posting(&mut self) -> ::protobuf::RepeatedField<Posting> {
        ::std::mem::replace(&mut self.posting, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for PostingsList {
    fn is_initialized(&self) -> bool {
        for v in &self.posting {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.term)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.df = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.cf = tmp;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.posting)?;
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
        if !self.term.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.term);
        }
        if self.df != 0 {
            my_size += ::protobuf::rt::value_size(2, self.df, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.cf != 0 {
            my_size += ::protobuf::rt::value_size(3, self.cf, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.posting {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.term.is_empty() {
            os.write_string(1, &self.term)?;
        }
        if self.df != 0 {
            os.write_int64(2, self.df)?;
        }
        if self.cf != 0 {
            os.write_int64(3, self.cf)?;
        }
        for v in &self.posting {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> PostingsList {
        PostingsList::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "term",
                    |m: &PostingsList| { &m.term },
                    |m: &mut PostingsList| { &mut m.term },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "df",
                    |m: &PostingsList| { &m.df },
                    |m: &mut PostingsList| { &mut m.df },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "cf",
                    |m: &PostingsList| { &m.cf },
                    |m: &mut PostingsList| { &mut m.cf },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Posting>>(
                    "posting",
                    |m: &PostingsList| { &m.posting },
                    |m: &mut PostingsList| { &mut m.posting },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PostingsList>(
                    "PostingsList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static PostingsList {
        static mut instance: ::protobuf::lazy::Lazy<PostingsList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PostingsList,
        };
        unsafe {
            instance.get(PostingsList::new)
        }
    }
}

impl ::protobuf::Clear for PostingsList {
    fn clear(&mut self) {
        self.term.clear();
        self.df = 0;
        self.cf = 0;
        self.posting.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PostingsList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PostingsList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19common-index-format.proto\x12\x03cif\"*\n\x07Posting\x12\x0f\n\x05\
    docid\x18\x01\x20\x01(\x05B\0\x12\x0c\n\x02tf\x18\x02\x20\x01(\x05B\0:\0\
    \"]\n\x0cPostingsList\x12\x0e\n\x04term\x18\x01\x20\x01(\tB\0\x12\x0c\n\
    \x02df\x18\x02\x20\x01(\x03B\0\x12\x0c\n\x02cf\x18\x03\x20\x01(\x03B\0\
    \x12\x1f\n\x07posting\x18\x04\x20\x03(\x0b2\x0c.cif.PostingB\0:\0B\0b\
    \x06proto3\
";

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
