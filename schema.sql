create table sales
(
    id          text not null constraint sales_pk primary key,
    sale_id     text,
    trx_id      text,
    timestamp   timestamp,
    asset_ids   text,
    listing_price  float,
    collection_name text,
);