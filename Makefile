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
	substreams run -e eos.substreams.pinax.network:443 map_events -s 323511941 -t +1

.PHONY: gui
gui: build
	substreams gui -e eos.substreams.pinax.network:443 map_events -s 323511941 -t +1

.PHONY: prometheus
prometheus: build
	substreams run -e eos.substreams.pinax.network:443 substreams.yaml prom_out -s 323511941 -t +1

.PHONY: graph_out
graph_out: build
	substreams run -e eos.substreams.pinax.network:443 substreams.yaml graph_out -s 323511941 -t +1

.PHONY: db_out
db_out: build
	substreams run -e eos.substreams.pinax.network:443 substreams.yaml db_out -s 323511941 -t +1

.PHONY: sink
sink: build
	substreams-sink-redis run -e https://eos.substreams.pinax.network:443  --manifest https://github.com/pinax-network/substreams-atomicmarket-sales/releases/download/v0.2.2/atomicmarketsales-v0.2.2.spkg --module-name prom_out -s 323323371 -t 328676379 --production-mode true