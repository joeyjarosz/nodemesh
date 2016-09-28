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
pub struct GetVersionResponse {
    // message fields
    version: ::protobuf::SingularField<::std::string::String>,
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
                    version: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string version = 1;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        if self.version.is_none() {
            self.version.set_default();
        };
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        self.version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_version(&self) -> &str {
        match self.version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for GetVersionResponse {
    fn is_initialized(&self) -> bool {
        if self.version.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.version));
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
        for value in &self.version {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version.as_ref() {
            try!(os.write_string(1, &v));
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
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "version",
                    GetVersionResponse::has_version,
                    GetVersionResponse::get_version,
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
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetVersionResponse {
    fn eq(&self, other: &GetVersionResponse) -> bool {
        self.version == other.version &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetVersionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Node {
    // message fields
    node_type: ::std::option::Option<Node_NodeType>,
    children: ::protobuf::RepeatedField<Node>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Node {}

impl Node {
    pub fn new() -> Node {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Node {
        static mut instance: ::protobuf::lazy::Lazy<Node> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Node,
        };
        unsafe {
            instance.get(|| {
                Node {
                    node_type: ::std::option::Option::None,
                    children: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .Node.NodeType node_type = 1;

    pub fn clear_node_type(&mut self) {
        self.node_type = ::std::option::Option::None;
    }

    pub fn has_node_type(&self) -> bool {
        self.node_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node_type(&mut self, v: Node_NodeType) {
        self.node_type = ::std::option::Option::Some(v);
    }

    pub fn get_node_type(&self) -> Node_NodeType {
        self.node_type.unwrap_or(Node_NodeType::EMPTY)
    }

    // repeated .Node children = 2;

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    // Param is passed by value, moved
    pub fn set_children(&mut self, v: ::protobuf::RepeatedField<Node>) {
        self.children = v;
    }

    // Mutable pointer to the field.
    pub fn mut_children(&mut self) -> &mut ::protobuf::RepeatedField<Node> {
        &mut self.children
    }

    // Take field
    pub fn take_children(&mut self) -> ::protobuf::RepeatedField<Node> {
        ::std::mem::replace(&mut self.children, ::protobuf::RepeatedField::new())
    }

    pub fn get_children(&self) -> &[Node] {
        &self.children
    }
}

impl ::protobuf::Message for Node {
    fn is_initialized(&self) -> bool {
        if self.node_type.is_none() {
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
                    self.node_type = ::std::option::Option::Some(tmp);
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
        for value in &self.node_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.children {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.node_type {
            try!(os.write_enum(1, v.value()));
        };
        for v in &self.children {
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
        ::std::any::TypeId::of::<Node>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Node {
    fn new() -> Node {
        Node::new()
    }

    fn descriptor_static(_: ::std::option::Option<Node>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "node_type",
                    Node::has_node_type,
                    Node::get_node_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "children",
                    Node::get_children,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Node>(
                    "Node",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Node {
    fn clear(&mut self) {
        self.clear_node_type();
        self.clear_children();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.node_type == other.node_type &&
        self.children == other.children &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Node {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Node_NodeType {
    EMPTY = 1,
    MESH = 2,
}

impl ::protobuf::ProtobufEnum for Node_NodeType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Node_NodeType> {
        match value {
            1 => ::std::option::Option::Some(Node_NodeType::EMPTY),
            2 => ::std::option::Option::Some(Node_NodeType::MESH),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Node_NodeType] = &[
            Node_NodeType::EMPTY,
            Node_NodeType::MESH,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Node_NodeType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Node_NodeType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Node_NodeType {
}

#[derive(Clone,Default)]
pub struct AddGraphRequest {
    // message fields
    nodes: ::protobuf::RepeatedField<Node>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AddGraphRequest {}

impl AddGraphRequest {
    pub fn new() -> AddGraphRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AddGraphRequest {
        static mut instance: ::protobuf::lazy::Lazy<AddGraphRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AddGraphRequest,
        };
        unsafe {
            instance.get(|| {
                AddGraphRequest {
                    nodes: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .Node nodes = 1;

    pub fn clear_nodes(&mut self) {
        self.nodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_nodes(&mut self, v: ::protobuf::RepeatedField<Node>) {
        self.nodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nodes(&mut self) -> &mut ::protobuf::RepeatedField<Node> {
        &mut self.nodes
    }

    // Take field
    pub fn take_nodes(&mut self) -> ::protobuf::RepeatedField<Node> {
        ::std::mem::replace(&mut self.nodes, ::protobuf::RepeatedField::new())
    }

    pub fn get_nodes(&self) -> &[Node] {
        &self.nodes
    }
}

impl ::protobuf::Message for AddGraphRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.nodes));
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
        for value in &self.nodes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.nodes {
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
        ::std::any::TypeId::of::<AddGraphRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AddGraphRequest {
    fn new() -> AddGraphRequest {
        AddGraphRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AddGraphRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "nodes",
                    AddGraphRequest::get_nodes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AddGraphRequest>(
                    "AddGraphRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AddGraphRequest {
    fn clear(&mut self) {
        self.clear_nodes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AddGraphRequest {
    fn eq(&self, other: &AddGraphRequest) -> bool {
        self.nodes == other.nodes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AddGraphRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UseGraphRequest {
    // message fields
    graph_id: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UseGraphRequest {}

impl UseGraphRequest {
    pub fn new() -> UseGraphRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UseGraphRequest {
        static mut instance: ::protobuf::lazy::Lazy<UseGraphRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UseGraphRequest,
        };
        unsafe {
            instance.get(|| {
                UseGraphRequest {
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

impl ::protobuf::Message for UseGraphRequest {
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
        for value in &self.graph_id {
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
        ::std::any::TypeId::of::<UseGraphRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UseGraphRequest {
    fn new() -> UseGraphRequest {
        UseGraphRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<UseGraphRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "graph_id",
                    UseGraphRequest::has_graph_id,
                    UseGraphRequest::get_graph_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UseGraphRequest>(
                    "UseGraphRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UseGraphRequest {
    fn clear(&mut self) {
        self.clear_graph_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UseGraphRequest {
    fn eq(&self, other: &UseGraphRequest) -> bool {
        self.graph_id == other.graph_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UseGraphRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RemoveGraphRequest {
    // message fields
    graph_id: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemoveGraphRequest {}

impl RemoveGraphRequest {
    pub fn new() -> RemoveGraphRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveGraphRequest {
        static mut instance: ::protobuf::lazy::Lazy<RemoveGraphRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveGraphRequest,
        };
        unsafe {
            instance.get(|| {
                RemoveGraphRequest {
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

impl ::protobuf::Message for RemoveGraphRequest {
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
        for value in &self.graph_id {
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
        ::std::any::TypeId::of::<RemoveGraphRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RemoveGraphRequest {
    fn new() -> RemoveGraphRequest {
        RemoveGraphRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveGraphRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "graph_id",
                    RemoveGraphRequest::has_graph_id,
                    RemoveGraphRequest::get_graph_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RemoveGraphRequest>(
                    "RemoveGraphRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveGraphRequest {
    fn clear(&mut self) {
        self.clear_graph_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RemoveGraphRequest {
    fn eq(&self, other: &RemoveGraphRequest) -> bool {
        self.graph_id == other.graph_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RemoveGraphRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AddNodesRequest {
    // message fields
    graph_id: ::std::option::Option<i64>,
    parent_id: ::std::option::Option<i64>,
    previous_sibling_id: ::std::option::Option<i64>,
    nodes: ::protobuf::RepeatedField<Node>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AddNodesRequest {}

impl AddNodesRequest {
    pub fn new() -> AddNodesRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AddNodesRequest {
        static mut instance: ::protobuf::lazy::Lazy<AddNodesRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AddNodesRequest,
        };
        unsafe {
            instance.get(|| {
                AddNodesRequest {
                    graph_id: ::std::option::Option::None,
                    parent_id: ::std::option::Option::None,
                    previous_sibling_id: ::std::option::Option::None,
                    nodes: ::protobuf::RepeatedField::new(),
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

    // optional int64 parent_id = 2;

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

    // repeated .Node nodes = 4;

    pub fn clear_nodes(&mut self) {
        self.nodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_nodes(&mut self, v: ::protobuf::RepeatedField<Node>) {
        self.nodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nodes(&mut self) -> &mut ::protobuf::RepeatedField<Node> {
        &mut self.nodes
    }

    // Take field
    pub fn take_nodes(&mut self) -> ::protobuf::RepeatedField<Node> {
        ::std::mem::replace(&mut self.nodes, ::protobuf::RepeatedField::new())
    }

    pub fn get_nodes(&self) -> &[Node] {
        &self.nodes
    }
}

impl ::protobuf::Message for AddNodesRequest {
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
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.nodes));
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
        for value in &self.graph_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.parent_id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.previous_sibling_id {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.nodes {
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
        for v in &self.nodes {
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
        ::std::any::TypeId::of::<AddNodesRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AddNodesRequest {
    fn new() -> AddNodesRequest {
        AddNodesRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AddNodesRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "graph_id",
                    AddNodesRequest::has_graph_id,
                    AddNodesRequest::get_graph_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "parent_id",
                    AddNodesRequest::has_parent_id,
                    AddNodesRequest::get_parent_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "previous_sibling_id",
                    AddNodesRequest::has_previous_sibling_id,
                    AddNodesRequest::get_previous_sibling_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "nodes",
                    AddNodesRequest::get_nodes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AddNodesRequest>(
                    "AddNodesRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AddNodesRequest {
    fn clear(&mut self) {
        self.clear_graph_id();
        self.clear_parent_id();
        self.clear_previous_sibling_id();
        self.clear_nodes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AddNodesRequest {
    fn eq(&self, other: &AddNodesRequest) -> bool {
        self.graph_id == other.graph_id &&
        self.parent_id == other.parent_id &&
        self.previous_sibling_id == other.previous_sibling_id &&
        self.nodes == other.nodes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AddNodesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RemoveNodesRequest {
    // message fields
    graph_id: ::std::option::Option<i64>,
    node_ids: ::std::vec::Vec<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemoveNodesRequest {}

impl RemoveNodesRequest {
    pub fn new() -> RemoveNodesRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveNodesRequest {
        static mut instance: ::protobuf::lazy::Lazy<RemoveNodesRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveNodesRequest,
        };
        unsafe {
            instance.get(|| {
                RemoveNodesRequest {
                    graph_id: ::std::option::Option::None,
                    node_ids: ::std::vec::Vec::new(),
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

    // repeated int64 node_ids = 2;

    pub fn clear_node_ids(&mut self) {
        self.node_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_node_ids(&mut self, v: ::std::vec::Vec<i64>) {
        self.node_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_node_ids(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.node_ids
    }

    // Take field
    pub fn take_node_ids(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.node_ids, ::std::vec::Vec::new())
    }

    pub fn get_node_ids(&self) -> &[i64] {
        &self.node_ids
    }
}

impl ::protobuf::Message for RemoveNodesRequest {
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
                    try!(::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.node_ids));
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
        for value in &self.graph_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.node_ids {
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
        for v in &self.node_ids {
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
        ::std::any::TypeId::of::<RemoveNodesRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RemoveNodesRequest {
    fn new() -> RemoveNodesRequest {
        RemoveNodesRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveNodesRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "graph_id",
                    RemoveNodesRequest::has_graph_id,
                    RemoveNodesRequest::get_graph_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "node_ids",
                    RemoveNodesRequest::get_node_ids,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RemoveNodesRequest>(
                    "RemoveNodesRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveNodesRequest {
    fn clear(&mut self) {
        self.clear_graph_id();
        self.clear_node_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RemoveNodesRequest {
    fn eq(&self, other: &RemoveNodesRequest) -> bool {
        self.graph_id == other.graph_id &&
        self.node_ids == other.node_ids &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RemoveNodesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Request {
    // message fields
    request_type: ::std::option::Option<Request_RequestType>,
    add_graph_request: ::protobuf::SingularPtrField<AddGraphRequest>,
    use_graph_request: ::protobuf::SingularPtrField<UseGraphRequest>,
    remove_graph_request: ::protobuf::SingularPtrField<RemoveGraphRequest>,
    add_nodes_request: ::protobuf::SingularPtrField<AddNodesRequest>,
    remove_nodes_request: ::protobuf::SingularPtrField<RemoveNodesRequest>,
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
                    request_type: ::std::option::Option::None,
                    add_graph_request: ::protobuf::SingularPtrField::none(),
                    use_graph_request: ::protobuf::SingularPtrField::none(),
                    remove_graph_request: ::protobuf::SingularPtrField::none(),
                    add_nodes_request: ::protobuf::SingularPtrField::none(),
                    remove_nodes_request: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .Request.RequestType request_type = 1;

    pub fn clear_request_type(&mut self) {
        self.request_type = ::std::option::Option::None;
    }

    pub fn has_request_type(&self) -> bool {
        self.request_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request_type(&mut self, v: Request_RequestType) {
        self.request_type = ::std::option::Option::Some(v);
    }

    pub fn get_request_type(&self) -> Request_RequestType {
        self.request_type.unwrap_or(Request_RequestType::GET_VERSION)
    }

    // optional .AddGraphRequest add_graph_request = 2;

    pub fn clear_add_graph_request(&mut self) {
        self.add_graph_request.clear();
    }

    pub fn has_add_graph_request(&self) -> bool {
        self.add_graph_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_add_graph_request(&mut self, v: AddGraphRequest) {
        self.add_graph_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_add_graph_request(&mut self) -> &mut AddGraphRequest {
        if self.add_graph_request.is_none() {
            self.add_graph_request.set_default();
        };
        self.add_graph_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_add_graph_request(&mut self) -> AddGraphRequest {
        self.add_graph_request.take().unwrap_or_else(|| AddGraphRequest::new())
    }

    pub fn get_add_graph_request(&self) -> &AddGraphRequest {
        self.add_graph_request.as_ref().unwrap_or_else(|| AddGraphRequest::default_instance())
    }

    // optional .UseGraphRequest use_graph_request = 3;

    pub fn clear_use_graph_request(&mut self) {
        self.use_graph_request.clear();
    }

    pub fn has_use_graph_request(&self) -> bool {
        self.use_graph_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_use_graph_request(&mut self, v: UseGraphRequest) {
        self.use_graph_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_use_graph_request(&mut self) -> &mut UseGraphRequest {
        if self.use_graph_request.is_none() {
            self.use_graph_request.set_default();
        };
        self.use_graph_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_use_graph_request(&mut self) -> UseGraphRequest {
        self.use_graph_request.take().unwrap_or_else(|| UseGraphRequest::new())
    }

    pub fn get_use_graph_request(&self) -> &UseGraphRequest {
        self.use_graph_request.as_ref().unwrap_or_else(|| UseGraphRequest::default_instance())
    }

    // optional .RemoveGraphRequest remove_graph_request = 4;

    pub fn clear_remove_graph_request(&mut self) {
        self.remove_graph_request.clear();
    }

    pub fn has_remove_graph_request(&self) -> bool {
        self.remove_graph_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remove_graph_request(&mut self, v: RemoveGraphRequest) {
        self.remove_graph_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_remove_graph_request(&mut self) -> &mut RemoveGraphRequest {
        if self.remove_graph_request.is_none() {
            self.remove_graph_request.set_default();
        };
        self.remove_graph_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_remove_graph_request(&mut self) -> RemoveGraphRequest {
        self.remove_graph_request.take().unwrap_or_else(|| RemoveGraphRequest::new())
    }

    pub fn get_remove_graph_request(&self) -> &RemoveGraphRequest {
        self.remove_graph_request.as_ref().unwrap_or_else(|| RemoveGraphRequest::default_instance())
    }

    // optional .AddNodesRequest add_nodes_request = 5;

    pub fn clear_add_nodes_request(&mut self) {
        self.add_nodes_request.clear();
    }

    pub fn has_add_nodes_request(&self) -> bool {
        self.add_nodes_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_add_nodes_request(&mut self, v: AddNodesRequest) {
        self.add_nodes_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_add_nodes_request(&mut self) -> &mut AddNodesRequest {
        if self.add_nodes_request.is_none() {
            self.add_nodes_request.set_default();
        };
        self.add_nodes_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_add_nodes_request(&mut self) -> AddNodesRequest {
        self.add_nodes_request.take().unwrap_or_else(|| AddNodesRequest::new())
    }

    pub fn get_add_nodes_request(&self) -> &AddNodesRequest {
        self.add_nodes_request.as_ref().unwrap_or_else(|| AddNodesRequest::default_instance())
    }

    // optional .RemoveNodesRequest remove_nodes_request = 6;

    pub fn clear_remove_nodes_request(&mut self) {
        self.remove_nodes_request.clear();
    }

    pub fn has_remove_nodes_request(&self) -> bool {
        self.remove_nodes_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remove_nodes_request(&mut self, v: RemoveNodesRequest) {
        self.remove_nodes_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_remove_nodes_request(&mut self) -> &mut RemoveNodesRequest {
        if self.remove_nodes_request.is_none() {
            self.remove_nodes_request.set_default();
        };
        self.remove_nodes_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_remove_nodes_request(&mut self) -> RemoveNodesRequest {
        self.remove_nodes_request.take().unwrap_or_else(|| RemoveNodesRequest::new())
    }

    pub fn get_remove_nodes_request(&self) -> &RemoveNodesRequest {
        self.remove_nodes_request.as_ref().unwrap_or_else(|| RemoveNodesRequest::default_instance())
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        if self.request_type.is_none() {
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
                    self.request_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.add_graph_request));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.use_graph_request));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.remove_graph_request));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.add_nodes_request));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.remove_nodes_request));
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
        for value in &self.request_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.add_graph_request {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.use_graph_request {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.remove_graph_request {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.add_nodes_request {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.remove_nodes_request {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.request_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.add_graph_request.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.use_graph_request.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.remove_graph_request.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.add_nodes_request.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.remove_nodes_request.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
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
                    "request_type",
                    Request::has_request_type,
                    Request::get_request_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "add_graph_request",
                    Request::has_add_graph_request,
                    Request::get_add_graph_request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "use_graph_request",
                    Request::has_use_graph_request,
                    Request::get_use_graph_request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "remove_graph_request",
                    Request::has_remove_graph_request,
                    Request::get_remove_graph_request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "add_nodes_request",
                    Request::has_add_nodes_request,
                    Request::get_add_nodes_request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "remove_nodes_request",
                    Request::has_remove_nodes_request,
                    Request::get_remove_nodes_request,
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
        self.clear_request_type();
        self.clear_add_graph_request();
        self.clear_use_graph_request();
        self.clear_remove_graph_request();
        self.clear_add_nodes_request();
        self.clear_remove_nodes_request();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Request {
    fn eq(&self, other: &Request) -> bool {
        self.request_type == other.request_type &&
        self.add_graph_request == other.add_graph_request &&
        self.use_graph_request == other.use_graph_request &&
        self.remove_graph_request == other.remove_graph_request &&
        self.add_nodes_request == other.add_nodes_request &&
        self.remove_nodes_request == other.remove_nodes_request &&
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
    ADD_GRAPH_REQUEST = 2,
    USE_GRAPH_REQUEST = 3,
    REMOVE_GRAPH_REQUEST = 4,
    ADD_NODES_REQUEST = 5,
    REMOVE_NODES_REQUEST = 7,
}

impl ::protobuf::ProtobufEnum for Request_RequestType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Request_RequestType> {
        match value {
            1 => ::std::option::Option::Some(Request_RequestType::GET_VERSION),
            2 => ::std::option::Option::Some(Request_RequestType::ADD_GRAPH_REQUEST),
            3 => ::std::option::Option::Some(Request_RequestType::USE_GRAPH_REQUEST),
            4 => ::std::option::Option::Some(Request_RequestType::REMOVE_GRAPH_REQUEST),
            5 => ::std::option::Option::Some(Request_RequestType::ADD_NODES_REQUEST),
            7 => ::std::option::Option::Some(Request_RequestType::REMOVE_NODES_REQUEST),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Request_RequestType] = &[
            Request_RequestType::GET_VERSION,
            Request_RequestType::ADD_GRAPH_REQUEST,
            Request_RequestType::USE_GRAPH_REQUEST,
            Request_RequestType::REMOVE_GRAPH_REQUEST,
            Request_RequestType::ADD_NODES_REQUEST,
            Request_RequestType::REMOVE_NODES_REQUEST,
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
        for value in &self.requests {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.requests {
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
    response_type: ::std::option::Option<Response_ResponseType>,
    index: ::std::option::Option<i64>,
    get_version_response: ::protobuf::SingularPtrField<GetVersionResponse>,
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
                    response_type: ::std::option::Option::None,
                    index: ::std::option::Option::None,
                    get_version_response: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .Response.ResponseType response_type = 1;

    pub fn clear_response_type(&mut self) {
        self.response_type = ::std::option::Option::None;
    }

    pub fn has_response_type(&self) -> bool {
        self.response_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response_type(&mut self, v: Response_ResponseType) {
        self.response_type = ::std::option::Option::Some(v);
    }

    pub fn get_response_type(&self) -> Response_ResponseType {
        self.response_type.unwrap_or(Response_ResponseType::GET_VERSION)
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
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        if self.response_type.is_none() {
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
                    self.response_type = ::std::option::Option::Some(tmp);
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
        for value in &self.response_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.index {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.get_version_response {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.response_type {
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
                    "response_type",
                    Response::has_response_type,
                    Response::get_response_type,
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
        self.clear_response_type();
        self.clear_index();
        self.clear_get_version_response();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Response {
    fn eq(&self, other: &Response) -> bool {
        self.response_type == other.response_type &&
        self.index == other.index &&
        self.get_version_response == other.get_version_response &&
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
}

impl ::protobuf::ProtobufEnum for Response_ResponseType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Response_ResponseType> {
        match value {
            1 => ::std::option::Option::Some(Response_ResponseType::GET_VERSION),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Response_ResponseType] = &[
            Response_ResponseType::GET_VERSION,
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
        for value in &self.responses {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.responses {
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
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x25, 0x0a,
    0x12, 0x47, 0x65, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x0f, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x09, 0x22, 0x63, 0x0a, 0x04, 0x4e, 0x6f, 0x64, 0x65, 0x12, 0x21, 0x0a, 0x09,
    0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32,
    0x0e, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12,
    0x17, 0x0a, 0x08, 0x63, 0x68, 0x69, 0x6c, 0x64, 0x72, 0x65, 0x6e, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x05, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x22, 0x1f, 0x0a, 0x08, 0x4e, 0x6f, 0x64, 0x65,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x09, 0x0a, 0x05, 0x45, 0x4d, 0x50, 0x54, 0x59, 0x10, 0x01, 0x12,
    0x08, 0x0a, 0x04, 0x4d, 0x45, 0x53, 0x48, 0x10, 0x02, 0x22, 0x27, 0x0a, 0x0f, 0x41, 0x64, 0x64,
    0x47, 0x72, 0x61, 0x70, 0x68, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x14, 0x0a, 0x05,
    0x6e, 0x6f, 0x64, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x4e, 0x6f,
    0x64, 0x65, 0x22, 0x23, 0x0a, 0x0f, 0x55, 0x73, 0x65, 0x47, 0x72, 0x61, 0x70, 0x68, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x08, 0x67, 0x72, 0x61, 0x70, 0x68, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x03, 0x22, 0x26, 0x0a, 0x12, 0x52, 0x65, 0x6d, 0x6f, 0x76,
    0x65, 0x47, 0x72, 0x61, 0x70, 0x68, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x10, 0x0a,
    0x08, 0x67, 0x72, 0x61, 0x70, 0x68, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x03, 0x22,
    0x69, 0x0a, 0x0f, 0x41, 0x64, 0x64, 0x4e, 0x6f, 0x64, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x10, 0x0a, 0x08, 0x67, 0x72, 0x61, 0x70, 0x68, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x03, 0x12, 0x11, 0x0a, 0x09, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x69,
    0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x12, 0x1b, 0x0a, 0x13, 0x70, 0x72, 0x65, 0x76, 0x69,
    0x6f, 0x75, 0x73, 0x5f, 0x73, 0x69, 0x62, 0x6c, 0x69, 0x6e, 0x67, 0x5f, 0x69, 0x64, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x03, 0x12, 0x14, 0x0a, 0x05, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x18, 0x04, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x05, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x22, 0x38, 0x0a, 0x12, 0x52, 0x65,
    0x6d, 0x6f, 0x76, 0x65, 0x4e, 0x6f, 0x64, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x12, 0x10, 0x0a, 0x08, 0x67, 0x72, 0x61, 0x70, 0x68, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x03, 0x12, 0x10, 0x0a, 0x08, 0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x03, 0x22, 0xbc, 0x03, 0x0a, 0x07, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x12, 0x2a, 0x0a, 0x0c, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x14, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x2b, 0x0a, 0x11,
    0x61, 0x64, 0x64, 0x5f, 0x67, 0x72, 0x61, 0x70, 0x68, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x41, 0x64, 0x64, 0x47, 0x72, 0x61,
    0x70, 0x68, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2b, 0x0a, 0x11, 0x75, 0x73, 0x65,
    0x5f, 0x67, 0x72, 0x61, 0x70, 0x68, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x55, 0x73, 0x65, 0x47, 0x72, 0x61, 0x70, 0x68, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x31, 0x0a, 0x14, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65,
    0x5f, 0x67, 0x72, 0x61, 0x70, 0x68, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x47, 0x72, 0x61,
    0x70, 0x68, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2b, 0x0a, 0x11, 0x61, 0x64, 0x64,
    0x5f, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x41, 0x64, 0x64, 0x4e, 0x6f, 0x64, 0x65, 0x73, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x31, 0x0a, 0x14, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65,
    0x5f, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x4e, 0x6f, 0x64,
    0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x97, 0x01, 0x0a, 0x0b, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0f, 0x0a, 0x0b, 0x47, 0x45, 0x54,
    0x5f, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4f, 0x4e, 0x10, 0x01, 0x12, 0x15, 0x0a, 0x11, 0x41, 0x44,
    0x44, 0x5f, 0x47, 0x52, 0x41, 0x50, 0x48, 0x5f, 0x52, 0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x10,
    0x02, 0x12, 0x15, 0x0a, 0x11, 0x55, 0x53, 0x45, 0x5f, 0x47, 0x52, 0x41, 0x50, 0x48, 0x5f, 0x52,
    0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x10, 0x03, 0x12, 0x18, 0x0a, 0x14, 0x52, 0x45, 0x4d, 0x4f,
    0x56, 0x45, 0x5f, 0x47, 0x52, 0x41, 0x50, 0x48, 0x5f, 0x52, 0x45, 0x51, 0x55, 0x45, 0x53, 0x54,
    0x10, 0x04, 0x12, 0x15, 0x0a, 0x11, 0x41, 0x44, 0x44, 0x5f, 0x4e, 0x4f, 0x44, 0x45, 0x53, 0x5f,
    0x52, 0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x10, 0x05, 0x12, 0x18, 0x0a, 0x14, 0x52, 0x45, 0x4d,
    0x4f, 0x56, 0x45, 0x5f, 0x4e, 0x4f, 0x44, 0x45, 0x53, 0x5f, 0x52, 0x45, 0x51, 0x55, 0x45, 0x53,
    0x54, 0x10, 0x07, 0x22, 0x26, 0x0a, 0x08, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x12,
    0x1a, 0x0a, 0x08, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x08, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x9c, 0x01, 0x0a, 0x08,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2d, 0x0a, 0x0d, 0x72, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32,
    0x16, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x03, 0x12, 0x31, 0x0a, 0x14, 0x67, 0x65, 0x74, 0x5f, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x47, 0x65, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x1f, 0x0a, 0x0c, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0f, 0x0a, 0x0b, 0x47, 0x45, 0x54,
    0x5f, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4f, 0x4e, 0x10, 0x01, 0x22, 0x29, 0x0a, 0x09, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x12, 0x1c, 0x0a, 0x09, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x4a, 0xf9, 0x13, 0x0a, 0x07, 0x12, 0x05, 0x08, 0x00, 0xa5, 0x01,
    0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x09, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x12, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x1c, 0x1d, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x44, 0x00, 0x4c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x44, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x04, 0x00, 0x12, 0x04,
    0x45, 0x02, 0x48, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x04, 0x00, 0x01, 0x12, 0x03, 0x45,
    0x07, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x46, 0x04,
    0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x46, 0x04,
    0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x46, 0x0c,
    0x0d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x47, 0x04, 0x0d,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x47, 0x04, 0x08,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x47, 0x0b, 0x0c,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x4a, 0x02, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x4a, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x4a, 0x14, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x4a, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x4b,
    0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4b, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x4b, 0x0b, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4b, 0x10, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4b, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02,
    0x12, 0x04, 0x50, 0x00, 0x52, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x50,
    0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x51, 0x02, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x51, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x51, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x51, 0x10, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x51, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x54,
    0x00, 0x56, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x54, 0x08, 0x17, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x55, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x55, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x55, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x55, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x55, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x58, 0x00, 0x5a, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x58, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x59, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x59, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x59, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x59, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x59, 0x1c,
    0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x5c, 0x00, 0x61, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x5c, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x00, 0x12, 0x03, 0x5d, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x5d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5d,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5d, 0x11, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5d, 0x1c, 0x1d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x5e, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x5e, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x5e, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x5e, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x5f, 0x02, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x5f, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x5f, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5f, 0x11, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x5f, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03,
    0x12, 0x03, 0x60, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x60, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x06, 0x12, 0x03, 0x60, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x60, 0x10, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x60, 0x18, 0x19, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x06, 0x12, 0x04, 0x68, 0x00, 0x6b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01,
    0x12, 0x03, 0x68, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x69,
    0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x69, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x69, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x69, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x69, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x01, 0x12, 0x03, 0x6a, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x6a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x6a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6a, 0x11,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6a, 0x1c, 0x1d, 0x0a,
    0x0b, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x05, 0x6f, 0x00, 0x8f, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x07, 0x01, 0x12, 0x03, 0x6f, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x04, 0x00,
    0x12, 0x04, 0x70, 0x02, 0x78, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x70, 0x07, 0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x71, 0x04, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x71, 0x04, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03,
    0x71, 0x12, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x72,
    0x04, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x72,
    0x04, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x72,
    0x18, 0x19, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x73, 0x04,
    0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x73, 0x04,
    0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x73, 0x18,
    0x19, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x74, 0x04, 0x1d,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x74, 0x04, 0x18,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x74, 0x1b, 0x1c,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x75, 0x04, 0x1a, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x75, 0x04, 0x15, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x75, 0x18, 0x19, 0x0a,
    0x2a, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x77, 0x04, 0x1d, 0x1a, 0x1b,
    0x20, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x5f, 0x4e, 0x4f, 0x44, 0x45, 0x53, 0x5f, 0x52, 0x45,
    0x51, 0x55, 0x45, 0x53, 0x54, 0x20, 0x3d, 0x20, 0x36, 0x3b, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x07, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x77, 0x04, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x07, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x77, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x00, 0x12, 0x03, 0x7a, 0x02, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x7a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x7a, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7a,
    0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7a, 0x26, 0x27,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x7c, 0x02, 0x31, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x06, 0x12, 0x03, 0x7c, 0x0b, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x7c, 0x1b, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x7c, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x03, 0x7d,
    0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x03, 0x7d, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x06, 0x12, 0x03, 0x7d, 0x0b, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x7d, 0x1b, 0x2c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x7d, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07,
    0x02, 0x03, 0x12, 0x03, 0x7e, 0x02, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x7e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x06, 0x12, 0x03,
    0x7e, 0x0b, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x03, 0x7e, 0x1e,
    0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03, 0x12, 0x03, 0x7e, 0x35, 0x36, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x04, 0x12, 0x03, 0x7f, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x04, 0x04, 0x12, 0x03, 0x7f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x04, 0x06, 0x12, 0x03, 0x7f, 0x0b, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x7f, 0x1b, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x7f, 0x2f, 0x30, 0x0a, 0x45, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x05, 0x12, 0x04, 0x81, 0x01,
    0x02, 0x37, 0x1a, 0x37, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x55, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x4e, 0x6f, 0x64, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x5f, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x3d, 0x20, 0x32, 0x3b, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x05, 0x04, 0x12, 0x04, 0x81, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x05, 0x06, 0x12, 0x04, 0x81, 0x01, 0x0b, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x05, 0x01, 0x12, 0x04, 0x81, 0x01, 0x1e, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05,
    0x03, 0x12, 0x04, 0x81, 0x01, 0x35, 0x36, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x06, 0x91,
    0x01, 0x00, 0x93, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x04, 0x91, 0x01,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x04, 0x92, 0x01, 0x02, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0x92, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x04, 0x92, 0x01, 0x0b, 0x12, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x04, 0x92, 0x01, 0x13, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x04, 0x92, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x09, 0x12, 0x06, 0x97, 0x01, 0x00, 0xa1, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x09,
    0x01, 0x12, 0x04, 0x97, 0x01, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x09, 0x04, 0x00, 0x12,
    0x06, 0x98, 0x01, 0x02, 0x9a, 0x01, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x04, 0x00, 0x01,
    0x12, 0x04, 0x98, 0x01, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x04, 0x99, 0x01, 0x04, 0x14, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x99, 0x01, 0x04, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x04, 0x99, 0x01, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00,
    0x12, 0x04, 0x9c, 0x01, 0x02, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x9c, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x04,
    0x9c, 0x01, 0x0b, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9c,
    0x01, 0x18, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9c, 0x01,
    0x28, 0x29, 0x0a, 0x42, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x02, 0x1b,
    0x1a, 0x34, 0x20, 0x54, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73,
    0x20, 0x66, 0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x9e, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x04,
    0x9e, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9e,
    0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9e, 0x01,
    0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x04, 0xa0, 0x01, 0x02, 0x37,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa0, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x06, 0x12, 0x04, 0xa0, 0x01, 0x0b, 0x1d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa0, 0x01, 0x1e, 0x32, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x04, 0xa0, 0x01, 0x35, 0x36, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x0a, 0x12, 0x06, 0xa3, 0x01, 0x00, 0xa5, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a,
    0x01, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12,
    0x04, 0xa4, 0x01, 0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xa4, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x04, 0xa4,
    0x01, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa4, 0x01,
    0x14, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa4, 0x01, 0x20,
    0x21,
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
