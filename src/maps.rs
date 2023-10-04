use substreams::log;
use substreams::errors::Error;
use substreams_antelope::Block;

use crate::abi;
use crate::atomicmarketsales::*;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<AssertSaleEvents, Error> {
    let mut response = vec![];

    for trx in block.all_transaction_traces() {
        // db_ops
        for db_op in &trx.db_ops {
            if db_op.table_name != "sales" {continue}
            if db_op.operation != 3 {continue}
            
            match abi::SalesS::try_from(db_op.old_data_json.as_str()) {
                Ok(data) => {
                    response.push(AssertSale {
                        trx_id: trx.id.clone(),
                        timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
                        sale_id: data.sale_id,
                        asset_ids: data.asset_ids,
                        listing_price: data.listing_price,
                        collection_name: data.collection_name,
                    });
                },
                Err(_) => {
                    // print data to console
                    log::info!("{:?}", db_op.old_data_json);
                    panic!("Failed to decode atomicmarket::SalesS");
                }
            };
        }
    }
    /*let mut db_ops_sales: HashMap<u64, String> = HashMap::new();
    for trx in block.all_transaction_traces() {
        // db_ops
        for db_op in &trx.db_ops {
            if db_op.table_name != "sales" {continue}
            if db_op.operation != 3 {continue}
            
            match abi::SalesS::try_from(db_op.old_data_json.as_str()) {
                Ok(data) => db_ops_sales.insert(
                    data.sale_id,
                    data.collection_name,
                ),
                Err(_) => {
                    // print data to console
                    log::info!("{:?}", db_op.old_data_json);
                    panic!("Failed to decode atomicmarket::sales");
                }
            };
        }

        // action traces
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();
            if action_trace.account != trace.receiver {continue}
            if action_trace.account != "atomicmarket" {continue}

            // Assertsale event
            if action_trace.name == "assertsale" {
                match abi::Assertsale::try_from(action_trace.json_data.as_str()) {
                    Ok(data) => {
                        //if data.collection_name != "pomelo" {continue}
                        let collection_name;
                        if let Some(value) = db_ops_sales.get(&data.sale_id) {
                            collection_name = value.clone();
                        } else {
                            collection_name = "".to_string();
                        }
                        response.push(AssertSale {
                            trx_id: trx.id.clone(),
                            timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
                            event: action_trace.name.clone(),
                            sale_id: data.sale_id,
                            asset_ids_to_assert: data.asset_ids_to_assert,
                            listing_price_to_assert: data.listing_price_to_assert,
                            settlement_symbol_to_assert: data.settlement_symbol_to_assert,
                            collection_name: collection_name,
                        });
                    },
                    Err(_) => {
                        // print data to console
                        log::info!("{:?}", action_trace.json_data);
                        panic!("Failed to decode atomicmarket::assertsale");
                    }
                }
            }
        }
    }*/
    Ok(AssertSaleEvents { items: response })
}

#[substreams::handlers::map]
fn map_cancelled(block: Block) -> Result<CancelSaleEvents, Error> {
    let mut response = vec![];

    for trx in block.all_transaction_traces() {
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();
            if action_trace.account != trace.receiver {continue}
            if action_trace.account != "atomicmarket" {continue}

            // Cancelled sale
            if action_trace.name == "cancelsale" {
                match abi::Cancelsale::try_from(action_trace.json_data.as_str()) {
                    Ok(data) => {
                        response.push(CancelSale {
                            trx_id: trx.id.clone(),
                            timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
                            sale_id: data.sale_id,
                        });
                    },
                    Err(_) => {
                        // print data to console
                        log::info!("{:?}", action_trace.json_data);
                        panic!("Failed to decode atomicmarket::cancelsale");
                    }
                }
            }
        }
    }
    Ok(CancelSaleEvents { items: response })
}

#[substreams::handlers::map]
// acceptbuyo
fn map_acceptbuyo(block: Block) -> Result<AcceptbuyoEvents, Error> {
    let mut response = vec![];

    for trx in block.all_transaction_traces() {
        // db_ops
        for db_op in &trx.db_ops {
            if db_op.table_name != "buyoffers" {continue}
            if db_op.operation != 3 {continue}
            
            match abi::BuyoffersS::try_from(db_op.old_data_json.as_str()) {
                Ok(data) => {
                    response.push(Acceptbuyo {
                        trx_id: trx.id.clone(),
                        timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
                        buyoffer_id: data.buyoffer_id,
                        asset_ids: data.asset_ids,
                        price: data.price,
                        collection_name: data.collection_name,
                    });
                },
                Err(_) => {
                    // print data to console
                    log::info!("{:?}", db_op.old_data_json);
                    panic!("Failed to decode atomicmarket::buyoffer");
                }
            };
        } 
        /*
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();
            if action_trace.account != trace.receiver {continue}
            if action_trace.account != "atomicmarket" {continue}

            // Acceptbuyo
            if action_trace.name == "acceptbuyo" {
                match abi::Acceptbuyo::try_from(action_trace.json_data.as_str()) {
                    Ok(data) => {
                        response.push(AcceptBuyo {
                            trx_id: trx.id.clone(),
                            timestamp: block.header.as_ref().unwrap().timestamp.as_ref().unwrap().to_string(),
                            buyoffer_id: data.buyoffer_id,
                            expected_asset_ids: data.expected_asset_ids,
                            expected_price: data.expected_price,
                            taker_marketplace: data.taker_marketplace,
                            collection_name: data.collection_name,
                        });
                    },
                    Err(_) => {
                        // print data to console
                        log::info!("{:?}", action_trace.json_data);
                        panic!("Failed to decode atomicmarket::acceptbuyo");
                    }
                }
            }
        }
        */
    }
    Ok(AcceptbuyoEvents { items: response })
}

/*
 // Purchasesale event
 if action_trace.name == "purchasesale" {
    match abi::Purchasesale::try_from(action_trace.json_data.as_str()) {
        Ok(data) => {
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
        Err(_) => {
            // print data to console
            log::info!("{:?}", action_trace.json_data);
            panic!("Failed to decode atomicmarket::purchasesale");
        }
    }
}

// Lognewsale event
if action_trace.name == "lognewsale" {
    match abi::Lognewsale::try_from(action_trace.json_data.as_str()) {
        Ok(data) => {
            //if data.collection_name != "pomelo" {continue}
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
        Err(_) => {
            // print data to console
            log::info!("{:?}", action_trace.json_data);
            panic!("Failed to decode atomicmarket::lognewsale");
        }
    }
}*/