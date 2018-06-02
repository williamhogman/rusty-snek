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
pub struct ClientCommand {
    // message oneof groups
    command: ::std::option::Option<ClientCommand_oneof_command>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientCommand {}

#[derive(Clone,PartialEq)]
pub enum ClientCommand_oneof_command {
    direction(DirectionCommand),
}

impl ClientCommand {
    pub fn new() -> ClientCommand {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientCommand {
        static mut instance: ::protobuf::lazy::Lazy<ClientCommand> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientCommand,
        };
        unsafe {
            instance.get(ClientCommand::new)
        }
    }

    // .DirectionCommand direction = 1;

    pub fn clear_direction(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_direction(&self) -> bool {
        match self.command {
            ::std::option::Option::Some(ClientCommand_oneof_command::direction(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_direction(&mut self, v: DirectionCommand) {
        self.command = ::std::option::Option::Some(ClientCommand_oneof_command::direction(v))
    }

    // Mutable pointer to the field.
    pub fn mut_direction(&mut self) -> &mut DirectionCommand {
        if let ::std::option::Option::Some(ClientCommand_oneof_command::direction(_)) = self.command {
        } else {
            self.command = ::std::option::Option::Some(ClientCommand_oneof_command::direction(DirectionCommand::new()));
        }
        match self.command {
            ::std::option::Option::Some(ClientCommand_oneof_command::direction(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_direction(&mut self) -> DirectionCommand {
        if self.has_direction() {
            match self.command.take() {
                ::std::option::Option::Some(ClientCommand_oneof_command::direction(v)) => v,
                _ => panic!(),
            }
        } else {
            DirectionCommand::new()
        }
    }

    pub fn get_direction(&self) -> &DirectionCommand {
        match self.command {
            ::std::option::Option::Some(ClientCommand_oneof_command::direction(ref v)) => v,
            _ => DirectionCommand::default_instance(),
        }
    }
}

impl ::protobuf::Message for ClientCommand {
    fn is_initialized(&self) -> bool {
        if let Some(ClientCommand_oneof_command::direction(ref v)) = self.command {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.command = ::std::option::Option::Some(ClientCommand_oneof_command::direction(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.command {
            match v {
                &ClientCommand_oneof_command::direction(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.command {
            match v {
                &ClientCommand_oneof_command::direction(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

impl ::protobuf::MessageStatic for ClientCommand {
    fn new() -> ClientCommand {
        ClientCommand::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientCommand>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DirectionCommand>(
                    "direction",
                    ClientCommand::has_direction,
                    ClientCommand::get_direction,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientCommand>(
                    "ClientCommand",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientCommand {
    fn clear(&mut self) {
        self.clear_direction();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientCommand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientCommand {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DirectionCommand {
    // message fields
    pub direction: DirectionCommand_Direction,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DirectionCommand {}

impl DirectionCommand {
    pub fn new() -> DirectionCommand {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DirectionCommand {
        static mut instance: ::protobuf::lazy::Lazy<DirectionCommand> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DirectionCommand,
        };
        unsafe {
            instance.get(DirectionCommand::new)
        }
    }

    // .DirectionCommand.Direction direction = 1;

    pub fn clear_direction(&mut self) {
        self.direction = DirectionCommand_Direction::INVALID;
    }

    // Param is passed by value, moved
    pub fn set_direction(&mut self, v: DirectionCommand_Direction) {
        self.direction = v;
    }

    pub fn get_direction(&self) -> DirectionCommand_Direction {
        self.direction
    }

    fn get_direction_for_reflect(&self) -> &DirectionCommand_Direction {
        &self.direction
    }

    fn mut_direction_for_reflect(&mut self) -> &mut DirectionCommand_Direction {
        &mut self.direction
    }
}

impl ::protobuf::Message for DirectionCommand {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.direction, 1, &mut self.unknown_fields)?
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
        if self.direction != DirectionCommand_Direction::INVALID {
            my_size += ::protobuf::rt::enum_size(1, self.direction);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.direction != DirectionCommand_Direction::INVALID {
            os.write_enum(1, self.direction.value())?;
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

impl ::protobuf::MessageStatic for DirectionCommand {
    fn new() -> DirectionCommand {
        DirectionCommand::new()
    }

    fn descriptor_static(_: ::std::option::Option<DirectionCommand>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DirectionCommand_Direction>>(
                    "direction",
                    DirectionCommand::get_direction_for_reflect,
                    DirectionCommand::mut_direction_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DirectionCommand>(
                    "DirectionCommand",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DirectionCommand {
    fn clear(&mut self) {
        self.clear_direction();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DirectionCommand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DirectionCommand {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DirectionCommand_Direction {
    INVALID = 0,
    NORTH = 1,
    SOUTH = 2,
    WEST = 3,
    EAST = 4,
}

impl ::protobuf::ProtobufEnum for DirectionCommand_Direction {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DirectionCommand_Direction> {
        match value {
            0 => ::std::option::Option::Some(DirectionCommand_Direction::INVALID),
            1 => ::std::option::Option::Some(DirectionCommand_Direction::NORTH),
            2 => ::std::option::Option::Some(DirectionCommand_Direction::SOUTH),
            3 => ::std::option::Option::Some(DirectionCommand_Direction::WEST),
            4 => ::std::option::Option::Some(DirectionCommand_Direction::EAST),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DirectionCommand_Direction] = &[
            DirectionCommand_Direction::INVALID,
            DirectionCommand_Direction::NORTH,
            DirectionCommand_Direction::SOUTH,
            DirectionCommand_Direction::WEST,
            DirectionCommand_Direction::EAST,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DirectionCommand_Direction>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DirectionCommand_Direction", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DirectionCommand_Direction {
}

impl ::std::default::Default for DirectionCommand_Direction {
    fn default() -> Self {
        DirectionCommand_Direction::INVALID
    }
}

impl ::protobuf::reflect::ProtobufValue for DirectionCommand_Direction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CellUpdate {
    // message fields
    pub cell: u32,
    pub content: CellContent,
    pub player: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CellUpdate {}

impl CellUpdate {
    pub fn new() -> CellUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CellUpdate {
        static mut instance: ::protobuf::lazy::Lazy<CellUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CellUpdate,
        };
        unsafe {
            instance.get(CellUpdate::new)
        }
    }

    // uint32 cell = 1;

    pub fn clear_cell(&mut self) {
        self.cell = 0;
    }

    // Param is passed by value, moved
    pub fn set_cell(&mut self, v: u32) {
        self.cell = v;
    }

    pub fn get_cell(&self) -> u32 {
        self.cell
    }

    fn get_cell_for_reflect(&self) -> &u32 {
        &self.cell
    }

    fn mut_cell_for_reflect(&mut self) -> &mut u32 {
        &mut self.cell
    }

    // .CellContent content = 2;

    pub fn clear_content(&mut self) {
        self.content = CellContent::UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: CellContent) {
        self.content = v;
    }

    pub fn get_content(&self) -> CellContent {
        self.content
    }

    fn get_content_for_reflect(&self) -> &CellContent {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut CellContent {
        &mut self.content
    }

    // uint32 player = 3;

    pub fn clear_player(&mut self) {
        self.player = 0;
    }

    // Param is passed by value, moved
    pub fn set_player(&mut self, v: u32) {
        self.player = v;
    }

    pub fn get_player(&self) -> u32 {
        self.player
    }

    fn get_player_for_reflect(&self) -> &u32 {
        &self.player
    }

    fn mut_player_for_reflect(&mut self) -> &mut u32 {
        &mut self.player
    }
}

impl ::protobuf::Message for CellUpdate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.cell = tmp;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.content, 2, &mut self.unknown_fields)?
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.player = tmp;
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
        if self.cell != 0 {
            my_size += ::protobuf::rt::value_size(1, self.cell, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.content != CellContent::UNKNOWN {
            my_size += ::protobuf::rt::enum_size(2, self.content);
        }
        if self.player != 0 {
            my_size += ::protobuf::rt::value_size(3, self.player, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.cell != 0 {
            os.write_uint32(1, self.cell)?;
        }
        if self.content != CellContent::UNKNOWN {
            os.write_enum(2, self.content.value())?;
        }
        if self.player != 0 {
            os.write_uint32(3, self.player)?;
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

impl ::protobuf::MessageStatic for CellUpdate {
    fn new() -> CellUpdate {
        CellUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CellUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "cell",
                    CellUpdate::get_cell_for_reflect,
                    CellUpdate::mut_cell_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CellContent>>(
                    "content",
                    CellUpdate::get_content_for_reflect,
                    CellUpdate::mut_content_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player",
                    CellUpdate::get_player_for_reflect,
                    CellUpdate::mut_player_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CellUpdate>(
                    "CellUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CellUpdate {
    fn clear(&mut self) {
        self.clear_cell();
        self.clear_content();
        self.clear_player();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CellUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CellUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScoreBoardUpdate {
    // message fields
    pub player: u32,
    pub score: u32,
    pub dead: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScoreBoardUpdate {}

impl ScoreBoardUpdate {
    pub fn new() -> ScoreBoardUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScoreBoardUpdate {
        static mut instance: ::protobuf::lazy::Lazy<ScoreBoardUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScoreBoardUpdate,
        };
        unsafe {
            instance.get(ScoreBoardUpdate::new)
        }
    }

    // uint32 player = 1;

    pub fn clear_player(&mut self) {
        self.player = 0;
    }

    // Param is passed by value, moved
    pub fn set_player(&mut self, v: u32) {
        self.player = v;
    }

    pub fn get_player(&self) -> u32 {
        self.player
    }

    fn get_player_for_reflect(&self) -> &u32 {
        &self.player
    }

    fn mut_player_for_reflect(&mut self) -> &mut u32 {
        &mut self.player
    }

    // uint32 score = 2;

    pub fn clear_score(&mut self) {
        self.score = 0;
    }

    // Param is passed by value, moved
    pub fn set_score(&mut self, v: u32) {
        self.score = v;
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    fn get_score_for_reflect(&self) -> &u32 {
        &self.score
    }

    fn mut_score_for_reflect(&mut self) -> &mut u32 {
        &mut self.score
    }

    // bool dead = 3;

    pub fn clear_dead(&mut self) {
        self.dead = false;
    }

    // Param is passed by value, moved
    pub fn set_dead(&mut self, v: bool) {
        self.dead = v;
    }

    pub fn get_dead(&self) -> bool {
        self.dead
    }

    fn get_dead_for_reflect(&self) -> &bool {
        &self.dead
    }

    fn mut_dead_for_reflect(&mut self) -> &mut bool {
        &mut self.dead
    }
}

impl ::protobuf::Message for ScoreBoardUpdate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.player = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.score = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.dead = tmp;
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
        if self.player != 0 {
            my_size += ::protobuf::rt::value_size(1, self.player, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.score != 0 {
            my_size += ::protobuf::rt::value_size(2, self.score, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.dead != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.player != 0 {
            os.write_uint32(1, self.player)?;
        }
        if self.score != 0 {
            os.write_uint32(2, self.score)?;
        }
        if self.dead != false {
            os.write_bool(3, self.dead)?;
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

impl ::protobuf::MessageStatic for ScoreBoardUpdate {
    fn new() -> ScoreBoardUpdate {
        ScoreBoardUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScoreBoardUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player",
                    ScoreBoardUpdate::get_player_for_reflect,
                    ScoreBoardUpdate::mut_player_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "score",
                    ScoreBoardUpdate::get_score_for_reflect,
                    ScoreBoardUpdate::mut_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "dead",
                    ScoreBoardUpdate::get_dead_for_reflect,
                    ScoreBoardUpdate::mut_dead_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScoreBoardUpdate>(
                    "ScoreBoardUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScoreBoardUpdate {
    fn clear(&mut self) {
        self.clear_player();
        self.clear_score();
        self.clear_dead();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScoreBoardUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScoreBoardUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GameStateUpdate {
    // message fields
    pub turn: u32,
    pub cells: ::protobuf::RepeatedField<CellUpdate>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GameStateUpdate {}

impl GameStateUpdate {
    pub fn new() -> GameStateUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GameStateUpdate {
        static mut instance: ::protobuf::lazy::Lazy<GameStateUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GameStateUpdate,
        };
        unsafe {
            instance.get(GameStateUpdate::new)
        }
    }

    // uint32 turn = 1;

    pub fn clear_turn(&mut self) {
        self.turn = 0;
    }

    // Param is passed by value, moved
    pub fn set_turn(&mut self, v: u32) {
        self.turn = v;
    }

    pub fn get_turn(&self) -> u32 {
        self.turn
    }

    fn get_turn_for_reflect(&self) -> &u32 {
        &self.turn
    }

    fn mut_turn_for_reflect(&mut self) -> &mut u32 {
        &mut self.turn
    }

    // repeated .CellUpdate cells = 2;

    pub fn clear_cells(&mut self) {
        self.cells.clear();
    }

    // Param is passed by value, moved
    pub fn set_cells(&mut self, v: ::protobuf::RepeatedField<CellUpdate>) {
        self.cells = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cells(&mut self) -> &mut ::protobuf::RepeatedField<CellUpdate> {
        &mut self.cells
    }

    // Take field
    pub fn take_cells(&mut self) -> ::protobuf::RepeatedField<CellUpdate> {
        ::std::mem::replace(&mut self.cells, ::protobuf::RepeatedField::new())
    }

    pub fn get_cells(&self) -> &[CellUpdate] {
        &self.cells
    }

    fn get_cells_for_reflect(&self) -> &::protobuf::RepeatedField<CellUpdate> {
        &self.cells
    }

    fn mut_cells_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CellUpdate> {
        &mut self.cells
    }
}

impl ::protobuf::Message for GameStateUpdate {
    fn is_initialized(&self) -> bool {
        for v in &self.cells {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.turn = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cells)?;
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
        if self.turn != 0 {
            my_size += ::protobuf::rt::value_size(1, self.turn, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.cells {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.turn != 0 {
            os.write_uint32(1, self.turn)?;
        }
        for v in &self.cells {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for GameStateUpdate {
    fn new() -> GameStateUpdate {
        GameStateUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<GameStateUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "turn",
                    GameStateUpdate::get_turn_for_reflect,
                    GameStateUpdate::mut_turn_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CellUpdate>>(
                    "cells",
                    GameStateUpdate::get_cells_for_reflect,
                    GameStateUpdate::mut_cells_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GameStateUpdate>(
                    "GameStateUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GameStateUpdate {
    fn clear(&mut self) {
        self.clear_turn();
        self.clear_cells();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GameStateUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GameStateUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ServerCommand {
    // message oneof groups
    command: ::std::option::Option<ServerCommand_oneof_command>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServerCommand {}

#[derive(Clone,PartialEq)]
pub enum ServerCommand_oneof_command {
    game_state_update(GameStateUpdate),
}

impl ServerCommand {
    pub fn new() -> ServerCommand {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServerCommand {
        static mut instance: ::protobuf::lazy::Lazy<ServerCommand> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServerCommand,
        };
        unsafe {
            instance.get(ServerCommand::new)
        }
    }

    // .GameStateUpdate game_state_update = 1;

    pub fn clear_game_state_update(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_game_state_update(&self) -> bool {
        match self.command {
            ::std::option::Option::Some(ServerCommand_oneof_command::game_state_update(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_game_state_update(&mut self, v: GameStateUpdate) {
        self.command = ::std::option::Option::Some(ServerCommand_oneof_command::game_state_update(v))
    }

    // Mutable pointer to the field.
    pub fn mut_game_state_update(&mut self) -> &mut GameStateUpdate {
        if let ::std::option::Option::Some(ServerCommand_oneof_command::game_state_update(_)) = self.command {
        } else {
            self.command = ::std::option::Option::Some(ServerCommand_oneof_command::game_state_update(GameStateUpdate::new()));
        }
        match self.command {
            ::std::option::Option::Some(ServerCommand_oneof_command::game_state_update(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_game_state_update(&mut self) -> GameStateUpdate {
        if self.has_game_state_update() {
            match self.command.take() {
                ::std::option::Option::Some(ServerCommand_oneof_command::game_state_update(v)) => v,
                _ => panic!(),
            }
        } else {
            GameStateUpdate::new()
        }
    }

    pub fn get_game_state_update(&self) -> &GameStateUpdate {
        match self.command {
            ::std::option::Option::Some(ServerCommand_oneof_command::game_state_update(ref v)) => v,
            _ => GameStateUpdate::default_instance(),
        }
    }
}

impl ::protobuf::Message for ServerCommand {
    fn is_initialized(&self) -> bool {
        if let Some(ServerCommand_oneof_command::game_state_update(ref v)) = self.command {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.command = ::std::option::Option::Some(ServerCommand_oneof_command::game_state_update(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.command {
            match v {
                &ServerCommand_oneof_command::game_state_update(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.command {
            match v {
                &ServerCommand_oneof_command::game_state_update(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

impl ::protobuf::MessageStatic for ServerCommand {
    fn new() -> ServerCommand {
        ServerCommand::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServerCommand>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GameStateUpdate>(
                    "game_state_update",
                    ServerCommand::has_game_state_update,
                    ServerCommand::get_game_state_update,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServerCommand>(
                    "ServerCommand",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServerCommand {
    fn clear(&mut self) {
        self.clear_game_state_update();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServerCommand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServerCommand {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PingRequest {
    // message fields
    pub version: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PingRequest {}

impl PingRequest {
    pub fn new() -> PingRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PingRequest {
        static mut instance: ::protobuf::lazy::Lazy<PingRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PingRequest,
        };
        unsafe {
            instance.get(PingRequest::new)
        }
    }

    // string version = 1;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.version, ::std::string::String::new())
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    fn get_version_for_reflect(&self) -> &::std::string::String {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }
}

impl ::protobuf::Message for PingRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.version)?;
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
        if !self.version.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.version);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.version.is_empty() {
            os.write_string(1, &self.version)?;
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

impl ::protobuf::MessageStatic for PingRequest {
    fn new() -> PingRequest {
        PingRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<PingRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    PingRequest::get_version_for_reflect,
                    PingRequest::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PingRequest>(
                    "PingRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PingRequest {
    fn clear(&mut self) {
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PingRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PingRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PingResponse {
    // message fields
    pub version: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PingResponse {}

impl PingResponse {
    pub fn new() -> PingResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PingResponse {
        static mut instance: ::protobuf::lazy::Lazy<PingResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PingResponse,
        };
        unsafe {
            instance.get(PingResponse::new)
        }
    }

    // string version = 1;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.version, ::std::string::String::new())
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    fn get_version_for_reflect(&self) -> &::std::string::String {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }
}

impl ::protobuf::Message for PingResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.version)?;
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
        if !self.version.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.version);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.version.is_empty() {
            os.write_string(1, &self.version)?;
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

impl ::protobuf::MessageStatic for PingResponse {
    fn new() -> PingResponse {
        PingResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PingResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    PingResponse::get_version_for_reflect,
                    PingResponse::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PingResponse>(
                    "PingResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PingResponse {
    fn clear(&mut self) {
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PingResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PingResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CellContent {
    UNKNOWN = 0,
    BLANK = 1,
    HEAD = 2,
    TAIL = 3,
    FRUIT = 4,
    WALL = 5,
}

impl ::protobuf::ProtobufEnum for CellContent {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CellContent> {
        match value {
            0 => ::std::option::Option::Some(CellContent::UNKNOWN),
            1 => ::std::option::Option::Some(CellContent::BLANK),
            2 => ::std::option::Option::Some(CellContent::HEAD),
            3 => ::std::option::Option::Some(CellContent::TAIL),
            4 => ::std::option::Option::Some(CellContent::FRUIT),
            5 => ::std::option::Option::Some(CellContent::WALL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CellContent] = &[
            CellContent::UNKNOWN,
            CellContent::BLANK,
            CellContent::HEAD,
            CellContent::TAIL,
            CellContent::FRUIT,
            CellContent::WALL,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CellContent>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CellContent", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CellContent {
}

impl ::std::default::Default for CellContent {
    fn default() -> Self {
        CellContent::UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for CellContent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11snake/snake.proto\"M\n\rClientCommand\x121\n\tdirection\x18\x01\
    \x20\x01(\x0b2\x11.DirectionCommandH\0R\tdirectionB\t\n\x07command\"\x91\
    \x01\n\x10DirectionCommand\x129\n\tdirection\x18\x01\x20\x01(\x0e2\x1b.D\
    irectionCommand.DirectionR\tdirection\"B\n\tDirection\x12\x0b\n\x07INVAL\
    ID\x10\0\x12\t\n\x05NORTH\x10\x01\x12\t\n\x05SOUTH\x10\x02\x12\x08\n\x04\
    WEST\x10\x03\x12\x08\n\x04EAST\x10\x04\"`\n\nCellUpdate\x12\x12\n\x04cel\
    l\x18\x01\x20\x01(\rR\x04cell\x12&\n\x07content\x18\x02\x20\x01(\x0e2\
    \x0c.CellContentR\x07content\x12\x16\n\x06player\x18\x03\x20\x01(\rR\x06\
    player\"T\n\x10ScoreBoardUpdate\x12\x16\n\x06player\x18\x01\x20\x01(\rR\
    \x06player\x12\x14\n\x05score\x18\x02\x20\x01(\rR\x05score\x12\x12\n\x04\
    dead\x18\x03\x20\x01(\x08R\x04dead\"H\n\x0fGameStateUpdate\x12\x12\n\x04\
    turn\x18\x01\x20\x01(\rR\x04turn\x12!\n\x05cells\x18\x02\x20\x03(\x0b2\
    \x0b.CellUpdateR\x05cells\"Z\n\rServerCommand\x12>\n\x11game_state_updat\
    e\x18\x01\x20\x01(\x0b2\x10.GameStateUpdateH\0R\x0fgameStateUpdateB\t\n\
    \x07command\"'\n\x0bPingRequest\x12\x18\n\x07version\x18\x01\x20\x01(\tR\
    \x07version\"(\n\x0cPingResponse\x12\x18\n\x07version\x18\x01\x20\x01(\t\
    R\x07version*N\n\x0bCellContent\x12\x0b\n\x07UNKNOWN\x10\0\x12\t\n\x05BL\
    ANK\x10\x01\x12\x08\n\x04HEAD\x10\x02\x12\x08\n\x04TAIL\x10\x03\x12\t\n\
    \x05FRUIT\x10\x04\x12\x08\n\x04WALL\x10\x052\\\n\x05Snake\x12%\n\x04Ping\
    \x12\x0c.PingRequest\x1a\r.PingResponse\"\0\x12,\n\x04Join\x12\x0e.Clien\
    tCommand\x1a\x0e.ServerCommand\"\0(\x010\x01b\x06proto3\
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
