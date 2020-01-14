#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloEntity {
    #[prost(string, tag = "1")]
    pub entity_message: std::string::String,
    #[prost(double, tag = "2")]
    pub entity_double: f64,
    #[prost(int32, tag = "3")]
    pub entity_int32: i32,
    #[prost(int64, tag = "4")]
    pub entity_int64: i64,
    #[prost(uint32, tag = "5")]
    pub entity_uint32: u32,
    #[prost(uint64, tag = "6")]
    pub entity_uint64: u64,
    #[prost(sint32, tag = "7")]
    pub entity_sint32: i32,
    #[prost(sint64, tag = "8")]
    pub entity_sint64: i64,
    #[prost(fixed32, tag = "9")]
    pub entity_fixed32: u32,
    #[prost(fixed64, tag = "10")]
    pub entity_fixed64: u64,
    #[prost(sfixed32, tag = "11")]
    pub entity_sfixed32: i32,
    #[prost(sfixed64, tag = "12")]
    pub entity_sfixed64: i64,
    #[prost(bool, tag = "13")]
    pub entity_bool: bool,
    #[prost(bytes, tag = "14")]
    pub entity_bytes: std::vec::Vec<u8>,
}
