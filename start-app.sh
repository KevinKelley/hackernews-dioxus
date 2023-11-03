#!/bin/sh

clear

dx build --features web --release
cargo run --features ssr --release
