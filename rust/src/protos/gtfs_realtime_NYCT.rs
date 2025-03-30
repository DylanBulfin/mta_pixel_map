// This file is generated by rust-protobuf 3.7.2. Do not edit
// .proto file is parsed by protoc 29.3
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `gtfs-realtime-NYCT.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

// @@protoc_insertion_point(message:transit_realtime.TripReplacementPeriod)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TripReplacementPeriod {
    // message fields
    // @@protoc_insertion_point(field:transit_realtime.TripReplacementPeriod.route_id)
    pub route_id: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:transit_realtime.TripReplacementPeriod.replacement_period)
    pub replacement_period: ::protobuf::MessageField<super::gtfs_realtime::TimeRange>,
    // special fields
    // @@protoc_insertion_point(special_field:transit_realtime.TripReplacementPeriod.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TripReplacementPeriod {
    fn default() -> &'a TripReplacementPeriod {
        <TripReplacementPeriod as ::protobuf::Message>::default_instance()
    }
}

impl TripReplacementPeriod {
    pub fn new() -> TripReplacementPeriod {
        ::std::default::Default::default()
    }

    // optional string route_id = 1;

    pub fn route_id(&self) -> &str {
        match self.route_id.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_route_id(&mut self) {
        self.route_id = ::std::option::Option::None;
    }

    pub fn has_route_id(&self) -> bool {
        self.route_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_route_id(&mut self, v: ::std::string::String) {
        self.route_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_route_id(&mut self) -> &mut ::std::string::String {
        if self.route_id.is_none() {
            self.route_id = ::std::option::Option::Some(::std::string::String::new());
        }
        self.route_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_route_id(&mut self) -> ::std::string::String {
        self.route_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "route_id",
            |m: &TripReplacementPeriod| { &m.route_id },
            |m: &mut TripReplacementPeriod| { &mut m.route_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::gtfs_realtime::TimeRange>(
            "replacement_period",
            |m: &TripReplacementPeriod| { &m.replacement_period },
            |m: &mut TripReplacementPeriod| { &mut m.replacement_period },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TripReplacementPeriod>(
            "TripReplacementPeriod",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TripReplacementPeriod {
    const NAME: &'static str = "TripReplacementPeriod";

    fn is_initialized(&self) -> bool {
        for v in &self.replacement_period {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.route_id = ::std::option::Option::Some(is.read_string()?);
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.replacement_period)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.route_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.replacement_period.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.route_id.as_ref() {
            os.write_string(1, v)?;
        }
        if let Some(v) = self.replacement_period.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> TripReplacementPeriod {
        TripReplacementPeriod::new()
    }

    fn clear(&mut self) {
        self.route_id = ::std::option::Option::None;
        self.replacement_period.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TripReplacementPeriod {
        static instance: TripReplacementPeriod = TripReplacementPeriod {
            route_id: ::std::option::Option::None,
            replacement_period: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TripReplacementPeriod {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TripReplacementPeriod").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TripReplacementPeriod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TripReplacementPeriod {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:transit_realtime.NyctFeedHeader)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NyctFeedHeader {
    // message fields
    // @@protoc_insertion_point(field:transit_realtime.NyctFeedHeader.nyct_subway_version)
    pub nyct_subway_version: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:transit_realtime.NyctFeedHeader.trip_replacement_period)
    pub trip_replacement_period: ::std::vec::Vec<TripReplacementPeriod>,
    // special fields
    // @@protoc_insertion_point(special_field:transit_realtime.NyctFeedHeader.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NyctFeedHeader {
    fn default() -> &'a NyctFeedHeader {
        <NyctFeedHeader as ::protobuf::Message>::default_instance()
    }
}

impl NyctFeedHeader {
    pub fn new() -> NyctFeedHeader {
        ::std::default::Default::default()
    }

    // required string nyct_subway_version = 1;

    pub fn nyct_subway_version(&self) -> &str {
        match self.nyct_subway_version.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_nyct_subway_version(&mut self) {
        self.nyct_subway_version = ::std::option::Option::None;
    }

    pub fn has_nyct_subway_version(&self) -> bool {
        self.nyct_subway_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nyct_subway_version(&mut self, v: ::std::string::String) {
        self.nyct_subway_version = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nyct_subway_version(&mut self) -> &mut ::std::string::String {
        if self.nyct_subway_version.is_none() {
            self.nyct_subway_version = ::std::option::Option::Some(::std::string::String::new());
        }
        self.nyct_subway_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_nyct_subway_version(&mut self) -> ::std::string::String {
        self.nyct_subway_version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "nyct_subway_version",
            |m: &NyctFeedHeader| { &m.nyct_subway_version },
            |m: &mut NyctFeedHeader| { &mut m.nyct_subway_version },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "trip_replacement_period",
            |m: &NyctFeedHeader| { &m.trip_replacement_period },
            |m: &mut NyctFeedHeader| { &mut m.trip_replacement_period },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NyctFeedHeader>(
            "NyctFeedHeader",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NyctFeedHeader {
    const NAME: &'static str = "NyctFeedHeader";

    fn is_initialized(&self) -> bool {
        if self.nyct_subway_version.is_none() {
            return false;
        }
        for v in &self.trip_replacement_period {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.nyct_subway_version = ::std::option::Option::Some(is.read_string()?);
                },
                18 => {
                    self.trip_replacement_period.push(is.read_message()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.nyct_subway_version.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.trip_replacement_period {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.nyct_subway_version.as_ref() {
            os.write_string(1, v)?;
        }
        for v in &self.trip_replacement_period {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> NyctFeedHeader {
        NyctFeedHeader::new()
    }

    fn clear(&mut self) {
        self.nyct_subway_version = ::std::option::Option::None;
        self.trip_replacement_period.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NyctFeedHeader {
        static instance: NyctFeedHeader = NyctFeedHeader {
            nyct_subway_version: ::std::option::Option::None,
            trip_replacement_period: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NyctFeedHeader {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NyctFeedHeader").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NyctFeedHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NyctFeedHeader {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:transit_realtime.NyctTripDescriptor)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NyctTripDescriptor {
    // message fields
    // @@protoc_insertion_point(field:transit_realtime.NyctTripDescriptor.train_id)
    pub train_id: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:transit_realtime.NyctTripDescriptor.is_assigned)
    pub is_assigned: ::std::option::Option<bool>,
    // @@protoc_insertion_point(field:transit_realtime.NyctTripDescriptor.direction)
    pub direction: ::std::option::Option<::protobuf::EnumOrUnknown<nyct_trip_descriptor::Direction>>,
    // special fields
    // @@protoc_insertion_point(special_field:transit_realtime.NyctTripDescriptor.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NyctTripDescriptor {
    fn default() -> &'a NyctTripDescriptor {
        <NyctTripDescriptor as ::protobuf::Message>::default_instance()
    }
}

impl NyctTripDescriptor {
    pub fn new() -> NyctTripDescriptor {
        ::std::default::Default::default()
    }

    // optional string train_id = 1;

    pub fn train_id(&self) -> &str {
        match self.train_id.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_train_id(&mut self) {
        self.train_id = ::std::option::Option::None;
    }

    pub fn has_train_id(&self) -> bool {
        self.train_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_train_id(&mut self, v: ::std::string::String) {
        self.train_id = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_train_id(&mut self) -> &mut ::std::string::String {
        if self.train_id.is_none() {
            self.train_id = ::std::option::Option::Some(::std::string::String::new());
        }
        self.train_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_train_id(&mut self) -> ::std::string::String {
        self.train_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional bool is_assigned = 2;

    pub fn is_assigned(&self) -> bool {
        self.is_assigned.unwrap_or(false)
    }

    pub fn clear_is_assigned(&mut self) {
        self.is_assigned = ::std::option::Option::None;
    }

    pub fn has_is_assigned(&self) -> bool {
        self.is_assigned.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_assigned(&mut self, v: bool) {
        self.is_assigned = ::std::option::Option::Some(v);
    }

    // optional .transit_realtime.NyctTripDescriptor.Direction direction = 3;

    pub fn direction(&self) -> nyct_trip_descriptor::Direction {
        match self.direction {
            Some(e) => e.enum_value_or(nyct_trip_descriptor::Direction::NORTH),
            None => nyct_trip_descriptor::Direction::NORTH,
        }
    }

    pub fn clear_direction(&mut self) {
        self.direction = ::std::option::Option::None;
    }

    pub fn has_direction(&self) -> bool {
        self.direction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_direction(&mut self, v: nyct_trip_descriptor::Direction) {
        self.direction = ::std::option::Option::Some(::protobuf::EnumOrUnknown::new(v));
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "train_id",
            |m: &NyctTripDescriptor| { &m.train_id },
            |m: &mut NyctTripDescriptor| { &mut m.train_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "is_assigned",
            |m: &NyctTripDescriptor| { &m.is_assigned },
            |m: &mut NyctTripDescriptor| { &mut m.is_assigned },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "direction",
            |m: &NyctTripDescriptor| { &m.direction },
            |m: &mut NyctTripDescriptor| { &mut m.direction },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NyctTripDescriptor>(
            "NyctTripDescriptor",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NyctTripDescriptor {
    const NAME: &'static str = "NyctTripDescriptor";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.train_id = ::std::option::Option::Some(is.read_string()?);
                },
                16 => {
                    self.is_assigned = ::std::option::Option::Some(is.read_bool()?);
                },
                24 => {
                    self.direction = ::std::option::Option::Some(is.read_enum_or_unknown()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.train_id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.is_assigned {
            my_size += 1 + 1;
        }
        if let Some(v) = self.direction {
            my_size += ::protobuf::rt::int32_size(3, v.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.train_id.as_ref() {
            os.write_string(1, v)?;
        }
        if let Some(v) = self.is_assigned {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.direction {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&v))?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> NyctTripDescriptor {
        NyctTripDescriptor::new()
    }

    fn clear(&mut self) {
        self.train_id = ::std::option::Option::None;
        self.is_assigned = ::std::option::Option::None;
        self.direction = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NyctTripDescriptor {
        static instance: NyctTripDescriptor = NyctTripDescriptor {
            train_id: ::std::option::Option::None,
            is_assigned: ::std::option::Option::None,
            direction: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NyctTripDescriptor {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NyctTripDescriptor").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NyctTripDescriptor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NyctTripDescriptor {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `NyctTripDescriptor`
pub mod nyct_trip_descriptor {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:transit_realtime.NyctTripDescriptor.Direction)
    pub enum Direction {
        // @@protoc_insertion_point(enum_value:transit_realtime.NyctTripDescriptor.Direction.NORTH)
        NORTH = 1,
        // @@protoc_insertion_point(enum_value:transit_realtime.NyctTripDescriptor.Direction.EAST)
        EAST = 2,
        // @@protoc_insertion_point(enum_value:transit_realtime.NyctTripDescriptor.Direction.SOUTH)
        SOUTH = 3,
        // @@protoc_insertion_point(enum_value:transit_realtime.NyctTripDescriptor.Direction.WEST)
        WEST = 4,
    }

    impl ::protobuf::Enum for Direction {
        const NAME: &'static str = "Direction";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<Direction> {
            match value {
                1 => ::std::option::Option::Some(Direction::NORTH),
                2 => ::std::option::Option::Some(Direction::EAST),
                3 => ::std::option::Option::Some(Direction::SOUTH),
                4 => ::std::option::Option::Some(Direction::WEST),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<Direction> {
            match str {
                "NORTH" => ::std::option::Option::Some(Direction::NORTH),
                "EAST" => ::std::option::Option::Some(Direction::EAST),
                "SOUTH" => ::std::option::Option::Some(Direction::SOUTH),
                "WEST" => ::std::option::Option::Some(Direction::WEST),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [Direction] = &[
            Direction::NORTH,
            Direction::EAST,
            Direction::SOUTH,
            Direction::WEST,
        ];
    }

    impl ::protobuf::EnumFull for Direction {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("NyctTripDescriptor.Direction").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = match self {
                Direction::NORTH => 0,
                Direction::EAST => 1,
                Direction::SOUTH => 2,
                Direction::WEST => 3,
            };
            Self::enum_descriptor().value_by_index(index)
        }
    }

    // Note, `Default` is implemented although default value is not 0
    impl ::std::default::Default for Direction {
        fn default() -> Self {
            Direction::NORTH
        }
    }

    impl Direction {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Direction>("NyctTripDescriptor.Direction")
        }
    }
}

// @@protoc_insertion_point(message:transit_realtime.NyctStopTimeUpdate)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NyctStopTimeUpdate {
    // message fields
    // @@protoc_insertion_point(field:transit_realtime.NyctStopTimeUpdate.scheduled_track)
    pub scheduled_track: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:transit_realtime.NyctStopTimeUpdate.actual_track)
    pub actual_track: ::std::option::Option<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:transit_realtime.NyctStopTimeUpdate.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NyctStopTimeUpdate {
    fn default() -> &'a NyctStopTimeUpdate {
        <NyctStopTimeUpdate as ::protobuf::Message>::default_instance()
    }
}

impl NyctStopTimeUpdate {
    pub fn new() -> NyctStopTimeUpdate {
        ::std::default::Default::default()
    }

    // optional string scheduled_track = 1;

    pub fn scheduled_track(&self) -> &str {
        match self.scheduled_track.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_scheduled_track(&mut self) {
        self.scheduled_track = ::std::option::Option::None;
    }

    pub fn has_scheduled_track(&self) -> bool {
        self.scheduled_track.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scheduled_track(&mut self, v: ::std::string::String) {
        self.scheduled_track = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scheduled_track(&mut self) -> &mut ::std::string::String {
        if self.scheduled_track.is_none() {
            self.scheduled_track = ::std::option::Option::Some(::std::string::String::new());
        }
        self.scheduled_track.as_mut().unwrap()
    }

    // Take field
    pub fn take_scheduled_track(&mut self) -> ::std::string::String {
        self.scheduled_track.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string actual_track = 2;

    pub fn actual_track(&self) -> &str {
        match self.actual_track.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_actual_track(&mut self) {
        self.actual_track = ::std::option::Option::None;
    }

    pub fn has_actual_track(&self) -> bool {
        self.actual_track.is_some()
    }

    // Param is passed by value, moved
    pub fn set_actual_track(&mut self, v: ::std::string::String) {
        self.actual_track = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_actual_track(&mut self) -> &mut ::std::string::String {
        if self.actual_track.is_none() {
            self.actual_track = ::std::option::Option::Some(::std::string::String::new());
        }
        self.actual_track.as_mut().unwrap()
    }

    // Take field
    pub fn take_actual_track(&mut self) -> ::std::string::String {
        self.actual_track.take().unwrap_or_else(|| ::std::string::String::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "scheduled_track",
            |m: &NyctStopTimeUpdate| { &m.scheduled_track },
            |m: &mut NyctStopTimeUpdate| { &mut m.scheduled_track },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "actual_track",
            |m: &NyctStopTimeUpdate| { &m.actual_track },
            |m: &mut NyctStopTimeUpdate| { &mut m.actual_track },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NyctStopTimeUpdate>(
            "NyctStopTimeUpdate",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NyctStopTimeUpdate {
    const NAME: &'static str = "NyctStopTimeUpdate";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.scheduled_track = ::std::option::Option::Some(is.read_string()?);
                },
                18 => {
                    self.actual_track = ::std::option::Option::Some(is.read_string()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.scheduled_track.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.actual_track.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.scheduled_track.as_ref() {
            os.write_string(1, v)?;
        }
        if let Some(v) = self.actual_track.as_ref() {
            os.write_string(2, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> NyctStopTimeUpdate {
        NyctStopTimeUpdate::new()
    }

    fn clear(&mut self) {
        self.scheduled_track = ::std::option::Option::None;
        self.actual_track = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NyctStopTimeUpdate {
        static instance: NyctStopTimeUpdate = NyctStopTimeUpdate {
            scheduled_track: ::std::option::Option::None,
            actual_track: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NyctStopTimeUpdate {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NyctStopTimeUpdate").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NyctStopTimeUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NyctStopTimeUpdate {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Extension fields
pub mod exts {

    pub const nyct_feed_header: ::protobuf::ext::ExtFieldOptional<super::super::gtfs_realtime::FeedHeader, super::NyctFeedHeader> = ::protobuf::ext::ExtFieldOptional::new(1001, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_MESSAGE);

    pub const nyct_trip_descriptor: ::protobuf::ext::ExtFieldOptional<super::super::gtfs_realtime::TripDescriptor, super::NyctTripDescriptor> = ::protobuf::ext::ExtFieldOptional::new(1001, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_MESSAGE);

    pub const nyct_stop_time_update: ::protobuf::ext::ExtFieldOptional<super::super::gtfs_realtime::trip_update::StopTimeUpdate, super::NyctStopTimeUpdate> = ::protobuf::ext::ExtFieldOptional::new(1001, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_MESSAGE);
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18gtfs-realtime-NYCT.proto\x12\x10transit_realtime\x1a\x13gtfs-realt\
    ime.proto\"~\n\x15TripReplacementPeriod\x12\x19\n\x08route_id\x18\x01\
    \x20\x01(\tR\x07routeId\x12J\n\x12replacement_period\x18\x02\x20\x01(\
    \x0b2\x1b.transit_realtime.TimeRangeR\x11replacementPeriod\"\xa1\x01\n\
    \x0eNyctFeedHeader\x12.\n\x13nyct_subway_version\x18\x01\x20\x02(\tR\x11\
    nyctSubwayVersion\x12_\n\x17trip_replacement_period\x18\x02\x20\x03(\x0b\
    2'.transit_realtime.TripReplacementPeriodR\x15tripReplacementPeriod\"\
    \xd5\x01\n\x12NyctTripDescriptor\x12\x19\n\x08train_id\x18\x01\x20\x01(\
    \tR\x07trainId\x12\x1f\n\x0bis_assigned\x18\x02\x20\x01(\x08R\nisAssigne\
    d\x12L\n\tdirection\x18\x03\x20\x01(\x0e2..transit_realtime.NyctTripDesc\
    riptor.DirectionR\tdirection\"5\n\tDirection\x12\t\n\x05NORTH\x10\x01\
    \x12\x08\n\x04EAST\x10\x02\x12\t\n\x05SOUTH\x10\x03\x12\x08\n\x04WEST\
    \x10\x04\"`\n\x12NyctStopTimeUpdate\x12'\n\x0fscheduled_track\x18\x01\
    \x20\x01(\tR\x0escheduledTrack\x12!\n\x0cactual_track\x18\x02\x20\x01(\t\
    R\x0bactualTrack:i\n\x10nyct_feed_header\x18\xe9\x07\x20\x01(\x0b2\x20.t\
    ransit_realtime.NyctFeedHeader\x12\x1c.transit_realtime.FeedHeaderR\x0en\
    yctFeedHeader:y\n\x14nyct_trip_descriptor\x18\xe9\x07\x20\x01(\x0b2$.tra\
    nsit_realtime.NyctTripDescriptor\x12\x20.transit_realtime.TripDescriptor\
    R\x12nyctTripDescriptor:\x85\x01\n\x15nyct_stop_time_update\x18\xe9\x07\
    \x20\x01(\x0b2$.transit_realtime.NyctStopTimeUpdate\x12+.transit_realtim\
    e.TripUpdate.StopTimeUpdateR\x12nyctStopTimeUpdateB\x1d\n\x1bcom.google.\
    transit.realtime\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::gtfs_realtime::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(4);
            messages.push(TripReplacementPeriod::generated_message_descriptor_data());
            messages.push(NyctFeedHeader::generated_message_descriptor_data());
            messages.push(NyctTripDescriptor::generated_message_descriptor_data());
            messages.push(NyctStopTimeUpdate::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(nyct_trip_descriptor::Direction::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
