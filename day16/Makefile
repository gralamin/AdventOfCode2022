prog=$(shell basename $(CURDIR))

build:
	cargo build --verbose
	cp target/debug/$(prog) $(prog).day

build_r:
	cargo build -r --verbose
	cp target/release/$(prog) $(prog).day

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
	cargo tarpaulin --doc --verbose --all-features --workspace --timeout 120  --exclude-files src/main.rs --fail-under 70

all: build

.PHONY: build test all clean format lint coverage
