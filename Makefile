build:
	rm -rf dist && \
	cargo build && \
	make -C scripts/content build && \
	cargo run --bin tools
