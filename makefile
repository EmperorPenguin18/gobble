build: gobble.rs Cargo.toml
	cargo build --release

install: target/release/gobble
	@mkdir -p /usr/bin/
	@mv target/release/gobble /usr/bin/gobble

uninstall:
	@rm /usr/bin/gobble
