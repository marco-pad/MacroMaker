#!/bin/bash
cargo build -p macromaker --release
cargo +nightly build -p firmware --release --target riscv64gc-unknown-linux-musl -Zbuild-std=core,std,panic_abort
