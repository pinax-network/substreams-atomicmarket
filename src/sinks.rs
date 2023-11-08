use antelope::{Asset};
use substreams::{errors::Error }; //, pb::substreams::Clock};
use std::collections::HashMap;
use substreams_sink_prometheus::{PrometheusOperations, Counter};

use substreams_entity_change::{tables::Tables, pb::entity::EntityChanges};
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};

use crate::atomicmarket::*;

#[substreams::handlers::map]
fn prom_out(anyevents: AnyEvents) -> Result<PrometheusOperations, Error> {
    let mut prom_ops: PrometheusOperations = Default::default();

    let mut sales_by_collection: HashMap<String, (f64, f64, f64)> = HashMap::new();

    // for each collection name, count the number of sales and the total price
    for anyevent in anyevents.items{
        match anyevent.event {
            Some(any_event::Event::Assertsaleitem(item)) => {
                let (sales_count, total_eos_price, total_usd_price) = sales_by_collection.entry(item.collection_name.clone()).or_insert((0.0, 0.0, 0.0));
                *sales_count += 1.0;
                
                // counts USD prices
                if item.listing_price.contains(" USD") {
                    let usd_price = item.listing_price.replace(" USD", "").parse::<f64>().unwrap();
                    *total_usd_price += usd_price;
                }
                // counts EOS prices
                if item.listing_price.contains(" EOS") {
                    let eos_price = item.listing_price.replace(" EOS", "").parse::<f64>().unwrap();
                    *total_eos_price += eos_price;
                }
            },
            _ => {continue}
        }
    }

    for (collection_name, (sales_count, total_eos_price, total_usd_price)) in &sales_by_collection {
        // create counters wtih collection name as a label
        let mut sales_counter = Counter::from("number_sales").with(HashMap::from([("collection_name".to_string(), collection_name.clone())]));
        let mut eos_volume_counter = Counter::from("volume").with(HashMap::from([("collection_name".to_string(), collection_name.clone()), ("currency".to_string(), "EOS".to_string())]));
        let mut usd_volume_counter = Counter::from("volume").with(HashMap::from([("collection_name".to_string(), collection_name.clone()), ("currency".to_string(), "USD".to_string())]));

        // add the values to each counter
        prom_ops.push(sales_counter.add(*sales_count));
        prom_ops.push(eos_volume_counter.add(*total_eos_price));
        prom_ops.push(usd_volume_counter.add(*total_usd_price));
    }

    Ok(prom_ops)
}

#[substreams::handlers::map]
fn graph_out(anyevents: AnyEvents) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    for anyevent in anyevents.items {
        match anyevent.event {
            Some(any_event::Event::Assertsaleitem(event)) => {
                let sale_id = &event.sale_id.to_string();
                let asset = Asset::from(event.listing_price.as_str());
        
                // convert Vec<u64> to Vec<String>
                let asset_ids: Vec<String> = event.asset_ids.iter().map(|x| x.to_string()).collect();
                tables
                    .create_row("Sales", sale_id)
                    .set_bigint("sale_id", sale_id)
                    .set("trx_id", &event.trx_id)
                    .set("asset_ids", asset_ids)
                    .set("listing_price_amount", asset.amount)
                    .set("listing_price_precision", asset.symbol.precision())
                    .set("listing_price_symcode", &asset.symbol.code().to_string())
                    .set("collection_name", &event.collection_name);
            },
            _ => {continue}
        }
    }
    Ok(tables.to_entity_changes())
}

#[substreams::handlers::map]
pub fn db_out(anyevents: AnyEvents) -> Result<DatabaseChanges, Error> {
    let mut database_changes: DatabaseChanges = Default::default();
  
    for anyevent in anyevents.items {
        match anyevent.event {
            Some(any_event::Event::Assertsaleitem(event)) => {
                let sale_id = &event.sale_id.to_string();
                let asset = Asset::from(event.listing_price.as_str());
                
                // convert Vec<u64> to String
                let asset_ids: String = event.asset_ids.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
                // required by the database_change crate (might have problems if the asset_id is too big)
                //let asset_ids: Vec<u8> = event.asset_ids.iter().map(|x| x.to_string().parse::<u8>().unwrap()).collect();

                database_changes
                .push_change("Sales", sale_id.clone(), 0, Operation::Create)
                .change("sale_id", (None, sale_id))
                .change("trx_id", (None, event.trx_id))

                .change("asset_ids", (None, asset_ids))

                .change("listing_price_amount", (None, asset.amount))
                .change("listing_price_precision", (None, asset.symbol.precision()))
                .change("listing_price_symcode", (None, asset.symbol.code().to_string()))

                .change("collection_name", (None, event.collection_name));
            },
            _ => {continue}
        }
    }
    Ok(database_changes)
}