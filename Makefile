all:
	cargo build
	cargo test

basic-example:
	cargo run --example basic | humanlog
