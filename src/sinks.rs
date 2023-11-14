use antelope::{Asset, Symbol};
use substreams::errors::Error;
use substreams_entity_change::{tables::Tables, pb::entity::EntityChanges};

use crate::atomicmarket::*;

#[substreams::handlers::map]
fn graph_out(anyevents: AnyEvents) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    for anyevent in anyevents.items {
        match anyevent.event {
            Some(any_event::Event::AssertSale(event)) => {
                let sale_id = &event.sale_id.to_string();
                let asset = Asset::from(event.listing_price.as_str());
                let settlement_symbol = Symbol::from(event.settlement_symbol.as_str());

                // convert Vec<u64> to Vec<String>
                let asset_ids: Vec<String> = event.asset_ids.iter().map(|x| x.to_string()).collect();
                tables
                    .create_row("Sales", sale_id)
                    .set_bigint("sale_id", sale_id)
                    .set("trx_id", &event.trx_id)
                    .set("seller", &event.seller)
                    .set("asset_ids", asset_ids)
                    .set("offer_id", event.offer_id)
                    .set("listing_price_amount", asset.amount)
                    .set("listing_price_precision", asset.symbol.precision())
                    .set("listing_price_symcode", &asset.symbol.code().to_string())
                    .set("settlement_symbol_precision", settlement_symbol.precision())
                    .set("settlement_symbol_code", &settlement_symbol.code().to_string())
                    .set("maker_marketplace", &event.maker_marketplace)
                    .set("collection_name", &event.collection_name)
                    .set("collection_fee", &event.collection_fee);
            },
            Some(any_event::Event::AnnounceSale(event)) => {
                let trx_id = &event.trx_id;
                let asset = Asset::from(event.listing_price.as_str());
                let settlement_symbol = Symbol::from(event.settlement_symbol.as_str());

                // convert Vec<u64> to Vec<String>
                let asset_ids: Vec<String> = event.asset_ids.iter().map(|x| x.to_string()).collect();
                tables
                    .create_row("AnnounceSales", trx_id)
                    .set("trx_id", &event.trx_id)
                    .set("asset_ids", asset_ids)
                    .set("seller", &event.seller)
                    .set("listing_price_amount", asset.amount)
                    .set("listing_price_precision", asset.symbol.precision())
                    .set("listing_price_symcode", &asset.symbol.code().to_string())
                    .set("settlement_symbol_precision", settlement_symbol.precision())
                    .set("settlement_symbol_code", &settlement_symbol.code().to_string())
                    .set("maker_marketplace", &event.maker_marketplace);
            },
            Some(any_event::Event::AnnonceAuction(event)) => {
                let trx_id = &event.trx_id;
                let asset = Asset::from(event.starting_bid.as_str());

                // convert Vec<u64> to Vec<String>
                let asset_ids: Vec<String> = event.asset_ids.iter().map(|x| x.to_string()).collect();
                tables
                    .create_row("AnnounceAuctions", trx_id)
                    .set("trx_id", &event.trx_id)
                    .set("seller", &event.seller)
                    .set("asset_ids", asset_ids)
                    .set("starting_bid_amount", asset.amount)
                    .set("starting_bid_precision", asset.symbol.precision())
                    .set("starting_bid_symcode", &asset.symbol.code().to_string())
                    .set("duration", event.duration)
                    .set("maker_marketplace", &event.maker_marketplace);
            },
            Some(any_event::Event::NewBuyOffer(event)) => {
                let buyoffer_id = &event.buyoffer_id.to_string();
                let asset = Asset::from(event.price.as_str());

                // convert Vec<u64> to Vec<String>
                let asset_ids: Vec<String> = event.asset_ids.iter().map(|x| x.to_string()).collect();
                tables
                    .create_row("NewBuyOffers", buyoffer_id)
                    .set("buyoffer_id", buyoffer_id)
                    .set("trx_id", &event.trx_id)
                    .set("buyer", &event.buyer)
                    .set("recipient", &event.recipient)
                    .set("price_amount", asset.amount)
                    .set("price_precision", asset.symbol.precision())
                    .set("price_symcode", &asset.symbol.code().to_string())
                    .set("asset_ids", asset_ids)
                    .set("memo", &event.memo)
                    .set("maker_marketplace", &event.maker_marketplace)
                    .set("collection_name", &event.collection_name)
                    .set("collection_fee", &event.collection_fee);
            },
            Some(any_event::Event::NewSale(event)) => {
                let sale_id = &event.sale_id.to_string();
                let asset = Asset::from(event.listing_price.as_str());
                let settlement_symbol = Symbol::from(event.settlement_symbol.as_str());

                // convert Vec<u64> to Vec<String>
                let asset_ids: Vec<String> = event.asset_ids.iter().map(|x| x.to_string()).collect();
                tables
                    .create_row("NewSales", sale_id)
                    .set("sale_id", sale_id)
                    .set("trx_id", &event.trx_id)
                    .set("seller", &event.seller)
                    .set("asset_ids", asset_ids)
                    .set("listing_price_amount", asset.amount)
                    .set("listing_price_precision", asset.symbol.precision())
                    .set("listing_price_symcode", &asset.symbol.code().to_string())
                    .set("settlement_symbol_precision", settlement_symbol.precision())
                    .set("settlement_symbol_code", &settlement_symbol.code().to_string())
                    .set("maker_marketplace", &event.maker_marketplace)
                    .set("collection_name", &event.collection_name)
                    .set("collection_fee", &event.collection_fee);
            },
            Some(any_event::Event::PurchaseSale(event)) => {
                let sale_id = &event.sale_id.to_string();

                tables
                    .create_row("PurchaseSales", sale_id)
                    .set("sale_id", sale_id)
                    .set("trx_id", &event.trx_id)
                    .set("buyer", &event.buyer)
                    .set("intended_delphi_median", event.intended_delphi_median)
                    .set("taker_marketplace", &event.taker_marketplace);
            },
            _ => {continue}
        }
    }
    Ok(tables.to_entity_changes())
}
