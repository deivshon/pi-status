#!/bin/sh

cargo build --release --manifest-path=./back/Cargo.toml || exit

npm --prefix ./front/pi-status-front run build || exit

./back/target/release/pi-status
