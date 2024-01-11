build:
	rm -rf dist && \
	cargo build && \
	make -C scripts/content build

build-chromium:
	make build && \
	cargo run --bin tools chromium
