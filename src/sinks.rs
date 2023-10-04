use substreams::errors::Error;
use std::collections::HashMap;
use substreams_sink_prometheus::{PrometheusOperations, Counter};

use crate::atomicmarketsales::*;

#[substreams::handlers::map]
fn prom_out(events: AssertSaleEvents) -> Result<PrometheusOperations, Error> {

    let mut prom_ops: PrometheusOperations = Default::default();

    let mut sales_by_collection: HashMap<String, (f64, f64)> = HashMap::new();
    // for each collection name, count the number of sales and the total price
    for item in &events.items {
        let (sales_count, total_price) = sales_by_collection.entry(item.collection_name.clone()).or_insert((0.0, 0.0));
        *sales_count += 1.0;
        
        // skips USD transactions as I need to convert them to EOS but not sure how yet
        if item.listing_price.contains(" USD") {
            continue;
        }
        // filter the numerous part of the string and convert it to float
        let price = item.listing_price.replace(" EOS", "").parse::<f64>().unwrap();
        *total_price += price;
    }

    for (collection_name, (sales_count, total_price)) in &sales_by_collection {
        let mut sales_counter = Counter::from("total_sales").with(HashMap::from([("collection_name".to_string(), collection_name.clone())]));
        let mut volume_counter = Counter::from("total_volume").with(HashMap::from([("collection_name".to_string(), collection_name.clone())]));
        prom_ops.push(sales_counter.add(*sales_count));
        prom_ops.push(volume_counter.add(*total_price));
    }

    Ok(prom_ops)
}
