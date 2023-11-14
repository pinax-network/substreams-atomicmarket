// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnyEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<AnyEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnyEvent {
    #[prost(oneof="any_event::Event", tags="1, 2, 3, 4, 5, 6")]
    pub event: ::core::option::Option<any_event::Event>,
}
/// Nested message and enum types in `AnyEvent`.
pub mod any_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        AnnounceSale(super::AnnounceSale),
        #[prost(message, tag="2")]
        AnnonceAuction(super::AnnounceAuction),
        #[prost(message, tag="3")]
        NewBuyOffer(super::NewBuyOffer),
        #[prost(message, tag="4")]
        PurchaseSale(super::PurchaseSale),
        #[prost(message, tag="5")]
        NewSale(super::NewSale),
        #[prost(message, tag="6")]
        AssertSale(super::AssertSale),
    }
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
    #[prost(string, tag="5")]
    pub seller: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag="6")]
    pub asset_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(int64, tag="7")]
    pub offer_id: i64,
    #[prost(string, tag="8")]
    pub listing_price: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub settlement_symbol: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub maker_marketplace: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub collection_name: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub collection_fee: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnounceSale {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    /// payload
    #[prost(string, tag="5")]
    pub seller: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag="6")]
    pub asset_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag="7")]
    pub listing_price: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub settlement_symbol: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub maker_marketplace: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnounceAuction {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    /// payload
    #[prost(string, tag="5")]
    pub seller: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag="6")]
    pub asset_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag="7")]
    pub starting_bid: ::prost::alloc::string::String,
    #[prost(uint32, tag="8")]
    pub duration: u32,
    #[prost(string, tag="9")]
    pub maker_marketplace: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBuyOffer {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    /// payload
    #[prost(uint64, tag="4")]
    pub buyoffer_id: u64,
    #[prost(string, tag="5")]
    pub buyer: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub price: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag="8")]
    pub asset_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag="9")]
    pub memo: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub maker_marketplace: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub collection_name: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub collection_fee: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSale {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    /// payload
    #[prost(uint64, tag="4")]
    pub sale_id: u64,
    #[prost(string, tag="5")]
    pub seller: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag="6")]
    pub asset_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag="7")]
    pub listing_price: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub settlement_symbol: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub maker_marketplace: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub collection_name: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub collection_fee: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurchaseSale {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    /// payload
    #[prost(string, tag="4")]
    pub buyer: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub sale_id: u64,
    #[prost(uint64, tag="6")]
    pub intended_delphi_median: u64,
    #[prost(string, tag="7")]
    pub taker_marketplace: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
