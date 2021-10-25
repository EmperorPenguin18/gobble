build: gobble.rs Cargo.toml gobble.1.md
	cargo build --release
	pandoc gobble.1.md -s -t man -o gobble.1
	gzip -f gobble.1

install: target/release/gobble gobble.1.gz
	@mkdir -p /usr/bin/
	@mv target/release/gobble /usr/bin/gobble
	@mkdir -p /usr/share/man/man1
	@mv gobble.1.gz /usr/share/man/man1/gobble.1.gz

uninstall:
	@rm /usr/bin/gobble
	@rm /usr/share/man/man1/gobble.1.gz
