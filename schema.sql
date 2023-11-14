CREATE TABLE Sales
(
    sale_id     UInt64,
    trx_id      String,
    seller      String,
    asset_ids   Array(UInt64),
    offer_id    Int64,
    listing_price_amount  Int64,
    listing_price_precision  UInt8,
    listing_price_symcode    String,
    settlement_symbol_precision   UInt8,
    settlement_symbol_code String,
    maker_marketplace String,
    collection_name String,
    collection_fee  Float64,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (sale_id)
ORDER BY (sale_id, collection_name, listing_price_symcode, trx_id);

CREATE TABLE AnnounceSales
(
    trx_id      String,
    seller      String,
    asset_ids   Array(UInt64),
    listing_price_amount  Int64,
    listing_price_precision  UInt8,
    listing_price_symcode    String,
    settlement_symbol_precision   UInt8,
    settlement_symbol_code String,
    maker_marketplace String,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (trx_id)
ORDER BY (trx_id, listing_price_symcode, maker_marketplace, seller);

CREATE TABLE AnnounceAuctions
(
    trx_id      String,
    seller      String,
    asset_ids   Array(UInt64),
    starting_bid_amount  Int64,
    starting_bid_precision  UInt8,
    starting_bid_symcode    String,
    duration    UInt32,
    maker_marketplace String,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (trx_id)
ORDER BY (trx_id, starting_bid_symcode, maker_marketplace, seller);

CREATE TABLE NewBuyOffers
(
    buyoffer_id UInt64,
    trx_id      String,
    buyer       String,
    recipient   String,
    price_amount   Int64,
    price_precision   UInt8,
    price_symcode   String,
    asset_ids   Array(UInt64),
    memo    String,
    maker_marketplace String,
    collection_name String,
    collection_fee  Float64,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (buyoffer_id)
ORDER BY (buyoffer_id, collection_name, price_symcode, trx_id);

CREATE TABLE NewSales
(
    sale_id     UInt64,
    trx_id      String,
    seller       String,
    asset_ids   Array(UInt64),
    listing_price_amount  Int64,
    listing_price_precision  UInt8,
    listing_price_symcode    String,
    settlement_symbol_precision   UInt8,
    settlement_symbol_code String,
    maker_marketplace String,
    collection_name String,
    collection_fee  Float64,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (sale_id)
ORDER BY (sale_id, collection_name, listing_price_symcode, trx_id);

CREATE TABLE PurchaseSales
(
    sale_id     UInt64,
    trx_id      String,
    buyer       String,
    intended_delphi_median   UInt64,
    taker_marketplace String,
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (sale_id)
ORDER BY (sale_id, taker_marketplace, trx_id);