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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelSaleEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<CancelSale>,
}
/// Cancelsale
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelSale {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    /// payload
    #[prost(uint64, tag="3")]
    pub sale_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptbuyoEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Acceptbuyo>,
}
/// Acceptbuyo
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Acceptbuyo {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    /// payload
    #[prost(uint64, tag="3")]
    pub buyoffer_id: u64,
    #[prost(uint64, repeated, tag="4")]
    pub asset_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag="5")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub collection_name: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
