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