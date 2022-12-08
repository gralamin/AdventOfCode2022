prog=$(shell basename $(CURDIR))

build:
	cargo build --verbose

test:
	cargo test --verbose

clean:
	cargo clean --verbose
	- rm $(prog).day

format:
	cargo fmt

lint:
	cargo fmt -- --check

coverage:
	cargo tarpaulin --verbose --all-features --workspace --timeout 120 --fail-under 70

all: build

.PHONY: build test all clean format lint coverage
