build:
	cd extension && \
	rm -rf pkg && \
	wasm-pack build && \
	cd ../tools && \
	cargo run
