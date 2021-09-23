#!/bin/bash
cargo build --release
cp ./target/release/libconsole.so ../../libconsole.so