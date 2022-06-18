// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tokens {
    #[prost(message, repeated, tag = "1")]
    pub tokens: ::prost::alloc::vec::Vec<Token>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub decimals: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pairs {
    #[prost(message, repeated, tag = "1")]
    pub pairs: ::prost::alloc::vec::Vec<Pair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pair {
    #[prost(uint64, tag = "1")]
    pub ordinal: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub token0: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub token1: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reserves {
    #[prost(message, repeated, tag = "1")]
    pub reserves: ::prost::alloc::vec::Vec<Reserve>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reserve {
    #[prost(uint64, tag = "1")]
    pub ordinal: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub pair: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub reserve0: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub reserve1: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub token0_price: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub token1_price: ::prost::alloc::string::String,
}
/// Encoded file descriptor set for the `uniswap.types.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xed, 0x0d, 0x0a, 0x0d, 0x75, 0x6e, 0x69, 0x73, 0x77, 0x61, 0x70, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x10, 0x75, 0x6e, 0x69, 0x73, 0x77, 0x61, 0x70, 0x2e, 0x74, 0x79, 0x70, 0x65,
    0x73, 0x2e, 0x76, 0x31, 0x22, 0x39, 0x0a, 0x06, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x73, 0x12, 0x2f,
    0x0a, 0x06, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x17,
    0x2e, 0x75, 0x6e, 0x69, 0x73, 0x77, 0x61, 0x70, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x76,
    0x31, 0x2e, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x52, 0x06, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x73, 0x22,
    0x69, 0x0a, 0x05, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65,
    0x73, 0x73, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x79, 0x6d, 0x62, 0x6f, 0x6c, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x06, 0x73, 0x79, 0x6d, 0x62, 0x6f, 0x6c, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1a,
    0x0a, 0x08, 0x64, 0x65, 0x63, 0x69, 0x6d, 0x61, 0x6c, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x08, 0x64, 0x65, 0x63, 0x69, 0x6d, 0x61, 0x6c, 0x73, 0x22, 0x35, 0x0a, 0x05, 0x50, 0x61,
    0x69, 0x72, 0x73, 0x12, 0x2c, 0x0a, 0x05, 0x70, 0x61, 0x69, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x16, 0x2e, 0x75, 0x6e, 0x69, 0x73, 0x77, 0x61, 0x70, 0x2e, 0x74, 0x79, 0x70,
    0x65, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x50, 0x61, 0x69, 0x72, 0x52, 0x05, 0x70, 0x61, 0x69, 0x72,
    0x73, 0x22, 0x6a, 0x0a, 0x04, 0x50, 0x61, 0x69, 0x72, 0x12, 0x18, 0x0a, 0x07, 0x6f, 0x72, 0x64,
    0x69, 0x6e, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x6f, 0x72, 0x64, 0x69,
    0x6e, 0x61, 0x6c, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x16, 0x0a,
    0x06, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x30, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x74,
    0x6f, 0x6b, 0x65, 0x6e, 0x30, 0x12, 0x16, 0x0a, 0x06, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x31, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x31, 0x22, 0x41, 0x0a,
    0x08, 0x52, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x73, 0x12, 0x35, 0x0a, 0x08, 0x72, 0x65, 0x73,
    0x65, 0x72, 0x76, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x75, 0x6e,
    0x69, 0x73, 0x77, 0x61, 0x70, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x52,
    0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x52, 0x08, 0x72, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x73,
    0x22, 0xb5, 0x01, 0x0a, 0x07, 0x52, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x12, 0x18, 0x0a, 0x07,
    0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x6f,
    0x72, 0x64, 0x69, 0x6e, 0x61, 0x6c, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x69, 0x72, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x70, 0x61, 0x69, 0x72, 0x12, 0x1a, 0x0a, 0x08, 0x72, 0x65,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x30, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x72, 0x65,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x30, 0x12, 0x1a, 0x0a, 0x08, 0x72, 0x65, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x31, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x72, 0x65, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x31, 0x12, 0x21, 0x0a, 0x0c, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x30, 0x5f, 0x70, 0x72, 0x69,
    0x63, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x30,
    0x50, 0x72, 0x69, 0x63, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x31, 0x5f,
    0x70, 0x72, 0x69, 0x63, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x74, 0x6f, 0x6b,
    0x65, 0x6e, 0x31, 0x50, 0x72, 0x69, 0x63, 0x65, 0x4a, 0xfd, 0x08, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x25, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04,
    0x00, 0x06, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x0e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x05, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x05, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x05, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x05, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x08, 0x00, 0x0d, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x08, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x09, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x09, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x09, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x02, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0a, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x02, 0x12, 0x03, 0x0b, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x0b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x0b, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x10,
    0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x02, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0c, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0c, 0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x0c, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04,
    0x0f, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x0d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x10, 0x02, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x10, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x10, 0x10, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x10, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x13, 0x00, 0x18,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x13, 0x08, 0x0c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x14, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x14, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x14, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x14, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x15, 0x02,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x15, 0x02, 0x07, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x15, 0x08, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x15, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x02, 0x12, 0x03, 0x16, 0x02, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x16, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x16, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x16,
    0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x17, 0x02, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x17, 0x02, 0x07, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x17, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x17, 0x11, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x1a, 0x00, 0x1c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1a, 0x08,
    0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x02, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1b, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x1b, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x1e, 0x00,
    0x25, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x0f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x1f, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1f, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x1f, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x1f, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x20,
    0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x20, 0x02, 0x07,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x20, 0x08, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x20, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x21, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x21, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x21, 0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x21, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x22, 0x02, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x05, 0x12, 0x03, 0x22, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x22, 0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x22, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x04, 0x12, 0x03, 0x23, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x23, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x23, 0x09, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x03, 0x12, 0x03, 0x23, 0x18,
    0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x05, 0x12, 0x03, 0x24, 0x02, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x05, 0x12, 0x03, 0x24, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x05, 0x01, 0x12, 0x03, 0x24, 0x09, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x05, 0x03, 0x12, 0x03, 0x24, 0x18, 0x19, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)