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

.PHONY: db_out
db_out: build
	substreams run -e eos.firehose.eosnation.io:9001 substreams.yaml db_out -s 323511941 -t +1

.PHONY: sink
sink: build
	substreams-sink-redis run -e https://eos.firehose.eosnation.io:9001  --manifest https://github.com/pinax-network/substreams-atomicmarket-sales/releases/download/v0.1.4/atomicmarketsales-v0.1.4.spkg --module-name prom_out -s 323323371 -t 328676379 --production-mode true

.PHONY: webhook
webhook: build
	~/substreams-sink-webhook-linux run -e https://eos.firehose.eosnation.io:9001 --manifest https://github.com/pinax-network/substreams-atomicmarket-sales/releases/download/v0.1.4/atomicmarketsales-v0.1.4.spkg --module-name db_out -s 323323371 -t 328676379 --webhook-url localhost:3003 --secret-key "d9ff4613e26f2d33a685be5ea03f0ebea5b00ef1acf4da0ea58bfbf436d27da6b246e55400ea143c30cf0d305f25998f3340538800de3cf6316e878f9d0101a3"