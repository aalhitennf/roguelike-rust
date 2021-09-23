#!/bin/bash
cargo build --release
cp ./target/release/libworldgen.so ../../libworldgen.so