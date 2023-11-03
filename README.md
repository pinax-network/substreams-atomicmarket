# Atomicmarket sales powered by **Substreams**

[![Build Status](https://github.com/pinax-network/substreams-atomicmarket-sales/actions/workflows/test.yml/badge.svg)](https://github.com/pinax-network/substreams-atomicmarket-sales/actions/workflows/test.yml)
![Version](https://img.shields.io/github/v/release/pinax-network/substreams-atomicmarket-sales)
![License](https://img.shields.io/github/license/pinax-network/substreams-atomicmarket-sales)

> Sale ID, Transaction ID, Asset IDs, Listing price & Collection name

## Quick Start

```
gh repo clone pinax-network/substreams-atomicmarket-sales
cd substreams-atomicmarket-sales
make
make gui        # runs the map_sales module for a block
```

### Mermaid graph

```mermaid
graph TD;
  map_sales[map: map_sales];
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_sales;
  prom_out[map: prom_out];
  map_sales --> prom_out;
  graph_out[map: graph_out];
  map_sales --> graph_out;
  db_out[map: db_out];
  map_sales --> db_out;

```
## Map Outputs

### `graph_out`

```json
{
  "entityChanges": [
      {
        "entity": "Sales",
        "id": "2321191",
        "operation": "OPERATION_CREATE",
        "fields": [
          {
            "name": "sale_id",
            "newValue": {
              "bigint": "2321191"
            }
          },
          {
            "name": "listing_price_symcode",
            "newValue": {
              "string": "USD"
            }
          },
          {
            "name": "listing_price_precision",
            "newValue": {
              "bigint": "2"
            }
          },
          {
            "name": "collection_name",
            "newValue": {
              "string": "chessunivers"
            }
          },
          {
            "name": "trx_id",
            "newValue": {
              "string": "b70bfe7ddea07a0be32991684fff17d6d74825e905b785e43be236845779f318"
            }
          },
          {
            "name": "asset_ids",
            "newValue": {
              "array": {
                "value": [
                  {
                    "string": "2199025056631"
                  }
                ]
              }
            }
          },
          {
            "name": "listing_price_amount",
            "newValue": {
              "bigint": "4"
            }
          },
          {
            "name": "listing_price_value",
            "newValue": {
              "string": "0.04"
            }
          }
        ]
      },
    ...
  ]
}
  
```

### Modules
```yaml
Package name: atomicmarketsales
Version: v0.2.3
Modules:
----
Name: map_sales
Initial block: 0
Kind: map
Output Type: proto:antelope.atomicmarketsales.v1.AssertSaleEvents
Hash: b9f0385d22cedd6d251e24cdb1345141bc71eaa8

Name: prom_out
Initial block: 0
Kind: map
Output Type: proto:pinax.substreams.sink.prometheus.v1.PrometheusOperations
Hash: 5060bc9fa0021d368221ed77468d1b835ef4ea39

Name: graph_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.entity.v1.EntityChanges
Hash: b804edd28e29f236ef6ea6b8f94f82813688e805

Name: db_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.database.v1.DatabaseChanges
Hash: a4daac2a805d33ddf175a2e0ec6f3048bdd472a7
```