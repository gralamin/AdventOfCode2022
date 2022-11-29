prog=$(shell basename $(CURDIR))

build:
	cargo build --verbose
	cp target/debug/$(prog) $(prog).day

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
	cargo +nightly tarpaulin --verbose --all-features --workspace --timeout 120

all: build

.PHONY: build test all clean format lint coverage
