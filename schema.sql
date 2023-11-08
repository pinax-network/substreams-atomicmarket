CREATE TABLE Sales
(
    sale_id     UInt64,
    trx_id      String,
    asset_ids   Array(UInt64),
    listing_price_amount  Int64,
    listing_price_precision  UInt8,
    listing_price_symcode    String,
    collection_name String,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (sale_id)
ORDER BY (sale_id, collection_name, listing_price_symcode, trx_id);
