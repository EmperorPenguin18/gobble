build: gobble.rs Cargo.toml
	cargo build

install: gobble
	@mkdir -p /usr/bin/
	@mv target/debug/gobble /usr/bin/gobble
