CREATE TABLE Sales
(
    sale_id     UInt64,
    trx_id      FixedString(64),
    asset_ids   Array(UInt64),
    listing_price_amount  Int64,
    listing_price_precision  UInt8,
    listing_price_symcode    FixedString(7),
    collection_name FixedString(12),
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (sale_id)
ORDER BY (sale_id, collection_name, listing_price_symcode, trx_id);
