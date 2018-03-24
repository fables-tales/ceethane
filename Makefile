.PHONY: all build test-examples
all: build test-examples

build:
	cargo build

test-examples:
	cargo run --example basic | grep -i 'hello' | grep -i '1337'
	cargo run --example err | grep -i 'oh no' | grep -i 'Kind: Other'
	cargo run --example fake_web_app | grep -i 'started' | grep -i '32'
