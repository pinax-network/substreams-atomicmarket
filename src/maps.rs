use substreams::log;
use substreams::errors::Error;
use substreams_antelope::Block;

use crate::abi;
use crate::atomicmarket::*;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<AnyEvents, Error> {
    let mut response = vec![];

    for trx in block.all_transaction_traces() {
        // db_ops
        for db_op in &trx.db_ops {
            if db_op.table_name != "sales" {continue}
            if db_op.operation != 3 {continue}
            
            // Retrieve the trace associated with the db_op
            let mut associated_trace = &trx.action_traces[0];
            for trace in &trx.action_traces{
                if db_op.action_index == trace.execution_index {
                    associated_trace = trace;
                    break;
                }
            }
            // If the action that triggered the db_op was a cancelsale, skip it
            if associated_trace.action.as_ref().unwrap().name == "cancelsale" {continue}
            match abi::SalesS::try_from(db_op.old_data_json.as_str()) {
                Ok(data) => {
                    response.push(AnyEvent {
                        event: Some(any_event::Event::Assertsaleitem(
                            AssertSale {
                                trx_id: trx.id.clone(),
                                sale_id: data.sale_id,
                                asset_ids: data.asset_ids,
                                listing_price: data.listing_price,
                                collection_name: data.collection_name,
                            }))
                        });
                },
                Err(error) => {
                    log::debug!("old data not decoded: {}", error);
                    continue;
                }
            };
        }        

        // action traces
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();
            if action_trace.account != trace.receiver {continue}
            if action_trace.account != "atomicmarket" {continue}
            // Announce Sale event
            if action_trace.name == "announcesale" {
                match abi::Announcesale::try_from(action_trace.json_data.as_str()) {
                    Ok(data) => {
                        //if !["nft.hive", "market.nefty", "chainchampss"].contains(&data.maker_marketplace.as_str()) {continue}
                        response.push(AnyEvent{
                            event: Some(any_event::Event::Saleitem(
                                SaleEvent {
                                trx_id: trx.id.clone(),
                                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
                                event: action_trace.name.clone(),
                                seller: data.seller,
                                asset_ids: data.asset_ids,
                                listing_price: data.listing_price,
                                settlement_symbol: data.settlement_symbol,
                                maker_marketplace: data.maker_marketplace,
                                }
                            ))
                        });
                    },
                    Err(error) => {
                        log::debug!("Failed to decode atomicmarket::announcesale: {}", error);
                        continue;
                    }
                }
            }
            // Announce Auction event
            if action_trace.name == "announceauct" {
                match abi::Announceauct::try_from(action_trace.json_data.as_str()) {
                    Ok(data) => {
                        //if !["nft.hive", "market.nefty", "chainchampss"].contains(&data.maker_marketplace.as_str()) {continue}
                        response.push(AnyEvent{
                            event: Some(any_event::Event::Auctionitem(
                                AuctionEvent {
                                trx_id: trx.id.clone(),
                                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
                                event: action_trace.name.clone(),
                                seller: data.seller,
                                asset_ids: data.asset_ids,
                                starting_bid: data.starting_bid,
                                duration: data.duration,
                                maker_marketplace: data.maker_marketplace,
                                }
                            ))
                        });
                    },
                    Err(error) => {
                        log::debug!("Failed to decode atomicmarket::announceauct: {}", error);
                        continue;
                    }
                }
            }
            // Lognewbuyo event
            if action_trace.name == "lognewbuyo" {
                match abi::Lognewbuyo::try_from(action_trace.json_data.as_str()) {
                    Ok(data) => {
                        //if !["nft.hive", "market.nefty", "chainchampss"].contains(&data.maker_marketplace.as_str()) {continue}
                        response.push(AnyEvent{
                            event: Some(any_event::Event::Newbuyoitem(
                                NewBuyo {
                                trx_id: trx.id.clone(),
                                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
                                event: action_trace.name.clone(),
                                buyoffer_id: data.buyoffer_id,
                                buyer: data.buyer,
                                recipient: data.recipient,
                                price: data.price,
                                asset_ids: data.asset_ids,
                                memo: data.memo,
                                maker_marketplace: data.maker_marketplace,
                                collection_name: data.collection_name,
                                collection_fee: data.collection_fee,
                                }
                            ))
                        });
                    },
                    Err(error) => {
                        log::debug!("Failed to decode atomicmarket::lognewbuyo: {}", error);
                        continue;
                    }
                }
            }
            
            // Purchasesale event
            if action_trace.name == "purchasesale" {
                match abi::Purchasesale::try_from(action_trace.json_data.as_str()) {
                    Ok(data) => {
                        //if !["nft.hive", "market.nefty", "chainchampss"].contains(&data.taker_marketplace.as_str()) {continue}
                        response.push(AnyEvent{
                            event: Some(any_event::Event::Purchasesaleitem(
                                PurchaseSale {
                                trx_id: trx.id.clone(),
                                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
                                event: action_trace.name.clone(),
                                buyer: data.buyer,
                                sale_id: data.sale_id,
                                intended_delphi_median: data.intended_delphi_median,
                                taker_marketplace: data.taker_marketplace,
                                }
                            ))
                        });
                    },
                    Err(error) => {
                        log::debug!("Failed to decode atomicmarket::purchasesale: {}", error);
                        continue;
                    }
                }
            }
            
            // Lognewsale event
            if action_trace.name == "lognewsale" {
                match abi::Lognewsale::try_from(action_trace.json_data.as_str()) {
                    Ok(data) => {
                        //if !["nft.hive", "market.nefty", "chainchampss"].contains(&data.maker_marketplace.as_str()) {continue}
                        response.push(AnyEvent{
                            event: Some(any_event::Event::Lognewsaleitem(
                                LogNewSale {
                                trx_id: trx.id.clone(),
                                timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
                                event: action_trace.name.clone(),
                                sale_id: data.sale_id,
                                seller: data.seller,
                                asset_ids: data.asset_ids,
                                listing_price: data.listing_price,
                                settlement_symbol: data.settlement_symbol,
                                maker_marketplace: data.maker_marketplace,
                                collection_name: data.collection_name,
                                collection_fee: data.collection_fee,
                                }
                            ))
                        });
                    },
                    Err(error) => {
                        log::debug!("Failed to decode atomicmarket::lognewsale: {}", error);
                        continue;
                    }
                }
            }
        }
    }
    // Handle any event
    Ok(AnyEvents { items: response })
}