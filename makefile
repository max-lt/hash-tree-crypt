clean:
	cargo clean

build: src/*
	cargo build --release

start: build
	./target/release/hash-tree-crypt

# Development mode
run:
	cargo run
