prepare:
	rm -f .git/hooks/pre-commit && \
	cp pre-commit .git/hooks/pre-commit

lint:
	./pre-commit

build:
	rm -rf dist && \
	cargo build && \
	make -C scripts/background build
	make -C scripts/content build
	make -C scripts/popup build

build-chromium:
	make build && \
	cargo run --bin browser-pack chromium

build-gecko:
	make build && \
	cargo run --bin browser-pack gecko
