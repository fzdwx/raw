#!/usr/bin/env just --justfile

run:
   cargo run

rm:
   RUST_BACKTRACE=full cargo run README.md justfile LICENSE

rb:
    ./target/release/raw

# install raw editor in linux.
install:
    sudo cp just /usr/local/bin/just
    cargo build --release
    sudo cp ./target/release/raw /usr/local/bin/ra

release:
  cargo build --release

test:
  cargo test

t lib:
   cargo test --color=always --package raw --lib '{{lib}}' -- --exact -Z unstable-options --show-output

# lint 普通模式
lint:
  cargo clippy

# lint 严格模式
lint-s:
    cargo clippy -- -W clippy::pedantic

bin:
  cargo run --bin raw