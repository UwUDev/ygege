#!/bin/bash
export CC=arm-linux-gnueabihf-gcc
export CXX=arm-linux-gnueabihf-g++
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-g++
export RUSTC_LINKER=arm-linux-gnueabihf-g++

cargo build --release --target armv7-unknown-linux-gnueabihf