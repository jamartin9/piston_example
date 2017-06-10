#!/usr/bin/env bash

function main() {
    #local TARGET="x86_64-unknown-linux-musl"
    local TARGET="x86_64-unknown-linux-gnu"
    export CARGO_INCREMENTAL=1 && \
    rustup run stable cargo update && \
    cargo check --target=$TARGET --release && \
    rustup run stable cargo build --target=$TARGET --release && \
    rustup run stable cargo  test --target=$TARGET --release && \
    rustup run stable cargo  doc --target=$TARGET --release && \
    rustup run nightly cargo clippy && \
    rustup run stable cargo fmt && \
    # Use help text and versions for readme
    rustup run stable cargo run --target=$TARGET --release --bin game-client -- --help > README.txt && \
    echo "" >> README.txt && \
    echo "build target: ${TARGET}" >> README.txt && \
    echo "" >> README.txt && \
    rustup run stable rustc --version >> README.txt && \
    echo "" >> README.txt && \
    rustup run stable cargo --version >> README.txt && \
    echo "" >> README.txt && \
    rustup run nightly cargo --version >> README.txt && \
    echo "" >> README.txt && \
    echo "Clippy version:" >> README.txt && \
    echo "" >> README.txt && \
    rustup run nightly cargo clippy --version >> README.txt && \
    echo "" >> README.txt && \
    rustup --version >> README.txt && \
    echo "" >> README.txt
}

main
