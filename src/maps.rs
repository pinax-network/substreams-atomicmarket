use substreams::log;
use substreams::errors::Error;
use substreams_antelope::Block;

use crate::abi;
use crate::atomicmarketsales::*;

#[substreams::handlers::map]
fn map_sales(block: Block) -> Result<AssertSaleEvents, Error> {
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
    }
    Ok(AcceptbuyoEvents { items: response })
}