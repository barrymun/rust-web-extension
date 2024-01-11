build:
	rm -rf extension/pkg && \
	cargo build && \
	make -C extension && \
	make -C scripts/content && \
	cargo run --bin tools
