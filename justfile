#!/usr/bin/env just --justfile

run:
   cargo run

release:
  cargo build --release    

# lint 普通模式
lint:
  cargo clippy

# lint 严格模式
lint-s:
    cargo clippy -- -W clippy::pedantic

bin:
  cargo run --bin raw