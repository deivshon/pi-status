default:
	cargo build --release

arm64:
	RUSTFLAGS='-C target-feature=+crt-static -C linker=aarch64-linux-gnu-gcc' cargo build --release --target aarch64-unknown-linux-gnu

clean:
	cargo clean
