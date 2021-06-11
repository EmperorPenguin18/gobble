build: gobble.rs Cargo.toml
	cargo build

install: target/debug/gobble
	@mkdir -p /usr/bin/
	@mv target/debug/gobble /usr/bin/gobble
