default:
	cargo build --release

# Required
# - rustup aarch64-unknown-linux-gnu toolchain
# - aarch64-linux-gnu toolchain
arm:
	RUSTFLAGS='-C target-feature=+crt-static -C linker=aarch64-linux-gnu-gcc' cargo install --path . --target aarch64-unknown-linux-gnu

clean:
	cargo clean
