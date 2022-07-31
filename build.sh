#!/bin/bash

# To build the OS we need to use Rust nightly and recompile the standard library for our custom
# target
# cargo build -Zbuild-std
cargo bootimage
