// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssertSaleEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<AssertSale>,
}
/// Assertsale
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssertSale {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    /// payload
    #[prost(uint64, tag="4")]
    pub sale_id: u64,
    #[prost(uint64, repeated, tag="5")]
    pub asset_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag="6")]
    pub listing_price: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub collection_name: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
