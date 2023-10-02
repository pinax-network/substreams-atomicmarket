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
	substreams run -e eos.firehose.eosnation.io:9001 map_events -s 329695542 -t +1

.PHONY: gui
gui: build
	substreams gui -e eos.firehose.eosnation.io:9001 map_events -s 329693282 -t 329871503

.PHONY: prometheus
prometheus: build
	substreams gui -e eos.firehose.eosnation.io:9001 substreams.yaml prom_out -s 329695542 -t +10

.PHONY: sink
sink: build
	substreams-sink-prometheus run -e eos.firehose.eosnation.io:9001 substreams.yaml prom_out -s 329695542 -t +1