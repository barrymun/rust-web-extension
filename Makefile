build:
	cd extension \
	&& rm -rf pkg \
	&& wasm-pack build \
	&& cd ../scripts \
	&& cargo run
