.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: pack
pack:
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run: build
	substreams run -e eos.firehose.eosnation.io:9001 map_sales -s 323511941 -t +1

.PHONY: gui
gui: build
	substreams gui -e eos.firehose.eosnation.io:9001 map_sales -s 329693282 -t 329871503

.PHONY: prometheus
prometheus: build
	substreams run -e eos.firehose.eosnation.io:9001 substreams.yaml prom_out -s 323511941 -t +1

.PHONY: graph_out
graph_out:
	substreams run -e eos.firehose.eosnation.io:9001 substreams.yaml graph_out -s 323511941 -t +1

.PHONY: sink
sink: build
	substreams-sink-redis run -e https://eos.firehose.eosnation.io:9001  --manifest https://github.com/pinax-network/substreams-atomicmarket-sales/releases/download/v0.1.3/atomicmarketsales-v0.1.3.spkg --module-name prom_out -s 323323371 -t 328676379 --production-mode true