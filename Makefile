setup:
	rm -f .git/hooks/pre-commit && \
	cp pre-commit .git/hooks/pre-commit

lint:
	./pre-commit

build:
	rm -rf dist && \
	cargo build && \
	make -C scripts/content build

build-chromium:
	make build && \
	cargo run --bin tools chromium
