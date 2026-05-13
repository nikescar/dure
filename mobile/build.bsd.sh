#!/bin/sh
# Build script for FreeBSD with Linux Rust toolchain
# This script sets up the necessary environment for building with
# Linux compatibility layer libraries

export PATH="/compat/linux/usr/bin:$PATH"
export LIBCLANG_PATH="/compat/linux/usr/lib64/llvm19/lib"
export LD_LIBRARY_PATH="/compat/linux/usr/lib64/llvm19/lib"
export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=/compat/linux -I/compat/linux/usr/include"
export RUSTFLAGS="-L/compat/linux/usr/lib64 -L/compat/linux/usr/lib"

echo "Building with Linux compatibility layer..."
echo "Go: $(which go)"
echo "Rust: $(rustc --version)"

cargo "$@"
