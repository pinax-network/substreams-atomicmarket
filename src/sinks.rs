use substreams::errors::Error;
use std::collections::HashMap;
use substreams_sink_prometheus::{PrometheusOperations, Counter};

use crate::atomicmarketsales::*;

#[substreams::handlers::map]
fn prom_out(events: AssertSaleEvents) -> Result<PrometheusOperations, Error> {

    let mut prom_ops: PrometheusOperations = Default::default();

    let mut sales_by_collection: HashMap<String, (f64, f64, f64)> = HashMap::new();
    // for each collection name, count the number of sales and the total price
    for item in &events.items {
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