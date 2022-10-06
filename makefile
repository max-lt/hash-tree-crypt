clean:
	cargo clean

build:
	cargo build --release

start: build
	./target/release/hash-tree-crypt

# Development mode
run:
	cargo run
