# `Atomicmarket` powered by **Substreams**

[![Build Status](https://github.com/pinax-network/substreams-atomicmarket/actions/workflows/test.yml/badge.svg)](https://github.com/pinax-network/substreams-atomicmarket/actions/workflows/test.yml)
![Version](https://img.shields.io/github/v/release/pinax-network/substreams-atomicmarket)
![License](https://img.shields.io/github/license/pinax-network/substreams-atomicmarket)

> Sale events: Sale ID, Transaction ID, Asset IDs, Listing price & Collection name

## Quick Start

```
make
make gui
```

### Mermaid graph

```mermaid
graph TD;
  map_events[map: map_events];
  sf.antelope.type.v1.Block[source: sf.antelope.type.v1.Block] --> map_events;
  graph_out[map: graph_out];
  map_events --> graph_out;
```
## Map Outputs

### `map_events`

```json
{
  "items": [
    {
      "assertSale": {
        "trxId": "b70bfe7ddea07a0be32991684fff17d6d74825e905b785e43be236845779f318",
        "saleId": "2319916",
        "assetIds": ["2199025081834"],
        "listingPrice": "0.2079 EOS",
        "collectionName": "chessunivers"
      }
    },
    {
      "purchaseSale": {
        "trxId": "b70bfe7ddea07a0be32991684fff17d6d74825e905b785e43be236845779f318",
        "buyer": "knottsy1.ftw",
        "saleId": "2319916"
      }
    }
    ...
  ]
}
```

### Modules
```yaml
Package name: atomicmarket
Version: v0.3.1
Doc: Substreams for AtomicMarket
Modules:
----
Name: map_events
Initial block: 0
Kind: map
Output Type: proto:antelope.atomicmarket.v1.AnyEvents
Hash: 5a5501f1620f3ae6025343497b917deae5044903

Name: graph_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.entity.v1.EntityChanges
Hash: 8d94d569565710b3556de2d382adda44960ac2c9
```