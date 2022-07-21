#!/usr/bin/env just --justfile

run:
   cargo run

rm:
   cargo run README.md justfile LICENSE

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

test-l:
   cargo test --color=always --package raw --lib 'render::document::tests::test_line_len' -- --exact -Z unstable-options --show-output

# lint 普通模式
lint:
  cargo clippy

# lint 严格模式
lint-s:
    cargo clippy -- -W clippy::pedantic

bin:
  cargo run --bin raw