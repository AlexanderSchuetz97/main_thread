#!/usr/bin/env bash
# This shell script only works on linux and requires wine with binfmt misc installed.

set -e

cargo run --example test
cargo run --example test --target x86_64-pc-windows-gnu

cd dll_test_crate
cargo build
cargo build --target x86_64-pc-windows-gnu
cargo run --example test
cargo run --example test --target x86_64-pc-windows-gnu

cd ..