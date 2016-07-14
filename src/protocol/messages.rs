// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct ItemCreatedResponse {
    // message fields
    item_id: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ItemCreatedResponse {}

impl ItemCreatedResponse {
    pub fn new() -> ItemCreatedResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ItemCreatedResponse {
        static mut instance: ::protobuf::lazy::Lazy<ItemCreatedResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ItemCreatedResponse,
        };
        unsafe {
            instance.get(|| {
                ItemCreatedResponse {
                    item_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 item_id = 1;

    pub fn clear_item_id(&mut self) {
        self.item_id = ::std::option::Option::None;
    }

    pub fn has_item_id(&self) -> bool {
        self.item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_id(&mut self, v: i64) {
        self.item_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_id(&self) -> i64 {
        self.item_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for ItemCreatedResponse {
    fn is_initialized(&self) -> bool {
        if self.item_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.item_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.item_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_id {
            try!(os.write_int64(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ItemCreatedResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ItemCreatedResponse {
    fn new() -> ItemCreatedResponse {
        ItemCreatedResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ItemCreatedResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "item_id",
                    ItemCreatedResponse::has_item_id,
                    ItemCreatedResponse::get_item_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ItemCreatedResponse>(
                    "ItemCreatedResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ItemCreatedResponse {
    fn clear(&mut self) {
        self.clear_item_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ItemCreatedResponse {
    fn eq(&self, other: &ItemCreatedResponse) -> bool {
        self.item_id == other.item_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ItemCreatedResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetVersionResponse {
    // message fields
    major_version: ::std::option::Option<i32>,
    minor_version: ::std::option::Option<i32>,
    patch_version: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetVersionResponse {}

impl GetVersionResponse {
    pub fn new() -> GetVersionResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetVersionResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetVersionResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetVersionResponse,
        };
        unsafe {
            instance.get(|| {
                GetVersionResponse {
                    major_version: ::std::option::Option::None,
                    minor_version: ::std::option::Option::None,
                    patch_version: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 major_version = 1;

    pub fn clear_major_version(&mut self) {
        self.major_version = ::std::option::Option::None;
    }

    pub fn has_major_version(&self) -> bool {
        self.major_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_major_version(&mut self, v: i32) {
        self.major_version = ::std::option::Option::Some(v);
    }

    pub fn get_major_version(&self) -> i32 {
        self.major_version.unwrap_or(0)
    }

    // required int32 minor_version = 2;

    pub fn clear_minor_version(&mut self) {
        self.minor_version = ::std::option::Option::None;
    }

    pub fn has_minor_version(&self) -> bool {
        self.minor_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minor_version(&mut self, v: i32) {
        self.minor_version = ::std::option::Option::Some(v);
    }

    pub fn get_minor_version(&self) -> i32 {
        self.minor_version.unwrap_or(0)
    }

    // required int32 patch_version = 3;

    pub fn clear_patch_version(&mut self) {
        self.patch_version = ::std::option::Option::None;
    }

    pub fn has_patch_version(&self) -> bool {
        self.patch_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_patch_version(&mut self, v: i32) {
        self.patch_version = ::std::option::Option::Some(v);
    }

    pub fn get_patch_version(&self) -> i32 {
        self.patch_version.unwrap_or(0)
    }
}

impl ::protobuf::Message for GetVersionResponse {
    fn is_initialized(&self) -> bool {
        if self.major_version.is_none() {
            return false;
        };
        if self.minor_version.is_none() {
            return false;
        };
        if self.patch_version.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.major_version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.minor_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.patch_version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.major_version.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.minor_version.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.patch_version.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.major_version {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.minor_version {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.patch_version {
            try!(os.write_int32(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetVersionResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetVersionResponse {
    fn new() -> GetVersionResponse {
        GetVersionResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetVersionResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "major_version",
                    GetVersionResponse::has_major_version,
                    GetVersionResponse::get_major_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "minor_version",
                    GetVersionResponse::has_minor_version,
                    GetVersionResponse::get_minor_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "patch_version",
                    GetVersionResponse::has_patch_version,
                    GetVersionResponse::get_patch_version,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetVersionResponse>(
                    "GetVersionResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetVersionResponse {
    fn clear(&mut self) {
        self.clear_major_version();
        self.clear_minor_version();
        self.clear_patch_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetVersionResponse {
    fn eq(&self, other: &GetVersionResponse) -> bool {
        self.major_version == other.major_version &&
        self.minor_version == other.minor_version &&
        self.patch_version == other.patch_version &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetVersionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Camera {
    // message fields
    id: ::std::option::Option<i64>,
    matrix_values: ::std::vec::Vec<f64>,
    projection_matrix_values: ::std::vec::Vec<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Camera {}

impl Camera {
    pub fn new() -> Camera {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Camera {
        static mut instance: ::protobuf::lazy::Lazy<Camera> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Camera,
        };
        unsafe {
            instance.get(|| {
                Camera {
                    id: ::std::option::Option::None,
                    matrix_values: ::std::vec::Vec::new(),
                    projection_matrix_values: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> i64 {
        self.id.unwrap_or(0)
    }

    // repeated double matrix_values = 2;

    pub fn clear_matrix_values(&mut self) {
        self.matrix_values.clear();
    }

    // Param is passed by value, moved
    pub fn set_matrix_values(&mut self, v: ::std::vec::Vec<f64>) {
        self.matrix_values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_matrix_values(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.matrix_values
    }

    // Take field
    pub fn take_matrix_values(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.matrix_values, ::std::vec::Vec::new())
    }

    pub fn get_matrix_values(&self) -> &[f64] {
        &self.matrix_values
    }

    // repeated double projection_matrix_values = 3;

    pub fn clear_projection_matrix_values(&mut self) {
        self.projection_matrix_values.clear();
    }

    // Param is passed by value, moved
    pub fn set_projection_matrix_values(&mut self, v: ::std::vec::Vec<f64>) {
        self.projection_matrix_values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_projection_matrix_values(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.projection_matrix_values
    }

    // Take field
    pub fn take_projection_matrix_values(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.projection_matrix_values, ::std::vec::Vec::new())
    }

    pub fn get_projection_matrix_values(&self) -> &[f64] {
        &self.projection_matrix_values
    }
}

impl ::protobuf::Message for Camera {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.matrix_values));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.projection_matrix_values));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.matrix_values.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.matrix_values.len() as u32) + (self.matrix_values.len() * 8) as u32;
        };
        if !self.projection_matrix_values.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.projection_matrix_values.len() as u32) + (self.projection_matrix_values.len() * 8) as u32;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_int64(1, v));
        };
        if !self.matrix_values.is_empty() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.matrix_values.len() * 8) as u32));
            for v in self.matrix_values.iter() {
                try!(os.write_double_no_tag(*v));
            };
        };
        if !self.projection_matrix_values.is_empty() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.projection_matrix_values.len() * 8) as u32));
            for v in self.projection_matrix_values.iter() {
                try!(os.write_double_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Camera>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Camera {
    fn new() -> Camera {
        Camera::new()
    }

    fn descriptor_static(_: ::std::option::Option<Camera>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "id",
                    Camera::has_id,
                    Camera::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "matrix_values",
                    Camera::get_matrix_values,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "projection_matrix_values",
                    Camera::get_projection_matrix_values,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Camera>(
                    "Camera",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Camera {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_matrix_values();
        self.clear_projection_matrix_values();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Camera {
    fn eq(&self, other: &Camera) -> bool {
        self.id == other.id &&
        self.matrix_values == other.matrix_values &&
        self.projection_matrix_values == other.projection_matrix_values &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Camera {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CreateRendererRequest {
    // message fields
    r_type: ::std::option::Option<CreateRendererRequest_RendererType>,
    options: ::protobuf::RepeatedField<CreateRendererRequest_RendererOption>,
    cameras: ::protobuf::RepeatedField<Camera>,
    viewer_matrix_values: ::std::vec::Vec<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateRendererRequest {}

impl CreateRendererRequest {
    pub fn new() -> CreateRendererRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateRendererRequest {
        static mut instance: ::protobuf::lazy::Lazy<CreateRendererRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateRendererRequest,
        };
        unsafe {
            instance.get(|| {
                CreateRendererRequest {
                    r_type: ::std::option::Option::None,
                    options: ::protobuf::RepeatedField::new(),
                    cameras: ::protobuf::RepeatedField::new(),
                    viewer_matrix_values: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .CreateRendererRequest.RendererType r_type = 1;

    pub fn clear_r_type(&mut self) {
        self.r_type = ::std::option::Option::None;
    }

    pub fn has_r_type(&self) -> bool {
        self.r_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_r_type(&mut self, v: CreateRendererRequest_RendererType) {
        self.r_type = ::std::option::Option::Some(v);
    }

    pub fn get_r_type(&self) -> CreateRendererRequest_RendererType {
        self.r_type.unwrap_or(CreateRendererRequest_RendererType::DISPLAY)
    }

    // repeated .CreateRendererRequest.RendererOption options = 2;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: ::protobuf::RepeatedField<CreateRendererRequest_RendererOption>) {
        self.options = v;
    }

    // Mutable pointer to the field.
    pub fn mut_options(&mut self) -> &mut ::protobuf::RepeatedField<CreateRendererRequest_RendererOption> {
        &mut self.options
    }

    // Take field
    pub fn take_options(&mut self) -> ::protobuf::RepeatedField<CreateRendererRequest_RendererOption> {
        ::std::mem::replace(&mut self.options, ::protobuf::RepeatedField::new())
    }

    pub fn get_options(&self) -> &[CreateRendererRequest_RendererOption] {
        &self.options
    }

    // repeated .Camera cameras = 3;

    pub fn clear_cameras(&mut self) {
        self.cameras.clear();
    }

    // Param is passed by value, moved
    pub fn set_cameras(&mut self, v: ::protobuf::RepeatedField<Camera>) {
        self.cameras = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cameras(&mut self) -> &mut ::protobuf::RepeatedField<Camera> {
        &mut self.cameras
    }

    // Take field
    pub fn take_cameras(&mut self) -> ::protobuf::RepeatedField<Camera> {
        ::std::mem::replace(&mut self.cameras, ::protobuf::RepeatedField::new())
    }

    pub fn get_cameras(&self) -> &[Camera] {
        &self.cameras
    }

    // repeated double viewer_matrix_values = 4;

    pub fn clear_viewer_matrix_values(&mut self) {
        self.viewer_matrix_values.clear();
    }

    // Param is passed by value, moved
    pub fn set_viewer_matrix_values(&mut self, v: ::std::vec::Vec<f64>) {
        self.viewer_matrix_values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_viewer_matrix_values(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.viewer_matrix_values
    }

    // Take field
    pub fn take_viewer_matrix_values(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.viewer_matrix_values, ::std::vec::Vec::new())
    }

    pub fn get_viewer_matrix_values(&self) -> &[f64] {
        &self.viewer_matrix_values
    }
}

impl ::protobuf::Message for CreateRendererRequest {
    fn is_initialized(&self) -> bool {
        if self.r_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.r_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.options));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cameras));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.viewer_matrix_values));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.r_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.options.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cameras.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.viewer_matrix_values.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.viewer_matrix_values.len() as u32) + (self.viewer_matrix_values.len() * 8) as u32;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.r_type {
            try!(os.write_enum(1, v.value()));
        };
        for v in self.options.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.cameras.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if !self.viewer_matrix_values.is_empty() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.viewer_matrix_values.len() * 8) as u32));
            for v in self.viewer_matrix_values.iter() {
                try!(os.write_double_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CreateRendererRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CreateRendererRequest {
    fn new() -> CreateRendererRequest {
        CreateRendererRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateRendererRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "r_type",
                    CreateRendererRequest::has_r_type,
                    CreateRendererRequest::get_r_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "options",
                    CreateRendererRequest::get_options,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "cameras",
                    CreateRendererRequest::get_cameras,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "viewer_matrix_values",
                    CreateRendererRequest::get_viewer_matrix_values,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateRendererRequest>(
                    "CreateRendererRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateRendererRequest {
    fn clear(&mut self) {
        self.clear_r_type();
        self.clear_options();
        self.clear_cameras();
        self.clear_viewer_matrix_values();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CreateRendererRequest {
    fn eq(&self, other: &CreateRendererRequest) -> bool {
        self.r_type == other.r_type &&
        self.options == other.options &&
        self.cameras == other.cameras &&
        self.viewer_matrix_values == other.viewer_matrix_values &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CreateRendererRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CreateRendererRequest_RendererOption {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateRendererRequest_RendererOption {}

impl CreateRendererRequest_RendererOption {
    pub fn new() -> CreateRendererRequest_RendererOption {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateRendererRequest_RendererOption {
        static mut instance: ::protobuf::lazy::Lazy<CreateRendererRequest_RendererOption> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateRendererRequest_RendererOption,
        };
        unsafe {
            instance.get(|| {
                CreateRendererRequest_RendererOption {
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        self.value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        match self.value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CreateRendererRequest_RendererOption {
    fn is_initialized(&self) -> bool {
        if self.key.is_none() {
            return false;
        };
        if self.value.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.value));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.key.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.value.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CreateRendererRequest_RendererOption>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CreateRendererRequest_RendererOption {
    fn new() -> CreateRendererRequest_RendererOption {
        CreateRendererRequest_RendererOption::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateRendererRequest_RendererOption>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "key",
                    CreateRendererRequest_RendererOption::has_key,
                    CreateRendererRequest_RendererOption::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "value",
                    CreateRendererRequest_RendererOption::has_value,
                    CreateRendererRequest_RendererOption::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateRendererRequest_RendererOption>(
                    "CreateRendererRequest_RendererOption",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateRendererRequest_RendererOption {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CreateRendererRequest_RendererOption {
    fn eq(&self, other: &CreateRendererRequest_RendererOption) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CreateRendererRequest_RendererOption {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CreateRendererRequest_RendererType {
    DISPLAY = 1,
    WEBGL = 2,
}

impl ::protobuf::ProtobufEnum for CreateRendererRequest_RendererType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CreateRendererRequest_RendererType> {
        match value {
            1 => ::std::option::Option::Some(CreateRendererRequest_RendererType::DISPLAY),
            2 => ::std::option::Option::Some(CreateRendererRequest_RendererType::WEBGL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CreateRendererRequest_RendererType] = &[
            CreateRendererRequest_RendererType::DISPLAY,
            CreateRendererRequest_RendererType::WEBGL,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CreateRendererRequest_RendererType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CreateRendererRequest_RendererType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CreateRendererRequest_RendererType {
}

#[derive(Clone,Default)]
pub struct DeleteRendererRequest {
    // message fields
    renderer_id: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteRendererRequest {}

impl DeleteRendererRequest {
    pub fn new() -> DeleteRendererRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteRendererRequest {
        static mut instance: ::protobuf::lazy::Lazy<DeleteRendererRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteRendererRequest,
        };
        unsafe {
            instance.get(|| {
                DeleteRendererRequest {
                    renderer_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 renderer_id = 1;

    pub fn clear_renderer_id(&mut self) {
        self.renderer_id = ::std::option::Option::None;
    }

    pub fn has_renderer_id(&self) -> bool {
        self.renderer_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_renderer_id(&mut self, v: i64) {
        self.renderer_id = ::std::option::Option::Some(v);
    }

    pub fn get_renderer_id(&self) -> i64 {
        self.renderer_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for DeleteRendererRequest {
    fn is_initialized(&self) -> bool {
        if self.renderer_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.renderer_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.renderer_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.renderer_id {
            try!(os.write_int64(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DeleteRendererRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteRendererRequest {
    fn new() -> DeleteRendererRequest {
        DeleteRendererRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteRendererRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "renderer_id",
                    DeleteRendererRequest::has_renderer_id,
                    DeleteRendererRequest::get_renderer_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteRendererRequest>(
                    "DeleteRendererRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteRendererRequest {
    fn clear(&mut self) {
        self.clear_renderer_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DeleteRendererRequest {
    fn eq(&self, other: &DeleteRendererRequest) -> bool {
        self.renderer_id == other.renderer_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DeleteRendererRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CreateCameraRequest {
    // message fields
    renderer_id: ::std::option::Option<i64>,
    camera: ::protobuf::RepeatedField<Camera>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateCameraRequest {}

impl CreateCameraRequest {
    pub fn new() -> CreateCameraRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateCameraRequest {
        static mut instance: ::protobuf::lazy::Lazy<CreateCameraRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateCameraRequest,
        };
        unsafe {
            instance.get(|| {
                CreateCameraRequest {
                    renderer_id: ::std::option::Option::None,
                    camera: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 renderer_id = 1;

    pub fn clear_renderer_id(&mut self) {
        self.renderer_id = ::std::option::Option::None;
    }

    pub fn has_renderer_id(&self) -> bool {
        self.renderer_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_renderer_id(&mut self, v: i64) {
        self.renderer_id = ::std::option::Option::Some(v);
    }

    pub fn get_renderer_id(&self) -> i64 {
        self.renderer_id.unwrap_or(0)
    }

    // repeated .Camera camera = 2;

    pub fn clear_camera(&mut self) {
        self.camera.clear();
    }

    // Param is passed by value, moved
    pub fn set_camera(&mut self, v: ::protobuf::RepeatedField<Camera>) {
        self.camera = v;
    }

    // Mutable pointer to the field.
    pub fn mut_camera(&mut self) -> &mut ::protobuf::RepeatedField<Camera> {
        &mut self.camera
    }

    // Take field
    pub fn take_camera(&mut self) -> ::protobuf::RepeatedField<Camera> {
        ::std::mem::replace(&mut self.camera, ::protobuf::RepeatedField::new())
    }

    pub fn get_camera(&self) -> &[Camera] {
        &self.camera
    }
}

impl ::protobuf::Message for CreateCameraRequest {
    fn is_initialized(&self) -> bool {
        if self.renderer_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.renderer_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.camera));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.renderer_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.camera.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.renderer_id {
            try!(os.write_int64(1, v));
        };
        for v in self.camera.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CreateCameraRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CreateCameraRequest {
    fn new() -> CreateCameraRequest {
        CreateCameraRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateCameraRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "renderer_id",
                    CreateCameraRequest::has_renderer_id,
                    CreateCameraRequest::get_renderer_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "camera",
                    CreateCameraRequest::get_camera,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateCameraRequest>(
                    "CreateCameraRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateCameraRequest {
    fn clear(&mut self) {
        self.clear_renderer_id();
        self.clear_camera();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CreateCameraRequest {
    fn eq(&self, other: &CreateCameraRequest) -> bool {
        self.renderer_id == other.renderer_id &&
        self.camera == other.camera &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CreateCameraRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DeleteCameraRequest {
    // message fields
    renderer_id: ::std::option::Option<i64>,
    camera_id: ::std::vec::Vec<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteCameraRequest {}

impl DeleteCameraRequest {
    pub fn new() -> DeleteCameraRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteCameraRequest {
        static mut instance: ::protobuf::lazy::Lazy<DeleteCameraRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteCameraRequest,
        };
        unsafe {
            instance.get(|| {
                DeleteCameraRequest {
                    renderer_id: ::std::option::Option::None,
                    camera_id: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 renderer_id = 1;

    pub fn clear_renderer_id(&mut self) {
        self.renderer_id = ::std::option::Option::None;
    }

    pub fn has_renderer_id(&self) -> bool {
        self.renderer_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_renderer_id(&mut self, v: i64) {
        self.renderer_id = ::std::option::Option::Some(v);
    }

    pub fn get_renderer_id(&self) -> i64 {
        self.renderer_id.unwrap_or(0)
    }

    // repeated int64 camera_id = 2;

    pub fn clear_camera_id(&mut self) {
        self.camera_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_camera_id(&mut self, v: ::std::vec::Vec<i64>) {
        self.camera_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_camera_id(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.camera_id
    }

    // Take field
    pub fn take_camera_id(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.camera_id, ::std::vec::Vec::new())
    }

    pub fn get_camera_id(&self) -> &[i64] {
        &self.camera_id
    }
}

impl ::protobuf::Message for DeleteCameraRequest {
    fn is_initialized(&self) -> bool {
        if self.renderer_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.renderer_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.camera_id));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.renderer_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.camera_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.renderer_id {
            try!(os.write_int64(1, v));
        };
        for v in self.camera_id.iter() {
            try!(os.write_int64(2, *v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DeleteCameraRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteCameraRequest {
    fn new() -> DeleteCameraRequest {
        DeleteCameraRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteCameraRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "renderer_id",
                    DeleteCameraRequest::has_renderer_id,
                    DeleteCameraRequest::get_renderer_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "camera_id",
                    DeleteCameraRequest::get_camera_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteCameraRequest>(
                    "DeleteCameraRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteCameraRequest {
    fn clear(&mut self) {
        self.clear_renderer_id();
        self.clear_camera_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DeleteCameraRequest {
    fn eq(&self, other: &DeleteCameraRequest) -> bool {
        self.renderer_id == other.renderer_id &&
        self.camera_id == other.camera_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DeleteCameraRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UpdateViewerMatrixRequest {
    // message fields
    renderer_id: ::std::option::Option<i64>,
    matrix_values: ::std::vec::Vec<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpdateViewerMatrixRequest {}

impl UpdateViewerMatrixRequest {
    pub fn new() -> UpdateViewerMatrixRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpdateViewerMatrixRequest {
        static mut instance: ::protobuf::lazy::Lazy<UpdateViewerMatrixRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpdateViewerMatrixRequest,
        };
        unsafe {
            instance.get(|| {
                UpdateViewerMatrixRequest {
                    renderer_id: ::std::option::Option::None,
                    matrix_values: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 renderer_id = 1;

    pub fn clear_renderer_id(&mut self) {
        self.renderer_id = ::std::option::Option::None;
    }

    pub fn has_renderer_id(&self) -> bool {
        self.renderer_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_renderer_id(&mut self, v: i64) {
        self.renderer_id = ::std::option::Option::Some(v);
    }

    pub fn get_renderer_id(&self) -> i64 {
        self.renderer_id.unwrap_or(0)
    }

    // repeated double matrix_values = 2;

    pub fn clear_matrix_values(&mut self) {
        self.matrix_values.clear();
    }

    // Param is passed by value, moved
    pub fn set_matrix_values(&mut self, v: ::std::vec::Vec<f64>) {
        self.matrix_values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_matrix_values(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.matrix_values
    }

    // Take field
    pub fn take_matrix_values(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.matrix_values, ::std::vec::Vec::new())
    }

    pub fn get_matrix_values(&self) -> &[f64] {
        &self.matrix_values
    }
}

impl ::protobuf::Message for UpdateViewerMatrixRequest {
    fn is_initialized(&self) -> bool {
        if self.renderer_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.renderer_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.matrix_values));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.renderer_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.matrix_values.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.matrix_values.len() as u32) + (self.matrix_values.len() * 8) as u32;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.renderer_id {
            try!(os.write_int64(1, v));
        };
        if !self.matrix_values.is_empty() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.matrix_values.len() * 8) as u32));
            for v in self.matrix_values.iter() {
                try!(os.write_double_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<UpdateViewerMatrixRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UpdateViewerMatrixRequest {
    fn new() -> UpdateViewerMatrixRequest {
        UpdateViewerMatrixRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpdateViewerMatrixRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "renderer_id",
                    UpdateViewerMatrixRequest::has_renderer_id,
                    UpdateViewerMatrixRequest::get_renderer_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "matrix_values",
                    UpdateViewerMatrixRequest::get_matrix_values,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpdateViewerMatrixRequest>(
                    "UpdateViewerMatrixRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpdateViewerMatrixRequest {
    fn clear(&mut self) {
        self.clear_renderer_id();
        self.clear_matrix_values();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UpdateViewerMatrixRequest {
    fn eq(&self, other: &UpdateViewerMatrixRequest) -> bool {
        self.renderer_id == other.renderer_id &&
        self.matrix_values == other.matrix_values &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UpdateViewerMatrixRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UpdateCameraMatrixRequest {
    // message fields
    renderer_id: ::std::option::Option<i64>,
    camera_id: ::std::option::Option<i64>,
    matrix_values: ::std::vec::Vec<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpdateCameraMatrixRequest {}

impl UpdateCameraMatrixRequest {
    pub fn new() -> UpdateCameraMatrixRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpdateCameraMatrixRequest {
        static mut instance: ::protobuf::lazy::Lazy<UpdateCameraMatrixRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpdateCameraMatrixRequest,
        };
        unsafe {
            instance.get(|| {
                UpdateCameraMatrixRequest {
                    renderer_id: ::std::option::Option::None,
                    camera_id: ::std::option::Option::None,
                    matrix_values: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 renderer_id = 1;

    pub fn clear_renderer_id(&mut self) {
        self.renderer_id = ::std::option::Option::None;
    }

    pub fn has_renderer_id(&self) -> bool {
        self.renderer_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_renderer_id(&mut self, v: i64) {
        self.renderer_id = ::std::option::Option::Some(v);
    }

    pub fn get_renderer_id(&self) -> i64 {
        self.renderer_id.unwrap_or(0)
    }

    // required int64 camera_id = 2;

    pub fn clear_camera_id(&mut self) {
        self.camera_id = ::std::option::Option::None;
    }

    pub fn has_camera_id(&self) -> bool {
        self.camera_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_camera_id(&mut self, v: i64) {
        self.camera_id = ::std::option::Option::Some(v);
    }

    pub fn get_camera_id(&self) -> i64 {
        self.camera_id.unwrap_or(0)
    }

    // repeated double matrix_values = 3;

    pub fn clear_matrix_values(&mut self) {
        self.matrix_values.clear();
    }

    // Param is passed by value, moved
    pub fn set_matrix_values(&mut self, v: ::std::vec::Vec<f64>) {
        self.matrix_values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_matrix_values(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.matrix_values
    }

    // Take field
    pub fn take_matrix_values(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.matrix_values, ::std::vec::Vec::new())
    }

    pub fn get_matrix_values(&self) -> &[f64] {
        &self.matrix_values
    }
}

impl ::protobuf::Message for UpdateCameraMatrixRequest {
    fn is_initialized(&self) -> bool {
        if self.renderer_id.is_none() {
            return false;
        };
        if self.camera_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.renderer_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.camera_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.matrix_values));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.renderer_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.camera_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.matrix_values.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.matrix_values.len() as u32) + (self.matrix_values.len() * 8) as u32;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.renderer_id {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.camera_id {
            try!(os.write_int64(2, v));
        };
        if !self.matrix_values.is_empty() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.matrix_values.len() * 8) as u32));
            for v in self.matrix_values.iter() {
                try!(os.write_double_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<UpdateCameraMatrixRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UpdateCameraMatrixRequest {
    fn new() -> UpdateCameraMatrixRequest {
        UpdateCameraMatrixRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpdateCameraMatrixRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "renderer_id",
                    UpdateCameraMatrixRequest::has_renderer_id,
                    UpdateCameraMatrixRequest::get_renderer_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "camera_id",
                    UpdateCameraMatrixRequest::has_camera_id,
                    UpdateCameraMatrixRequest::get_camera_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "matrix_values",
                    UpdateCameraMatrixRequest::get_matrix_values,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpdateCameraMatrixRequest>(
                    "UpdateCameraMatrixRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpdateCameraMatrixRequest {
    fn clear(&mut self) {
        self.clear_renderer_id();
        self.clear_camera_id();
        self.clear_matrix_values();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UpdateCameraMatrixRequest {
    fn eq(&self, other: &UpdateCameraMatrixRequest) -> bool {
        self.renderer_id == other.renderer_id &&
        self.camera_id == other.camera_id &&
        self.matrix_values == other.matrix_values &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UpdateCameraMatrixRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UpdateCameraProjectionRequest {
    // message fields
    renderer_id: ::std::option::Option<i64>,
    camera_id: ::std::option::Option<i64>,
    matrix_values: ::std::vec::Vec<f64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpdateCameraProjectionRequest {}

impl UpdateCameraProjectionRequest {
    pub fn new() -> UpdateCameraProjectionRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpdateCameraProjectionRequest {
        static mut instance: ::protobuf::lazy::Lazy<UpdateCameraProjectionRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpdateCameraProjectionRequest,
        };
        unsafe {
            instance.get(|| {
                UpdateCameraProjectionRequest {
                    renderer_id: ::std::option::Option::None,
                    camera_id: ::std::option::Option::None,
                    matrix_values: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 renderer_id = 1;

    pub fn clear_renderer_id(&mut self) {
        self.renderer_id = ::std::option::Option::None;
    }

    pub fn has_renderer_id(&self) -> bool {
        self.renderer_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_renderer_id(&mut self, v: i64) {
        self.renderer_id = ::std::option::Option::Some(v);
    }

    pub fn get_renderer_id(&self) -> i64 {
        self.renderer_id.unwrap_or(0)
    }

    // required int64 camera_id = 2;

    pub fn clear_camera_id(&mut self) {
        self.camera_id = ::std::option::Option::None;
    }

    pub fn has_camera_id(&self) -> bool {
        self.camera_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_camera_id(&mut self, v: i64) {
        self.camera_id = ::std::option::Option::Some(v);
    }

    pub fn get_camera_id(&self) -> i64 {
        self.camera_id.unwrap_or(0)
    }

    // repeated double matrix_values = 3;

    pub fn clear_matrix_values(&mut self) {
        self.matrix_values.clear();
    }

    // Param is passed by value, moved
    pub fn set_matrix_values(&mut self, v: ::std::vec::Vec<f64>) {
        self.matrix_values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_matrix_values(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.matrix_values
    }

    // Take field
    pub fn take_matrix_values(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.matrix_values, ::std::vec::Vec::new())
    }

    pub fn get_matrix_values(&self) -> &[f64] {
        &self.matrix_values
    }
}

impl ::protobuf::Message for UpdateCameraProjectionRequest {
    fn is_initialized(&self) -> bool {
        if self.renderer_id.is_none() {
            return false;
        };
        if self.camera_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.renderer_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.camera_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.matrix_values));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.renderer_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.camera_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.matrix_values.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.matrix_values.len() as u32) + (self.matrix_values.len() * 8) as u32;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.renderer_id {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.camera_id {
            try!(os.write_int64(2, v));
        };
        if !self.matrix_values.is_empty() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.matrix_values.len() * 8) as u32));
            for v in self.matrix_values.iter() {
                try!(os.write_double_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<UpdateCameraProjectionRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UpdateCameraProjectionRequest {
    fn new() -> UpdateCameraProjectionRequest {
        UpdateCameraProjectionRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpdateCameraProjectionRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "renderer_id",
                    UpdateCameraProjectionRequest::has_renderer_id,
                    UpdateCameraProjectionRequest::get_renderer_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "camera_id",
                    UpdateCameraProjectionRequest::has_camera_id,
                    UpdateCameraProjectionRequest::get_camera_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "matrix_values",
                    UpdateCameraProjectionRequest::get_matrix_values,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpdateCameraProjectionRequest>(
                    "UpdateCameraProjectionRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpdateCameraProjectionRequest {
    fn clear(&mut self) {
        self.clear_renderer_id();
        self.clear_camera_id();
        self.clear_matrix_values();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UpdateCameraProjectionRequest {
    fn eq(&self, other: &UpdateCameraProjectionRequest) -> bool {
        self.renderer_id == other.renderer_id &&
        self.camera_id == other.camera_id &&
        self.matrix_values == other.matrix_values &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UpdateCameraProjectionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Element {
    // message fields
    e_type: ::std::option::Option<Element_ElementType>,
    children: ::protobuf::RepeatedField<Element>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Element {}

impl Element {
    pub fn new() -> Element {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Element {
        static mut instance: ::protobuf::lazy::Lazy<Element> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Element,
        };
        unsafe {
            instance.get(|| {
                Element {
                    e_type: ::std::option::Option::None,
                    children: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .Element.ElementType e_type = 1;

    pub fn clear_e_type(&mut self) {
        self.e_type = ::std::option::Option::None;
    }

    pub fn has_e_type(&self) -> bool {
        self.e_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_e_type(&mut self, v: Element_ElementType) {
        self.e_type = ::std::option::Option::Some(v);
    }

    pub fn get_e_type(&self) -> Element_ElementType {
        self.e_type.unwrap_or(Element_ElementType::MESH)
    }

    // repeated .Element children = 2;

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    // Param is passed by value, moved
    pub fn set_children(&mut self, v: ::protobuf::RepeatedField<Element>) {
        self.children = v;
    }

    // Mutable pointer to the field.
    pub fn mut_children(&mut self) -> &mut ::protobuf::RepeatedField<Element> {
        &mut self.children
    }

    // Take field
    pub fn take_children(&mut self) -> ::protobuf::RepeatedField<Element> {
        ::std::mem::replace(&mut self.children, ::protobuf::RepeatedField::new())
    }

    pub fn get_children(&self) -> &[Element] {
        &self.children
    }
}

impl ::protobuf::Message for Element {
    fn is_initialized(&self) -> bool {
        if self.e_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.e_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.children));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.e_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.children.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.e_type {
            try!(os.write_enum(1, v.value()));
        };
        for v in self.children.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Element>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Element {
    fn new() -> Element {
        Element::new()
    }

    fn descriptor_static(_: ::std::option::Option<Element>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "e_type",
                    Element::has_e_type,
                    Element::get_e_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "children",
                    Element::get_children,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Element>(
                    "Element",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Element {
    fn clear(&mut self) {
        self.clear_e_type();
        self.clear_children();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Element {
    fn eq(&self, other: &Element) -> bool {
        self.e_type == other.e_type &&
        self.children == other.children &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Element {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Element_ElementType {
    MESH = 1,
}

impl ::protobuf::ProtobufEnum for Element_ElementType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Element_ElementType> {
        match value {
            1 => ::std::option::Option::Some(Element_ElementType::MESH),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Element_ElementType] = &[
            Element_ElementType::MESH,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Element_ElementType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Element_ElementType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Element_ElementType {
}

#[derive(Clone,Default)]
pub struct CreateGraphRequest {
    // message fields
    children: ::protobuf::RepeatedField<Element>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateGraphRequest {}

impl CreateGraphRequest {
    pub fn new() -> CreateGraphRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateGraphRequest {
        static mut instance: ::protobuf::lazy::Lazy<CreateGraphRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateGraphRequest,
        };
        unsafe {
            instance.get(|| {
                CreateGraphRequest {
                    children: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .Element children = 1;

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    // Param is passed by value, moved
    pub fn set_children(&mut self, v: ::protobuf::RepeatedField<Element>) {
        self.children = v;
    }

    // Mutable pointer to the field.
    pub fn mut_children(&mut self) -> &mut ::protobuf::RepeatedField<Element> {
        &mut self.children
    }

    // Take field
    pub fn take_children(&mut self) -> ::protobuf::RepeatedField<Element> {
        ::std::mem::replace(&mut self.children, ::protobuf::RepeatedField::new())
    }

    pub fn get_children(&self) -> &[Element] {
        &self.children
    }
}

impl ::protobuf::Message for CreateGraphRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.children));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.children.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.children.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CreateGraphRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CreateGraphRequest {
    fn new() -> CreateGraphRequest {
        CreateGraphRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateGraphRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "children",
                    CreateGraphRequest::get_children,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateGraphRequest>(
                    "CreateGraphRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateGraphRequest {
    fn clear(&mut self) {
        self.clear_children();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CreateGraphRequest {
    fn eq(&self, other: &CreateGraphRequest) -> bool {
        self.children == other.children &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CreateGraphRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AddElementsRequest {
    // message fields
    graph_id: ::std::option::Option<i64>,
    parent_id: ::std::option::Option<i64>,
    previous_sibling_id: ::std::option::Option<i64>,
    added_elements: ::protobuf::RepeatedField<Element>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AddElementsRequest {}

impl AddElementsRequest {
    pub fn new() -> AddElementsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AddElementsRequest {
        static mut instance: ::protobuf::lazy::Lazy<AddElementsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AddElementsRequest,
        };
        unsafe {
            instance.get(|| {
                AddElementsRequest {
                    graph_id: ::std::option::Option::None,
                    parent_id: ::std::option::Option::None,
                    previous_sibling_id: ::std::option::Option::None,
                    added_elements: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 graph_id = 1;

    pub fn clear_graph_id(&mut self) {
        self.graph_id = ::std::option::Option::None;
    }

    pub fn has_graph_id(&self) -> bool {
        self.graph_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_graph_id(&mut self, v: i64) {
        self.graph_id = ::std::option::Option::Some(v);
    }

    pub fn get_graph_id(&self) -> i64 {
        self.graph_id.unwrap_or(0)
    }

    // required int64 parent_id = 2;

    pub fn clear_parent_id(&mut self) {
        self.parent_id = ::std::option::Option::None;
    }

    pub fn has_parent_id(&self) -> bool {
        self.parent_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parent_id(&mut self, v: i64) {
        self.parent_id = ::std::option::Option::Some(v);
    }

    pub fn get_parent_id(&self) -> i64 {
        self.parent_id.unwrap_or(0)
    }

    // optional int64 previous_sibling_id = 3;

    pub fn clear_previous_sibling_id(&mut self) {
        self.previous_sibling_id = ::std::option::Option::None;
    }

    pub fn has_previous_sibling_id(&self) -> bool {
        self.previous_sibling_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_previous_sibling_id(&mut self, v: i64) {
        self.previous_sibling_id = ::std::option::Option::Some(v);
    }

    pub fn get_previous_sibling_id(&self) -> i64 {
        self.previous_sibling_id.unwrap_or(0)
    }

    // repeated .Element added_elements = 4;

    pub fn clear_added_elements(&mut self) {
        self.added_elements.clear();
    }

    // Param is passed by value, moved
    pub fn set_added_elements(&mut self, v: ::protobuf::RepeatedField<Element>) {
        self.added_elements = v;
    }

    // Mutable pointer to the field.
    pub fn mut_added_elements(&mut self) -> &mut ::protobuf::RepeatedField<Element> {
        &mut self.added_elements
    }

    // Take field
    pub fn take_added_elements(&mut self) -> ::protobuf::RepeatedField<Element> {
        ::std::mem::replace(&mut self.added_elements, ::protobuf::RepeatedField::new())
    }

    pub fn get_added_elements(&self) -> &[Element] {
        &self.added_elements
    }
}

impl ::protobuf::Message for AddElementsRequest {
    fn is_initialized(&self) -> bool {
        if self.graph_id.is_none() {
            return false;
        };
        if self.parent_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.graph_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.parent_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.previous_sibling_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.added_elements));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.graph_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.parent_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.previous_sibling_id.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.added_elements.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.graph_id {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.parent_id {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.previous_sibling_id {
            try!(os.write_int64(3, v));
        };
        for v in self.added_elements.iter() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AddElementsRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AddElementsRequest {
    fn new() -> AddElementsRequest {
        AddElementsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AddElementsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "graph_id",
                    AddElementsRequest::has_graph_id,
                    AddElementsRequest::get_graph_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "parent_id",
                    AddElementsRequest::has_parent_id,
                    AddElementsRequest::get_parent_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "previous_sibling_id",
                    AddElementsRequest::has_previous_sibling_id,
                    AddElementsRequest::get_previous_sibling_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "added_elements",
                    AddElementsRequest::get_added_elements,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AddElementsRequest>(
                    "AddElementsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AddElementsRequest {
    fn clear(&mut self) {
        self.clear_graph_id();
        self.clear_parent_id();
        self.clear_previous_sibling_id();
        self.clear_added_elements();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AddElementsRequest {
    fn eq(&self, other: &AddElementsRequest) -> bool {
        self.graph_id == other.graph_id &&
        self.parent_id == other.parent_id &&
        self.previous_sibling_id == other.previous_sibling_id &&
        self.added_elements == other.added_elements &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AddElementsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UpdateElementsRequest {
    // message fields
    graph_id: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpdateElementsRequest {}

impl UpdateElementsRequest {
    pub fn new() -> UpdateElementsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpdateElementsRequest {
        static mut instance: ::protobuf::lazy::Lazy<UpdateElementsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpdateElementsRequest,
        };
        unsafe {
            instance.get(|| {
                UpdateElementsRequest {
                    graph_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 graph_id = 1;

    pub fn clear_graph_id(&mut self) {
        self.graph_id = ::std::option::Option::None;
    }

    pub fn has_graph_id(&self) -> bool {
        self.graph_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_graph_id(&mut self, v: i64) {
        self.graph_id = ::std::option::Option::Some(v);
    }

    pub fn get_graph_id(&self) -> i64 {
        self.graph_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for UpdateElementsRequest {
    fn is_initialized(&self) -> bool {
        if self.graph_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.graph_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.graph_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.graph_id {
            try!(os.write_int64(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<UpdateElementsRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UpdateElementsRequest {
    fn new() -> UpdateElementsRequest {
        UpdateElementsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpdateElementsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "graph_id",
                    UpdateElementsRequest::has_graph_id,
                    UpdateElementsRequest::get_graph_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpdateElementsRequest>(
                    "UpdateElementsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpdateElementsRequest {
    fn clear(&mut self) {
        self.clear_graph_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UpdateElementsRequest {
    fn eq(&self, other: &UpdateElementsRequest) -> bool {
        self.graph_id == other.graph_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UpdateElementsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DeleteElementsRequest {
    // message fields
    graph_id: ::std::option::Option<i64>,
    deleted_element_ids: ::std::vec::Vec<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteElementsRequest {}

impl DeleteElementsRequest {
    pub fn new() -> DeleteElementsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteElementsRequest {
        static mut instance: ::protobuf::lazy::Lazy<DeleteElementsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteElementsRequest,
        };
        unsafe {
            instance.get(|| {
                DeleteElementsRequest {
                    graph_id: ::std::option::Option::None,
                    deleted_element_ids: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 graph_id = 1;

    pub fn clear_graph_id(&mut self) {
        self.graph_id = ::std::option::Option::None;
    }

    pub fn has_graph_id(&self) -> bool {
        self.graph_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_graph_id(&mut self, v: i64) {
        self.graph_id = ::std::option::Option::Some(v);
    }

    pub fn get_graph_id(&self) -> i64 {
        self.graph_id.unwrap_or(0)
    }

    // repeated int64 deleted_element_ids = 2;

    pub fn clear_deleted_element_ids(&mut self) {
        self.deleted_element_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_deleted_element_ids(&mut self, v: ::std::vec::Vec<i64>) {
        self.deleted_element_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_deleted_element_ids(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.deleted_element_ids
    }

    // Take field
    pub fn take_deleted_element_ids(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.deleted_element_ids, ::std::vec::Vec::new())
    }

    pub fn get_deleted_element_ids(&self) -> &[i64] {
        &self.deleted_element_ids
    }
}

impl ::protobuf::Message for DeleteElementsRequest {
    fn is_initialized(&self) -> bool {
        if self.graph_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.graph_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.deleted_element_ids));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.graph_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.deleted_element_ids.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.graph_id {
            try!(os.write_int64(1, v));
        };
        for v in self.deleted_element_ids.iter() {
            try!(os.write_int64(2, *v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DeleteElementsRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteElementsRequest {
    fn new() -> DeleteElementsRequest {
        DeleteElementsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteElementsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "graph_id",
                    DeleteElementsRequest::has_graph_id,
                    DeleteElementsRequest::get_graph_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "deleted_element_ids",
                    DeleteElementsRequest::get_deleted_element_ids,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteElementsRequest>(
                    "DeleteElementsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteElementsRequest {
    fn clear(&mut self) {
        self.clear_graph_id();
        self.clear_deleted_element_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DeleteElementsRequest {
    fn eq(&self, other: &DeleteElementsRequest) -> bool {
        self.graph_id == other.graph_id &&
        self.deleted_element_ids == other.deleted_element_ids &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DeleteElementsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Request {
    // message fields
    r_type: ::std::option::Option<Request_RequestType>,
    create_renderer_request: ::protobuf::SingularPtrField<CreateRendererRequest>,
    delete_renderer_request: ::protobuf::SingularPtrField<DeleteRendererRequest>,
    update_viewer_matrix_request: ::protobuf::SingularPtrField<UpdateViewerMatrixRequest>,
    create_camera_request: ::protobuf::SingularPtrField<CreateCameraRequest>,
    delete_camera_request: ::protobuf::SingularPtrField<DeleteCameraRequest>,
    update_camera_matrix_request: ::protobuf::SingularPtrField<UpdateCameraMatrixRequest>,
    update_camera_projection_request: ::protobuf::SingularPtrField<UpdateCameraProjectionRequest>,
    create_graph_request: ::protobuf::SingularPtrField<CreateGraphRequest>,
    add_elements_request: ::protobuf::SingularPtrField<AddElementsRequest>,
    update_elements_request: ::protobuf::SingularPtrField<UpdateElementsRequest>,
    delete_elements_request: ::protobuf::SingularPtrField<DeleteElementsRequest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Request {}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe {
            instance.get(|| {
                Request {
                    r_type: ::std::option::Option::None,
                    create_renderer_request: ::protobuf::SingularPtrField::none(),
                    delete_renderer_request: ::protobuf::SingularPtrField::none(),
                    update_viewer_matrix_request: ::protobuf::SingularPtrField::none(),
                    create_camera_request: ::protobuf::SingularPtrField::none(),
                    delete_camera_request: ::protobuf::SingularPtrField::none(),
                    update_camera_matrix_request: ::protobuf::SingularPtrField::none(),
                    update_camera_projection_request: ::protobuf::SingularPtrField::none(),
                    create_graph_request: ::protobuf::SingularPtrField::none(),
                    add_elements_request: ::protobuf::SingularPtrField::none(),
                    update_elements_request: ::protobuf::SingularPtrField::none(),
                    delete_elements_request: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .Request.RequestType r_type = 1;

    pub fn clear_r_type(&mut self) {
        self.r_type = ::std::option::Option::None;
    }

    pub fn has_r_type(&self) -> bool {
        self.r_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_r_type(&mut self, v: Request_RequestType) {
        self.r_type = ::std::option::Option::Some(v);
    }

    pub fn get_r_type(&self) -> Request_RequestType {
        self.r_type.unwrap_or(Request_RequestType::GET_VERSION)
    }

    // optional .CreateRendererRequest create_renderer_request = 2;

    pub fn clear_create_renderer_request(&mut self) {
        self.create_renderer_request.clear();
    }

    pub fn has_create_renderer_request(&self) -> bool {
        self.create_renderer_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_create_renderer_request(&mut self, v: CreateRendererRequest) {
        self.create_renderer_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_create_renderer_request(&mut self) -> &mut CreateRendererRequest {
        if self.create_renderer_request.is_none() {
            self.create_renderer_request.set_default();
        };
        self.create_renderer_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_create_renderer_request(&mut self) -> CreateRendererRequest {
        self.create_renderer_request.take().unwrap_or_else(|| CreateRendererRequest::new())
    }

    pub fn get_create_renderer_request(&self) -> &CreateRendererRequest {
        self.create_renderer_request.as_ref().unwrap_or_else(|| CreateRendererRequest::default_instance())
    }

    // optional .DeleteRendererRequest delete_renderer_request = 3;

    pub fn clear_delete_renderer_request(&mut self) {
        self.delete_renderer_request.clear();
    }

    pub fn has_delete_renderer_request(&self) -> bool {
        self.delete_renderer_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delete_renderer_request(&mut self, v: DeleteRendererRequest) {
        self.delete_renderer_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete_renderer_request(&mut self) -> &mut DeleteRendererRequest {
        if self.delete_renderer_request.is_none() {
            self.delete_renderer_request.set_default();
        };
        self.delete_renderer_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_delete_renderer_request(&mut self) -> DeleteRendererRequest {
        self.delete_renderer_request.take().unwrap_or_else(|| DeleteRendererRequest::new())
    }

    pub fn get_delete_renderer_request(&self) -> &DeleteRendererRequest {
        self.delete_renderer_request.as_ref().unwrap_or_else(|| DeleteRendererRequest::default_instance())
    }

    // optional .UpdateViewerMatrixRequest update_viewer_matrix_request = 4;

    pub fn clear_update_viewer_matrix_request(&mut self) {
        self.update_viewer_matrix_request.clear();
    }

    pub fn has_update_viewer_matrix_request(&self) -> bool {
        self.update_viewer_matrix_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_viewer_matrix_request(&mut self, v: UpdateViewerMatrixRequest) {
        self.update_viewer_matrix_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_viewer_matrix_request(&mut self) -> &mut UpdateViewerMatrixRequest {
        if self.update_viewer_matrix_request.is_none() {
            self.update_viewer_matrix_request.set_default();
        };
        self.update_viewer_matrix_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_viewer_matrix_request(&mut self) -> UpdateViewerMatrixRequest {
        self.update_viewer_matrix_request.take().unwrap_or_else(|| UpdateViewerMatrixRequest::new())
    }

    pub fn get_update_viewer_matrix_request(&self) -> &UpdateViewerMatrixRequest {
        self.update_viewer_matrix_request.as_ref().unwrap_or_else(|| UpdateViewerMatrixRequest::default_instance())
    }

    // optional .CreateCameraRequest create_camera_request = 5;

    pub fn clear_create_camera_request(&mut self) {
        self.create_camera_request.clear();
    }

    pub fn has_create_camera_request(&self) -> bool {
        self.create_camera_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_create_camera_request(&mut self, v: CreateCameraRequest) {
        self.create_camera_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_create_camera_request(&mut self) -> &mut CreateCameraRequest {
        if self.create_camera_request.is_none() {
            self.create_camera_request.set_default();
        };
        self.create_camera_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_create_camera_request(&mut self) -> CreateCameraRequest {
        self.create_camera_request.take().unwrap_or_else(|| CreateCameraRequest::new())
    }

    pub fn get_create_camera_request(&self) -> &CreateCameraRequest {
        self.create_camera_request.as_ref().unwrap_or_else(|| CreateCameraRequest::default_instance())
    }

    // optional .DeleteCameraRequest delete_camera_request = 6;

    pub fn clear_delete_camera_request(&mut self) {
        self.delete_camera_request.clear();
    }

    pub fn has_delete_camera_request(&self) -> bool {
        self.delete_camera_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delete_camera_request(&mut self, v: DeleteCameraRequest) {
        self.delete_camera_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete_camera_request(&mut self) -> &mut DeleteCameraRequest {
        if self.delete_camera_request.is_none() {
            self.delete_camera_request.set_default();
        };
        self.delete_camera_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_delete_camera_request(&mut self) -> DeleteCameraRequest {
        self.delete_camera_request.take().unwrap_or_else(|| DeleteCameraRequest::new())
    }

    pub fn get_delete_camera_request(&self) -> &DeleteCameraRequest {
        self.delete_camera_request.as_ref().unwrap_or_else(|| DeleteCameraRequest::default_instance())
    }

    // optional .UpdateCameraMatrixRequest update_camera_matrix_request = 7;

    pub fn clear_update_camera_matrix_request(&mut self) {
        self.update_camera_matrix_request.clear();
    }

    pub fn has_update_camera_matrix_request(&self) -> bool {
        self.update_camera_matrix_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_camera_matrix_request(&mut self, v: UpdateCameraMatrixRequest) {
        self.update_camera_matrix_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_camera_matrix_request(&mut self) -> &mut UpdateCameraMatrixRequest {
        if self.update_camera_matrix_request.is_none() {
            self.update_camera_matrix_request.set_default();
        };
        self.update_camera_matrix_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_camera_matrix_request(&mut self) -> UpdateCameraMatrixRequest {
        self.update_camera_matrix_request.take().unwrap_or_else(|| UpdateCameraMatrixRequest::new())
    }

    pub fn get_update_camera_matrix_request(&self) -> &UpdateCameraMatrixRequest {
        self.update_camera_matrix_request.as_ref().unwrap_or_else(|| UpdateCameraMatrixRequest::default_instance())
    }

    // optional .UpdateCameraProjectionRequest update_camera_projection_request = 8;

    pub fn clear_update_camera_projection_request(&mut self) {
        self.update_camera_projection_request.clear();
    }

    pub fn has_update_camera_projection_request(&self) -> bool {
        self.update_camera_projection_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_camera_projection_request(&mut self, v: UpdateCameraProjectionRequest) {
        self.update_camera_projection_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_camera_projection_request(&mut self) -> &mut UpdateCameraProjectionRequest {
        if self.update_camera_projection_request.is_none() {
            self.update_camera_projection_request.set_default();
        };
        self.update_camera_projection_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_camera_projection_request(&mut self) -> UpdateCameraProjectionRequest {
        self.update_camera_projection_request.take().unwrap_or_else(|| UpdateCameraProjectionRequest::new())
    }

    pub fn get_update_camera_projection_request(&self) -> &UpdateCameraProjectionRequest {
        self.update_camera_projection_request.as_ref().unwrap_or_else(|| UpdateCameraProjectionRequest::default_instance())
    }

    // optional .CreateGraphRequest create_graph_request = 9;

    pub fn clear_create_graph_request(&mut self) {
        self.create_graph_request.clear();
    }

    pub fn has_create_graph_request(&self) -> bool {
        self.create_graph_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_create_graph_request(&mut self, v: CreateGraphRequest) {
        self.create_graph_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_create_graph_request(&mut self) -> &mut CreateGraphRequest {
        if self.create_graph_request.is_none() {
            self.create_graph_request.set_default();
        };
        self.create_graph_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_create_graph_request(&mut self) -> CreateGraphRequest {
        self.create_graph_request.take().unwrap_or_else(|| CreateGraphRequest::new())
    }

    pub fn get_create_graph_request(&self) -> &CreateGraphRequest {
        self.create_graph_request.as_ref().unwrap_or_else(|| CreateGraphRequest::default_instance())
    }

    // optional .AddElementsRequest add_elements_request = 10;

    pub fn clear_add_elements_request(&mut self) {
        self.add_elements_request.clear();
    }

    pub fn has_add_elements_request(&self) -> bool {
        self.add_elements_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_add_elements_request(&mut self, v: AddElementsRequest) {
        self.add_elements_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_add_elements_request(&mut self) -> &mut AddElementsRequest {
        if self.add_elements_request.is_none() {
            self.add_elements_request.set_default();
        };
        self.add_elements_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_add_elements_request(&mut self) -> AddElementsRequest {
        self.add_elements_request.take().unwrap_or_else(|| AddElementsRequest::new())
    }

    pub fn get_add_elements_request(&self) -> &AddElementsRequest {
        self.add_elements_request.as_ref().unwrap_or_else(|| AddElementsRequest::default_instance())
    }

    // optional .UpdateElementsRequest update_elements_request = 11;

    pub fn clear_update_elements_request(&mut self) {
        self.update_elements_request.clear();
    }

    pub fn has_update_elements_request(&self) -> bool {
        self.update_elements_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_elements_request(&mut self, v: UpdateElementsRequest) {
        self.update_elements_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_elements_request(&mut self) -> &mut UpdateElementsRequest {
        if self.update_elements_request.is_none() {
            self.update_elements_request.set_default();
        };
        self.update_elements_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_elements_request(&mut self) -> UpdateElementsRequest {
        self.update_elements_request.take().unwrap_or_else(|| UpdateElementsRequest::new())
    }

    pub fn get_update_elements_request(&self) -> &UpdateElementsRequest {
        self.update_elements_request.as_ref().unwrap_or_else(|| UpdateElementsRequest::default_instance())
    }

    // optional .DeleteElementsRequest delete_elements_request = 12;

    pub fn clear_delete_elements_request(&mut self) {
        self.delete_elements_request.clear();
    }

    pub fn has_delete_elements_request(&self) -> bool {
        self.delete_elements_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delete_elements_request(&mut self, v: DeleteElementsRequest) {
        self.delete_elements_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete_elements_request(&mut self) -> &mut DeleteElementsRequest {
        if self.delete_elements_request.is_none() {
            self.delete_elements_request.set_default();
        };
        self.delete_elements_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_delete_elements_request(&mut self) -> DeleteElementsRequest {
        self.delete_elements_request.take().unwrap_or_else(|| DeleteElementsRequest::new())
    }

    pub fn get_delete_elements_request(&self) -> &DeleteElementsRequest {
        self.delete_elements_request.as_ref().unwrap_or_else(|| DeleteElementsRequest::default_instance())
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        if self.r_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.r_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.create_renderer_request));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.delete_renderer_request));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_viewer_matrix_request));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.create_camera_request));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.delete_camera_request));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_camera_matrix_request));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_camera_projection_request));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.create_graph_request));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.add_elements_request));
                },
                11 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_elements_request));
                },
                12 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.delete_elements_request));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.r_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.create_renderer_request.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.delete_renderer_request.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.update_viewer_matrix_request.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.create_camera_request.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.delete_camera_request.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.update_camera_matrix_request.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.update_camera_projection_request.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.create_graph_request.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.add_elements_request.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.update_elements_request.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.delete_elements_request.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.r_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.create_renderer_request.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.delete_renderer_request.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.update_viewer_matrix_request.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.create_camera_request.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.delete_camera_request.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.update_camera_matrix_request.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.update_camera_projection_request.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.create_graph_request.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.add_elements_request.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.update_elements_request.as_ref() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.delete_elements_request.as_ref() {
            try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Request>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Request {
    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "r_type",
                    Request::has_r_type,
                    Request::get_r_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "create_renderer_request",
                    Request::has_create_renderer_request,
                    Request::get_create_renderer_request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "delete_renderer_request",
                    Request::has_delete_renderer_request,
                    Request::get_delete_renderer_request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "update_viewer_matrix_request",
                    Request::has_update_viewer_matrix_request,
                    Request::get_update_viewer_matrix_request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "create_camera_request",
                    Request::has_create_camera_request,
                    Request::get_create_camera_request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "delete_camera_request",
                    Request::has_delete_camera_request,
                    Request::get_delete_camera_request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "update_camera_matrix_request",
                    Request::has_update_camera_matrix_request,
                    Request::get_update_camera_matrix_request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "update_camera_projection_request",
                    Request::has_update_camera_projection_request,
                    Request::get_update_camera_projection_request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "create_graph_request",
                    Request::has_create_graph_request,
                    Request::get_create_graph_request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "add_elements_request",
                    Request::has_add_elements_request,
                    Request::get_add_elements_request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "update_elements_request",
                    Request::has_update_elements_request,
                    Request::get_update_elements_request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "delete_elements_request",
                    Request::has_delete_elements_request,
                    Request::get_delete_elements_request,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.clear_r_type();
        self.clear_create_renderer_request();
        self.clear_delete_renderer_request();
        self.clear_update_viewer_matrix_request();
        self.clear_create_camera_request();
        self.clear_delete_camera_request();
        self.clear_update_camera_matrix_request();
        self.clear_update_camera_projection_request();
        self.clear_create_graph_request();
        self.clear_add_elements_request();
        self.clear_update_elements_request();
        self.clear_delete_elements_request();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Request {
    fn eq(&self, other: &Request) -> bool {
        self.r_type == other.r_type &&
        self.create_renderer_request == other.create_renderer_request &&
        self.delete_renderer_request == other.delete_renderer_request &&
        self.update_viewer_matrix_request == other.update_viewer_matrix_request &&
        self.create_camera_request == other.create_camera_request &&
        self.delete_camera_request == other.delete_camera_request &&
        self.update_camera_matrix_request == other.update_camera_matrix_request &&
        self.update_camera_projection_request == other.update_camera_projection_request &&
        self.create_graph_request == other.create_graph_request &&
        self.add_elements_request == other.add_elements_request &&
        self.update_elements_request == other.update_elements_request &&
        self.delete_elements_request == other.delete_elements_request &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Request_RequestType {
    GET_VERSION = 1,
    CREATE_RENDERER = 2,
    DELETE_RENDERER = 3,
    UPDATE_VIEWER_MATRIX = 4,
    CREATE_CAMERA = 5,
    DELETE_CAMERA = 6,
    UPDATE_CAMERA_MATRIX = 7,
    UDPATE_CAMERA_PROJECTION = 8,
    CREATE_GRAPH_REQUEST = 9,
    ADD_ELEMENTS_REQUEST = 10,
    UPDATE_ELEMENTS_REQUEST = 11,
    DELETE_ELEMENTS_REQUEST = 12,
}

impl ::protobuf::ProtobufEnum for Request_RequestType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Request_RequestType> {
        match value {
            1 => ::std::option::Option::Some(Request_RequestType::GET_VERSION),
            2 => ::std::option::Option::Some(Request_RequestType::CREATE_RENDERER),
            3 => ::std::option::Option::Some(Request_RequestType::DELETE_RENDERER),
            4 => ::std::option::Option::Some(Request_RequestType::UPDATE_VIEWER_MATRIX),
            5 => ::std::option::Option::Some(Request_RequestType::CREATE_CAMERA),
            6 => ::std::option::Option::Some(Request_RequestType::DELETE_CAMERA),
            7 => ::std::option::Option::Some(Request_RequestType::UPDATE_CAMERA_MATRIX),
            8 => ::std::option::Option::Some(Request_RequestType::UDPATE_CAMERA_PROJECTION),
            9 => ::std::option::Option::Some(Request_RequestType::CREATE_GRAPH_REQUEST),
            10 => ::std::option::Option::Some(Request_RequestType::ADD_ELEMENTS_REQUEST),
            11 => ::std::option::Option::Some(Request_RequestType::UPDATE_ELEMENTS_REQUEST),
            12 => ::std::option::Option::Some(Request_RequestType::DELETE_ELEMENTS_REQUEST),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Request_RequestType] = &[
            Request_RequestType::GET_VERSION,
            Request_RequestType::CREATE_RENDERER,
            Request_RequestType::DELETE_RENDERER,
            Request_RequestType::UPDATE_VIEWER_MATRIX,
            Request_RequestType::CREATE_CAMERA,
            Request_RequestType::DELETE_CAMERA,
            Request_RequestType::UPDATE_CAMERA_MATRIX,
            Request_RequestType::UDPATE_CAMERA_PROJECTION,
            Request_RequestType::CREATE_GRAPH_REQUEST,
            Request_RequestType::ADD_ELEMENTS_REQUEST,
            Request_RequestType::UPDATE_ELEMENTS_REQUEST,
            Request_RequestType::DELETE_ELEMENTS_REQUEST,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Request_RequestType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Request_RequestType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Request_RequestType {
}

#[derive(Clone,Default)]
pub struct Requests {
    // message fields
    requests: ::protobuf::RepeatedField<Request>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Requests {}

impl Requests {
    pub fn new() -> Requests {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Requests {
        static mut instance: ::protobuf::lazy::Lazy<Requests> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Requests,
        };
        unsafe {
            instance.get(|| {
                Requests {
                    requests: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .Request requests = 1;

    pub fn clear_requests(&mut self) {
        self.requests.clear();
    }

    // Param is passed by value, moved
    pub fn set_requests(&mut self, v: ::protobuf::RepeatedField<Request>) {
        self.requests = v;
    }

    // Mutable pointer to the field.
    pub fn mut_requests(&mut self) -> &mut ::protobuf::RepeatedField<Request> {
        &mut self.requests
    }

    // Take field
    pub fn take_requests(&mut self) -> ::protobuf::RepeatedField<Request> {
        ::std::mem::replace(&mut self.requests, ::protobuf::RepeatedField::new())
    }

    pub fn get_requests(&self) -> &[Request] {
        &self.requests
    }
}

impl ::protobuf::Message for Requests {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.requests));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.requests.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.requests.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Requests>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Requests {
    fn new() -> Requests {
        Requests::new()
    }

    fn descriptor_static(_: ::std::option::Option<Requests>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "requests",
                    Requests::get_requests,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Requests>(
                    "Requests",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Requests {
    fn clear(&mut self) {
        self.clear_requests();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Requests {
    fn eq(&self, other: &Requests) -> bool {
        self.requests == other.requests &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Requests {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Response {
    // message fields
    r_type: ::std::option::Option<Response_ResponseType>,
    index: ::std::option::Option<i64>,
    get_version_response: ::protobuf::SingularPtrField<GetVersionResponse>,
    item_created_response: ::protobuf::SingularPtrField<ItemCreatedResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response {}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response,
        };
        unsafe {
            instance.get(|| {
                Response {
                    r_type: ::std::option::Option::None,
                    index: ::std::option::Option::None,
                    get_version_response: ::protobuf::SingularPtrField::none(),
                    item_created_response: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .Response.ResponseType r_type = 1;

    pub fn clear_r_type(&mut self) {
        self.r_type = ::std::option::Option::None;
    }

    pub fn has_r_type(&self) -> bool {
        self.r_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_r_type(&mut self, v: Response_ResponseType) {
        self.r_type = ::std::option::Option::Some(v);
    }

    pub fn get_r_type(&self) -> Response_ResponseType {
        self.r_type.unwrap_or(Response_ResponseType::GET_VERSION)
    }

    // required int64 index = 2;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: i64) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> i64 {
        self.index.unwrap_or(0)
    }

    // optional .GetVersionResponse get_version_response = 3;

    pub fn clear_get_version_response(&mut self) {
        self.get_version_response.clear();
    }

    pub fn has_get_version_response(&self) -> bool {
        self.get_version_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get_version_response(&mut self, v: GetVersionResponse) {
        self.get_version_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get_version_response(&mut self) -> &mut GetVersionResponse {
        if self.get_version_response.is_none() {
            self.get_version_response.set_default();
        };
        self.get_version_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_get_version_response(&mut self) -> GetVersionResponse {
        self.get_version_response.take().unwrap_or_else(|| GetVersionResponse::new())
    }

    pub fn get_get_version_response(&self) -> &GetVersionResponse {
        self.get_version_response.as_ref().unwrap_or_else(|| GetVersionResponse::default_instance())
    }

    // optional .ItemCreatedResponse item_created_response = 4;

    pub fn clear_item_created_response(&mut self) {
        self.item_created_response.clear();
    }

    pub fn has_item_created_response(&self) -> bool {
        self.item_created_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_created_response(&mut self, v: ItemCreatedResponse) {
        self.item_created_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_item_created_response(&mut self) -> &mut ItemCreatedResponse {
        if self.item_created_response.is_none() {
            self.item_created_response.set_default();
        };
        self.item_created_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_item_created_response(&mut self) -> ItemCreatedResponse {
        self.item_created_response.take().unwrap_or_else(|| ItemCreatedResponse::new())
    }

    pub fn get_item_created_response(&self) -> &ItemCreatedResponse {
        self.item_created_response.as_ref().unwrap_or_else(|| ItemCreatedResponse::default_instance())
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        if self.r_type.is_none() {
            return false;
        };
        if self.index.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.r_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.index = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get_version_response));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.item_created_response));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.r_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.index.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.get_version_response.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.item_created_response.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.r_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.index {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.get_version_response.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.item_created_response.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Response>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Response {
    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "r_type",
                    Response::has_r_type,
                    Response::get_r_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "index",
                    Response::has_index,
                    Response::get_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_version_response",
                    Response::has_get_version_response,
                    Response::get_get_version_response,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "item_created_response",
                    Response::has_item_created_response,
                    Response::get_item_created_response,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.clear_r_type();
        self.clear_index();
        self.clear_get_version_response();
        self.clear_item_created_response();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Response {
    fn eq(&self, other: &Response) -> bool {
        self.r_type == other.r_type &&
        self.index == other.index &&
        self.get_version_response == other.get_version_response &&
        self.item_created_response == other.item_created_response &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Response_ResponseType {
    GET_VERSION = 1,
    ITEM_CREATED = 2,
}

impl ::protobuf::ProtobufEnum for Response_ResponseType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Response_ResponseType> {
        match value {
            1 => ::std::option::Option::Some(Response_ResponseType::GET_VERSION),
            2 => ::std::option::Option::Some(Response_ResponseType::ITEM_CREATED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Response_ResponseType] = &[
            Response_ResponseType::GET_VERSION,
            Response_ResponseType::ITEM_CREATED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Response_ResponseType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Response_ResponseType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Response_ResponseType {
}

#[derive(Clone,Default)]
pub struct Responses {
    // message fields
    responses: ::protobuf::RepeatedField<Response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Responses {}

impl Responses {
    pub fn new() -> Responses {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Responses {
        static mut instance: ::protobuf::lazy::Lazy<Responses> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Responses,
        };
        unsafe {
            instance.get(|| {
                Responses {
                    responses: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .Response responses = 1;

    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }

    // Param is passed by value, moved
    pub fn set_responses(&mut self, v: ::protobuf::RepeatedField<Response>) {
        self.responses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_responses(&mut self) -> &mut ::protobuf::RepeatedField<Response> {
        &mut self.responses
    }

    // Take field
    pub fn take_responses(&mut self) -> ::protobuf::RepeatedField<Response> {
        ::std::mem::replace(&mut self.responses, ::protobuf::RepeatedField::new())
    }

    pub fn get_responses(&self) -> &[Response] {
        &self.responses
    }
}

impl ::protobuf::Message for Responses {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.responses));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.responses.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.responses.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Responses>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Responses {
    fn new() -> Responses {
        Responses::new()
    }

    fn descriptor_static(_: ::std::option::Option<Responses>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "responses",
                    Responses::get_responses,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Responses>(
                    "Responses",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Responses {
    fn clear(&mut self) {
        self.clear_responses();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Responses {
    fn eq(&self, other: &Responses) -> bool {
        self.responses == other.responses &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Responses {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1b, 0x73, 0x72, 0x63, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2f, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x26, 0x0a,
    0x13, 0x49, 0x74, 0x65, 0x6d, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x0f, 0x0a, 0x07, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x03, 0x22, 0x59, 0x0a, 0x12, 0x47, 0x65, 0x74, 0x56, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x15, 0x0a, 0x0d, 0x6d,
    0x61, 0x6a, 0x6f, 0x72, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x05, 0x12, 0x15, 0x0a, 0x0d, 0x6d, 0x69, 0x6e, 0x6f, 0x72, 0x5f, 0x76, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x02, 0x28, 0x05, 0x12, 0x15, 0x0a, 0x0d, 0x70, 0x61, 0x74,
    0x63, 0x68, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x02, 0x28, 0x05,
    0x22, 0x55, 0x0a, 0x06, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x03, 0x12, 0x19, 0x0a, 0x0d, 0x6d, 0x61, 0x74, 0x72, 0x69, 0x78,
    0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x01, 0x42, 0x02, 0x10,
    0x01, 0x12, 0x24, 0x0a, 0x18, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f,
    0x6d, 0x61, 0x74, 0x72, 0x69, 0x78, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x18, 0x03, 0x20,
    0x03, 0x28, 0x01, 0x42, 0x02, 0x10, 0x01, 0x22, 0x96, 0x02, 0x0a, 0x15, 0x43, 0x72, 0x65, 0x61,
    0x74, 0x65, 0x52, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x65, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x33, 0x0a, 0x06, 0x72, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0e, 0x32, 0x23, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x52, 0x65, 0x6e, 0x64, 0x65, 0x72,
    0x65, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x52, 0x65, 0x6e, 0x64, 0x65, 0x72,
    0x65, 0x72, 0x54, 0x79, 0x70, 0x65, 0x12, 0x36, 0x0a, 0x07, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65,
    0x52, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x65, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e,
    0x52, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x65, 0x72, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x18,
    0x0a, 0x07, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x07, 0x2e, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x12, 0x20, 0x0a, 0x14, 0x76, 0x69, 0x65, 0x77,
    0x65, 0x72, 0x5f, 0x6d, 0x61, 0x74, 0x72, 0x69, 0x78, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73,
    0x18, 0x04, 0x20, 0x03, 0x28, 0x01, 0x42, 0x02, 0x10, 0x01, 0x1a, 0x2c, 0x0a, 0x0e, 0x52, 0x65,
    0x6e, 0x64, 0x65, 0x72, 0x65, 0x72, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x0b, 0x0a, 0x03,
    0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x22, 0x26, 0x0a, 0x0c, 0x52, 0x65, 0x6e, 0x64,
    0x65, 0x72, 0x65, 0x72, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0b, 0x0a, 0x07, 0x44, 0x49, 0x53, 0x50,
    0x4c, 0x41, 0x59, 0x10, 0x01, 0x12, 0x09, 0x0a, 0x05, 0x57, 0x45, 0x42, 0x47, 0x4c, 0x10, 0x02,
    0x22, 0x2c, 0x0a, 0x15, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x65, 0x6e, 0x64, 0x65, 0x72,
    0x65, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x13, 0x0a, 0x0b, 0x72, 0x65, 0x6e,
    0x64, 0x65, 0x72, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x03, 0x22, 0x43,
    0x0a, 0x13, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x13, 0x0a, 0x0b, 0x72, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x65,
    0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x03, 0x12, 0x17, 0x0a, 0x06, 0x63, 0x61,
    0x6d, 0x65, 0x72, 0x61, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x43, 0x61, 0x6d,
    0x65, 0x72, 0x61, 0x22, 0x3d, 0x0a, 0x13, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x43, 0x61, 0x6d,
    0x65, 0x72, 0x61, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x13, 0x0a, 0x0b, 0x72, 0x65,
    0x6e, 0x64, 0x65, 0x72, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x03, 0x12,
    0x11, 0x0a, 0x09, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x03,
    0x28, 0x03, 0x22, 0x4b, 0x0a, 0x19, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x56, 0x69, 0x65, 0x77,
    0x65, 0x72, 0x4d, 0x61, 0x74, 0x72, 0x69, 0x78, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x13, 0x0a, 0x0b, 0x72, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x03, 0x12, 0x19, 0x0a, 0x0d, 0x6d, 0x61, 0x74, 0x72, 0x69, 0x78, 0x5f, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x01, 0x42, 0x02, 0x10, 0x01, 0x22,
    0x5e, 0x0a, 0x19, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x4d,
    0x61, 0x74, 0x72, 0x69, 0x78, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x13, 0x0a, 0x0b,
    0x72, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x03, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x5f, 0x69, 0x64, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x03, 0x12, 0x19, 0x0a, 0x0d, 0x6d, 0x61, 0x74, 0x72, 0x69, 0x78, 0x5f, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x01, 0x42, 0x02, 0x10, 0x01, 0x22,
    0x62, 0x0a, 0x1d, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x50,
    0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x12, 0x13, 0x0a, 0x0b, 0x72, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x03, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x5f,
    0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x03, 0x12, 0x19, 0x0a, 0x0d, 0x6d, 0x61, 0x74, 0x72,
    0x69, 0x78, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x01, 0x42,
    0x02, 0x10, 0x01, 0x22, 0x64, 0x0a, 0x07, 0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x12, 0x24,
    0x0a, 0x06, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x14,
    0x2e, 0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x2e, 0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x63, 0x68, 0x69, 0x6c, 0x64, 0x72, 0x65, 0x6e,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74,
    0x22, 0x17, 0x0a, 0x0b, 0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12,
    0x08, 0x0a, 0x04, 0x4d, 0x45, 0x53, 0x48, 0x10, 0x01, 0x22, 0x30, 0x0a, 0x12, 0x43, 0x72, 0x65,
    0x61, 0x74, 0x65, 0x47, 0x72, 0x61, 0x70, 0x68, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x1a, 0x0a, 0x08, 0x63, 0x68, 0x69, 0x6c, 0x64, 0x72, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x08, 0x2e, 0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x22, 0x78, 0x0a, 0x12, 0x41,
    0x64, 0x64, 0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x10, 0x0a, 0x08, 0x67, 0x72, 0x61, 0x70, 0x68, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x03, 0x12, 0x11, 0x0a, 0x09, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x03, 0x12, 0x1b, 0x0a, 0x13, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f,
    0x75, 0x73, 0x5f, 0x73, 0x69, 0x62, 0x6c, 0x69, 0x6e, 0x67, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x03, 0x12, 0x20, 0x0a, 0x0e, 0x61, 0x64, 0x64, 0x65, 0x64, 0x5f, 0x65, 0x6c, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x45, 0x6c,
    0x65, 0x6d, 0x65, 0x6e, 0x74, 0x22, 0x29, 0x0a, 0x15, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x45,
    0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x10,
    0x0a, 0x08, 0x67, 0x72, 0x61, 0x70, 0x68, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x03,
    0x22, 0x46, 0x0a, 0x15, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e,
    0x74, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x08, 0x67, 0x72, 0x61,
    0x70, 0x68, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x03, 0x12, 0x1b, 0x0a, 0x13, 0x64,
    0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x5f, 0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x69,
    0x64, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x03, 0x22, 0xe2, 0x07, 0x0a, 0x07, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x24, 0x0a, 0x06, 0x72, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0e, 0x32, 0x14, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x37, 0x0a, 0x17, 0x63, 0x72,
    0x65, 0x61, 0x74, 0x65, 0x5f, 0x72, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x65, 0x72, 0x5f, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x43, 0x72,
    0x65, 0x61, 0x74, 0x65, 0x52, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x65, 0x72, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x37, 0x0a, 0x17, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x5f, 0x72, 0x65,
    0x6e, 0x64, 0x65, 0x72, 0x65, 0x72, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x65, 0x6e,
    0x64, 0x65, 0x72, 0x65, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x40, 0x0a, 0x1c,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x76, 0x69, 0x65, 0x77, 0x65, 0x72, 0x5f, 0x6d, 0x61,
    0x74, 0x72, 0x69, 0x78, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x56, 0x69, 0x65, 0x77, 0x65,
    0x72, 0x4d, 0x61, 0x74, 0x72, 0x69, 0x78, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x33,
    0x0a, 0x15, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x5f, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x5f,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e,
    0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x33, 0x0a, 0x15, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x5f, 0x63, 0x61,
    0x6d, 0x65, 0x72, 0x61, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x14, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x43, 0x61, 0x6d, 0x65, 0x72,
    0x61, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x40, 0x0a, 0x1c, 0x75, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x5f, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x5f, 0x6d, 0x61, 0x74, 0x72, 0x69, 0x78,
    0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a,
    0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x4d, 0x61, 0x74,
    0x72, 0x69, 0x78, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x48, 0x0a, 0x20, 0x75, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x5f, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x5f, 0x70, 0x72, 0x6f, 0x6a,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x08,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x43, 0x61, 0x6d,
    0x65, 0x72, 0x61, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x31, 0x0a, 0x14, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x5f, 0x67,
    0x72, 0x61, 0x70, 0x68, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x09, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x13, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x47, 0x72, 0x61, 0x70, 0x68,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x31, 0x0a, 0x14, 0x61, 0x64, 0x64, 0x5f, 0x65,
    0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x41, 0x64, 0x64, 0x45, 0x6c, 0x65, 0x6d, 0x65,
    0x6e, 0x74, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x37, 0x0a, 0x17, 0x75, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x5f, 0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x5f, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x55, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x37, 0x0a, 0x17, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x5f, 0x65, 0x6c,
    0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x0c,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x45, 0x6c, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0xae, 0x02, 0x0a,
    0x0b, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0f, 0x0a, 0x0b,
    0x47, 0x45, 0x54, 0x5f, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4f, 0x4e, 0x10, 0x01, 0x12, 0x13, 0x0a,
    0x0f, 0x43, 0x52, 0x45, 0x41, 0x54, 0x45, 0x5f, 0x52, 0x45, 0x4e, 0x44, 0x45, 0x52, 0x45, 0x52,
    0x10, 0x02, 0x12, 0x13, 0x0a, 0x0f, 0x44, 0x45, 0x4c, 0x45, 0x54, 0x45, 0x5f, 0x52, 0x45, 0x4e,
    0x44, 0x45, 0x52, 0x45, 0x52, 0x10, 0x03, 0x12, 0x18, 0x0a, 0x14, 0x55, 0x50, 0x44, 0x41, 0x54,
    0x45, 0x5f, 0x56, 0x49, 0x45, 0x57, 0x45, 0x52, 0x5f, 0x4d, 0x41, 0x54, 0x52, 0x49, 0x58, 0x10,
    0x04, 0x12, 0x11, 0x0a, 0x0d, 0x43, 0x52, 0x45, 0x41, 0x54, 0x45, 0x5f, 0x43, 0x41, 0x4d, 0x45,
    0x52, 0x41, 0x10, 0x05, 0x12, 0x11, 0x0a, 0x0d, 0x44, 0x45, 0x4c, 0x45, 0x54, 0x45, 0x5f, 0x43,
    0x41, 0x4d, 0x45, 0x52, 0x41, 0x10, 0x06, 0x12, 0x18, 0x0a, 0x14, 0x55, 0x50, 0x44, 0x41, 0x54,
    0x45, 0x5f, 0x43, 0x41, 0x4d, 0x45, 0x52, 0x41, 0x5f, 0x4d, 0x41, 0x54, 0x52, 0x49, 0x58, 0x10,
    0x07, 0x12, 0x1c, 0x0a, 0x18, 0x55, 0x44, 0x50, 0x41, 0x54, 0x45, 0x5f, 0x43, 0x41, 0x4d, 0x45,
    0x52, 0x41, 0x5f, 0x50, 0x52, 0x4f, 0x4a, 0x45, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x10, 0x08, 0x12,
    0x18, 0x0a, 0x14, 0x43, 0x52, 0x45, 0x41, 0x54, 0x45, 0x5f, 0x47, 0x52, 0x41, 0x50, 0x48, 0x5f,
    0x52, 0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x10, 0x09, 0x12, 0x18, 0x0a, 0x14, 0x41, 0x44, 0x44,
    0x5f, 0x45, 0x4c, 0x45, 0x4d, 0x45, 0x4e, 0x54, 0x53, 0x5f, 0x52, 0x45, 0x51, 0x55, 0x45, 0x53,
    0x54, 0x10, 0x0a, 0x12, 0x1b, 0x0a, 0x17, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x5f, 0x45, 0x4c,
    0x45, 0x4d, 0x45, 0x4e, 0x54, 0x53, 0x5f, 0x52, 0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x10, 0x0b,
    0x12, 0x1b, 0x0a, 0x17, 0x44, 0x45, 0x4c, 0x45, 0x54, 0x45, 0x5f, 0x45, 0x4c, 0x45, 0x4d, 0x45,
    0x4e, 0x54, 0x53, 0x5f, 0x52, 0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x10, 0x0c, 0x22, 0x26, 0x0a,
    0x08, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x72, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0xdc, 0x01, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x26, 0x0a, 0x06, 0x72, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x0e, 0x32, 0x16, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x69, 0x6e,
    0x64, 0x65, 0x78, 0x18, 0x02, 0x20, 0x02, 0x28, 0x03, 0x12, 0x31, 0x0a, 0x14, 0x67, 0x65, 0x74,
    0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x47, 0x65, 0x74, 0x56, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x33, 0x0a, 0x15,
    0x69, 0x74, 0x65, 0x6d, 0x5f, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x49, 0x74,
    0x65, 0x6d, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x22, 0x31, 0x0a, 0x0c, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x54, 0x79, 0x70,
    0x65, 0x12, 0x0f, 0x0a, 0x0b, 0x47, 0x45, 0x54, 0x5f, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4f, 0x4e,
    0x10, 0x01, 0x12, 0x10, 0x0a, 0x0c, 0x49, 0x54, 0x45, 0x4d, 0x5f, 0x43, 0x52, 0x45, 0x41, 0x54,
    0x45, 0x44, 0x10, 0x02, 0x22, 0x29, 0x0a, 0x09, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x73, 0x12, 0x1c, 0x0a, 0x09, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x4a,
    0x93, 0x2e, 0x0a, 0x07, 0x12, 0x05, 0x02, 0x00, 0x9f, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x02, 0x00, 0x04, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x02, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x02, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x03, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x03, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x03, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x08, 0x00, 0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x08, 0x08, 0x1a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x09, 0x11, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x09, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0a,
    0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x11, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x02, 0x12, 0x03, 0x0b, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x0b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x0b, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x11,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x21, 0x22, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x10, 0x00, 0x14, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x10, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12,
    0x03, 0x11, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x11,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x11, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x11, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x11, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x12, 0x02, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x12, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x12, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x12, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x12, 0x22,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x08, 0x12, 0x03, 0x12, 0x24, 0x31, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x12, 0x25, 0x30,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x12,
    0x25, 0x2b, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x02, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x12, 0x25, 0x2b, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x02, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x25, 0x2b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02,
    0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x12, 0x2c, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x13, 0x02, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x13, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x13, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x13,
    0x12, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x13, 0x2d, 0x2e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x08, 0x12, 0x03, 0x13, 0x2f, 0x3c, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x02, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x13, 0x30, 0x3b, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x13, 0x30,
    0x36, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x02, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x13, 0x30, 0x36, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x02, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x30, 0x36, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x02,
    0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x13, 0x37, 0x3b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x16, 0x00, 0x25, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x16,
    0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x04, 0x00, 0x12, 0x04, 0x17, 0x02, 0x1a, 0x03,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x17, 0x07, 0x13, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x18, 0x04, 0x10, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x04, 0x0b, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x18, 0x0e, 0x0f, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x19, 0x04, 0x0e, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x19, 0x04, 0x09, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x19, 0x0c, 0x0d, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x03, 0x03, 0x00, 0x12, 0x04, 0x1c, 0x02, 0x1f, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x03, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x0a, 0x18, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x1d, 0x04, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x1d, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x1d, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x1d, 0x14, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x1d, 0x1a, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x1e, 0x04, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x1e, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x1e, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x1e, 0x14, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x1e, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03,
    0x21, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x21, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x21, 0x0b, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x18, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x01, 0x12, 0x03, 0x22, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x22, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x22, 0x0b, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x22,
    0x1a, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x22, 0x24, 0x25,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x23, 0x02, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x23, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x02, 0x06, 0x12, 0x03, 0x23, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x23, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x23, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x24,
    0x02, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x24, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x24, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x24, 0x12, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x24, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x03, 0x08, 0x12, 0x03, 0x24, 0x2b, 0x38, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x03, 0x02, 0x03,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x24, 0x2c, 0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x03, 0x02,
    0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x24, 0x2c, 0x32, 0x0a, 0x11, 0x0a, 0x0a, 0x04,
    0x03, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x24, 0x2c, 0x32, 0x0a, 0x12,
    0x0a, 0x0b, 0x04, 0x03, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24,
    0x2c, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x03, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12,
    0x03, 0x24, 0x33, 0x37, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x27, 0x00, 0x29, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x27, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x28, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x28, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x28, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x28, 0x11, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x28, 0x1f,
    0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x2b, 0x00, 0x2e, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x2b, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x00, 0x12, 0x03, 0x2c, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2c,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2c, 0x11, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x1f, 0x20, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x2d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x2d, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x2d, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x30, 0x00, 0x33, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x30, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x00, 0x12, 0x03, 0x31, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x31, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x31, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x31,
    0x11, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x31, 0x1f, 0x20,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x32, 0x02, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x32, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x32, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x32, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x32, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x35, 0x00, 0x38,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x35, 0x08, 0x21, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x36, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x36, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x36, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x36, 0x11, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x36,
    0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x37, 0x02, 0x32, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x37, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x37, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x37, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x37, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x08,
    0x12, 0x03, 0x37, 0x24, 0x31, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x07, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x37, 0x25, 0x30, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x07, 0x02, 0x01, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x12, 0x03, 0x37, 0x25, 0x2b, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x07, 0x02, 0x01,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x37, 0x25, 0x2b, 0x0a, 0x12, 0x0a, 0x0b, 0x04,
    0x07, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x37, 0x25, 0x2b, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x07, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x37, 0x2c,
    0x30, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x3a, 0x00, 0x3e, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02,
    0x00, 0x12, 0x03, 0x3b, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x3b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3b,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3b, 0x11, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3b, 0x1f, 0x20, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x3c, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x3c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x3c, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x3c, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x3d, 0x02, 0x32,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x03, 0x3d, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3d, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x3d, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02,
    0x08, 0x12, 0x03, 0x3d, 0x24, 0x31, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x08, 0x02, 0x02, 0x08, 0xe7,
    0x07, 0x00, 0x12, 0x03, 0x3d, 0x25, 0x30, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x08, 0x02, 0x02, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x3d, 0x25, 0x2b, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x08, 0x02,
    0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x3d, 0x25, 0x2b, 0x0a, 0x12, 0x0a, 0x0b,
    0x04, 0x08, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3d, 0x25, 0x2b,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x08, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x3d,
    0x2c, 0x30, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x40, 0x00, 0x44, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x40, 0x08, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09,
    0x02, 0x00, 0x12, 0x03, 0x41, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x41, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x41, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x41, 0x11,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x41, 0x1f, 0x20, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x42, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x03, 0x42, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x42, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x42, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x42, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x03, 0x43, 0x02,
    0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12, 0x03, 0x43, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x05, 0x12, 0x03, 0x43, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x03, 0x43, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x02, 0x03, 0x12, 0x03, 0x43, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x02, 0x08, 0x12, 0x03, 0x43, 0x24, 0x31, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x09, 0x02, 0x02, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x43, 0x25, 0x30, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x09, 0x02, 0x02,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x43, 0x25, 0x2b, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x09,
    0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x43, 0x25, 0x2b, 0x0a, 0x12, 0x0a,
    0x0b, 0x04, 0x09, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x43, 0x25,
    0x2b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x09, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03,
    0x43, 0x2c, 0x30, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x48, 0x00, 0x4f, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x48, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0a, 0x04, 0x00, 0x12, 0x04, 0x49, 0x02, 0x4b, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x49, 0x07, 0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0a, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x4a, 0x04, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x4a, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x4a, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03,
    0x4d, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4d, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x03, 0x4d, 0x0b, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4d, 0x17, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4d, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0a, 0x02, 0x01, 0x12, 0x03, 0x4e, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x4e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x4e, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4e,
    0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4e, 0x1e, 0x1f,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x51, 0x00, 0x53, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x0b, 0x01, 0x12, 0x03, 0x51, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00,
    0x12, 0x03, 0x52, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x52, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x06, 0x12, 0x03, 0x52, 0x0b,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x52, 0x13, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x52, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x0c, 0x12, 0x04, 0x55, 0x00, 0x5a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01,
    0x12, 0x03, 0x55, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x56,
    0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x56, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x03, 0x56, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x56, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x56, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c,
    0x02, 0x01, 0x12, 0x03, 0x57, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x57, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x57, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x03, 0x57, 0x11,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x03, 0x57, 0x1d, 0x1e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12, 0x03, 0x58, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x03, 0x58, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x58, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x58, 0x11, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x58, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x03, 0x12, 0x03, 0x59, 0x02,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x04, 0x12, 0x03, 0x59, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x06, 0x12, 0x03, 0x59, 0x0b, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x03, 0x01, 0x12, 0x03, 0x59, 0x13, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x03, 0x03, 0x12, 0x03, 0x59, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0d, 0x12,
    0x04, 0x5c, 0x00, 0x5f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x03, 0x5c, 0x08,
    0x1d, 0x0a, 0x30, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x5d, 0x02, 0x1e, 0x22, 0x23,
    0x20, 0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20,
    0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x20, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74,
    0x65, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5d, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5d, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5d, 0x11, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5d, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x0e, 0x12, 0x04, 0x61, 0x00, 0x64, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x03,
    0x61, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x03, 0x62, 0x02, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x03, 0x62, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x03, 0x62, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x03, 0x62, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x62, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01,
    0x12, 0x03, 0x63, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x63, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x05, 0x12, 0x03, 0x63, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12, 0x03, 0x63, 0x11, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x03, 0x63, 0x27, 0x28, 0x0a, 0x0b, 0x0a,
    0x02, 0x04, 0x0f, 0x12, 0x05, 0x68, 0x00, 0x88, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0f,
    0x01, 0x12, 0x03, 0x68, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x04, 0x00, 0x12, 0x04,
    0x69, 0x02, 0x78, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x04, 0x00, 0x01, 0x12, 0x03, 0x69,
    0x07, 0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x6a, 0x04,
    0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6a, 0x04,
    0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x6a, 0x12,
    0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x6c, 0x04, 0x18,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6c, 0x04, 0x13,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x6c, 0x16, 0x17,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x6d, 0x04, 0x18, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6d, 0x04, 0x13, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x6d, 0x16, 0x17, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x6e, 0x04, 0x1d, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x6e, 0x04, 0x18, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x6e, 0x1b, 0x1c, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x6f, 0x04, 0x16, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x6f, 0x04, 0x11, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x6f, 0x14, 0x15, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x70, 0x04, 0x16, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x0f, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x70, 0x04, 0x11, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x0f, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x70, 0x14, 0x15, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x0f, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x71, 0x04, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x0f, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x71, 0x04, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x0f, 0x04, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x71, 0x1b, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x0f, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x72, 0x04, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f,
    0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x72, 0x04, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f,
    0x04, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x72, 0x1f, 0x20, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0f,
    0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x74, 0x04, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x04,
    0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x74, 0x04, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x04,
    0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x74, 0x1b, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0f, 0x04,
    0x00, 0x02, 0x09, 0x12, 0x03, 0x75, 0x04, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00,
    0x02, 0x09, 0x01, 0x12, 0x03, 0x75, 0x04, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00,
    0x02, 0x09, 0x02, 0x12, 0x03, 0x75, 0x1b, 0x1d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00,
    0x02, 0x0a, 0x12, 0x03, 0x76, 0x04, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02,
    0x0a, 0x01, 0x12, 0x03, 0x76, 0x04, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02,
    0x0a, 0x02, 0x12, 0x03, 0x76, 0x1e, 0x20, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0f, 0x04, 0x00, 0x02,
    0x0b, 0x12, 0x03, 0x77, 0x04, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x0b,
    0x01, 0x12, 0x03, 0x77, 0x04, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x04, 0x00, 0x02, 0x0b,
    0x02, 0x12, 0x03, 0x77, 0x1e, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x03,
    0x7a, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7a, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x06, 0x12, 0x03, 0x7a, 0x0b, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7a, 0x17, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7a, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0f, 0x02, 0x01, 0x12, 0x03, 0x7c, 0x02, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x7c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x7c, 0x0b, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7c,
    0x21, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x03, 0x7c, 0x3b, 0x3c,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x02, 0x12, 0x03, 0x7d, 0x02, 0x3d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x02, 0x04, 0x12, 0x03, 0x7d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x02, 0x06, 0x12, 0x03, 0x7d, 0x0b, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x7d, 0x21, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x7d, 0x3b, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x03, 0x12, 0x03, 0x7e,
    0x02, 0x46, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x04, 0x12, 0x03, 0x7e, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x06, 0x12, 0x03, 0x7e, 0x0b, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x01, 0x12, 0x03, 0x7e, 0x25, 0x41, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x03, 0x03, 0x12, 0x03, 0x7e, 0x44, 0x45, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f,
    0x02, 0x04, 0x12, 0x03, 0x7f, 0x02, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x7f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x06, 0x12, 0x03,
    0x7f, 0x0b, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x01, 0x12, 0x03, 0x7f, 0x1f,
    0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x03, 0x12, 0x03, 0x7f, 0x37, 0x38, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x05, 0x12, 0x04, 0x80, 0x01, 0x02, 0x39, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x05, 0x04, 0x12, 0x04, 0x80, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x05, 0x06, 0x12, 0x04, 0x80, 0x01, 0x0b, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x05, 0x01, 0x12, 0x04, 0x80, 0x01, 0x1f, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x05, 0x03, 0x12, 0x04, 0x80, 0x01, 0x37, 0x38, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02,
    0x06, 0x12, 0x04, 0x81, 0x01, 0x02, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x04,
    0x12, 0x04, 0x81, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x06, 0x12,
    0x04, 0x81, 0x01, 0x0b, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x01, 0x12, 0x04,
    0x81, 0x01, 0x25, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x03, 0x12, 0x04, 0x81,
    0x01, 0x44, 0x45, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x07, 0x12, 0x04, 0x82, 0x01, 0x02,
    0x4e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x07, 0x04, 0x12, 0x04, 0x82, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x07, 0x06, 0x12, 0x04, 0x82, 0x01, 0x0b, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x07, 0x01, 0x12, 0x04, 0x82, 0x01, 0x29, 0x49, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x07, 0x03, 0x12, 0x04, 0x82, 0x01, 0x4c, 0x4d, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0f, 0x02, 0x08, 0x12, 0x04, 0x84, 0x01, 0x02, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x08, 0x04, 0x12, 0x04, 0x84, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x08, 0x06, 0x12, 0x04, 0x84, 0x01, 0x0b, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x08, 0x01, 0x12, 0x04, 0x84, 0x01, 0x1e, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08,
    0x03, 0x12, 0x04, 0x84, 0x01, 0x35, 0x36, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x09, 0x12,
    0x04, 0x85, 0x01, 0x02, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x04, 0x12, 0x04,
    0x85, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x06, 0x12, 0x04, 0x85,
    0x01, 0x0b, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x01, 0x12, 0x04, 0x85, 0x01,
    0x1e, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x03, 0x12, 0x04, 0x85, 0x01, 0x35,
    0x37, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x0a, 0x12, 0x04, 0x86, 0x01, 0x02, 0x3e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0a, 0x04, 0x12, 0x04, 0x86, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x86, 0x01, 0x0b, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x0a, 0x01, 0x12, 0x04, 0x86, 0x01, 0x21, 0x38, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x0a, 0x03, 0x12, 0x04, 0x86, 0x01, 0x3b, 0x3d, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0f, 0x02, 0x0b, 0x12, 0x04, 0x87, 0x01, 0x02, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x0b, 0x04, 0x12, 0x04, 0x87, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0b,
    0x06, 0x12, 0x04, 0x87, 0x01, 0x0b, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0b, 0x01,
    0x12, 0x04, 0x87, 0x01, 0x21, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0b, 0x03, 0x12,
    0x04, 0x87, 0x01, 0x3b, 0x3d, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0x8a, 0x01, 0x00,
    0x8c, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0x8b, 0x01, 0x02, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8b, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x00, 0x06, 0x12, 0x04, 0x8b, 0x01, 0x0b, 0x12, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x13, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8b, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x11,
    0x12, 0x06, 0x90, 0x01, 0x00, 0x9b, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12,
    0x04, 0x90, 0x01, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x11, 0x04, 0x00, 0x12, 0x06, 0x91,
    0x01, 0x02, 0x94, 0x01, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x04, 0x00, 0x01, 0x12, 0x04,
    0x91, 0x01, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x11, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04,
    0x92, 0x01, 0x04, 0x14, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x92, 0x01, 0x04, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x04, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x04, 0x92, 0x01, 0x12, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x11, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x04, 0x93, 0x01, 0x04, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x93, 0x01, 0x04, 0x10, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x04, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x04, 0x93, 0x01, 0x13, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00,
    0x12, 0x04, 0x96, 0x01, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x96, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x06, 0x12, 0x04,
    0x96, 0x01, 0x0b, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0x96,
    0x01, 0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0x96, 0x01,
    0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0x97, 0x01, 0x02, 0x1b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x04, 0x97, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x05, 0x12, 0x04, 0x97, 0x01, 0x0b, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04, 0x97, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0x97, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x11, 0x02, 0x02, 0x12, 0x04, 0x99, 0x01, 0x02, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x99, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x02, 0x06, 0x12, 0x04, 0x99, 0x01, 0x0b, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02,
    0x01, 0x12, 0x04, 0x99, 0x01, 0x1e, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x03,
    0x12, 0x04, 0x99, 0x01, 0x35, 0x36, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x03, 0x12, 0x04,
    0x9a, 0x01, 0x02, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x04, 0x12, 0x04, 0x9a,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x06, 0x12, 0x04, 0x9a, 0x01,
    0x0b, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x1f,
    0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x03, 0x12, 0x04, 0x9a, 0x01, 0x37, 0x38,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0x9d, 0x01, 0x00, 0x9f, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x12, 0x02, 0x00, 0x12, 0x04, 0x9e, 0x01, 0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x9e, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00,
    0x06, 0x12, 0x04, 0x9e, 0x01, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x9e, 0x01, 0x14, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x9e, 0x01, 0x20, 0x21,
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
