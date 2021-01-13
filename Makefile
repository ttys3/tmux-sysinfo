.PHONY: all bin

all: static

static:
	cargo build --release --target x86_64-unknown-linux-musl
