#!/usr/bin/env bash

# trunk build --release

set -e

RELEASE=false
for arg in "$@"; do
    case "$arg" in
        --release|-r) RELEASE=true ;;
    esac
done

rustup target add wasm32-unknown-unknown
# Add wasm32-unknown-unknown toolchain

# Auto-detect and install matching wasm-bindgen-cli version
# This prevents "schema version mismatch" errors between the compiled .wasm and the CLI tool
# Works even without Cargo.lock by querying cargo's dependency resolution
echo "Detecting wasm-bindgen version..."
REQUIRED_WASM_BINDGEN_VERSION="$(
    cargo tree --target wasm32-unknown-unknown -p wasm-bindgen --depth 0 2>/dev/null | \
    grep '^wasm-bindgen' | \
    sed 's/wasm-bindgen v\([^ ]*\).*/\1/'
)"

if [ -z "$REQUIRED_WASM_BINDGEN_VERSION" ]; then
    echo "ERROR: Could not determine wasm-bindgen version from cargo tree" >&2
    echo "Make sure wasm-bindgen is in your dependencies for wasm32-unknown-unknown target" >&2
    exit 1
fi

echo "Required wasm-bindgen-cli version: $REQUIRED_WASM_BINDGEN_VERSION"
INSTALLED_WASM_BINDGEN_VERSION="$(wasm-bindgen --version 2>/dev/null | awk '{print $2}')"

if [ "$INSTALLED_WASM_BINDGEN_VERSION" != "$REQUIRED_WASM_BINDGEN_VERSION" ]; then
    echo "Installing wasm-bindgen-cli $REQUIRED_WASM_BINDGEN_VERSION (currently: ${INSTALLED_WASM_BINDGEN_VERSION:-none})..."
    cargo install wasm-bindgen-cli --locked --version "$REQUIRED_WASM_BINDGEN_VERSION"
else
    echo "wasm-bindgen-cli $INSTALLED_WASM_BINDGEN_VERSION is already installed"
fi

if $RELEASE; then
    cargo build --target wasm32-unknown-unknown --release
    WASM_PATH=../target/wasm32-unknown-unknown/release/dure.wasm
else
    cargo build --target wasm32-unknown-unknown
    WASM_PATH=../target/wasm32-unknown-unknown/debug/dure.wasm
fi

rm -rf dist/*
# Build wasm
wasm-bindgen "$WASM_PATH" --out-dir dist --web
# bindgen

cp index.html dist
cp worker.js dist
cp favicon.ico dist

# Compress assets for faster web transport
echo "Compressing assets..."
cd dist

# Compress WASM files (typically the largest assets)
for file in *.wasm; do
    if [ -f "$file" ]; then
        echo "  Compressing $file..."
        gzip -9 -k -f "$file"  # -9 = best compression, -k = keep original, -f = force overwrite
        echo "    Original: $(du -h "$file" | cut -f1)"
        echo "    Gzipped:  $(du -h "$file.gz" | cut -f1)"
        
        # Also create brotli if available (often 15-20% smaller than gzip)
        if command -v brotli &> /dev/null; then
            brotli -9 -k -f "$file"  # -9 = best compression
            echo "    Brotli:   $(du -h "$file.br" | cut -f1)"
        fi
    fi
done

# Compress JavaScript files
for file in *.js; do
    if [ -f "$file" ]; then
        echo "  Compressing $file..."
        gzip -9 -k -f "$file"
        if command -v brotli &> /dev/null; then
            brotli -9 -k -f "$file"
        fi
    fi
done

cd ..
echo "Build complete! Assets are in dist/ with compressed versions (.gz and .br)"

rm -rf ./serv/*
cp -rf dist/* ./serv

# Copy to ~/work/dure-wasm if it exists
if [ -d ~/work/dure-wasm ]; then
    rm -rf ~/work/dure-wasm/*
    echo "Copying to ~/work/dure-wasm..."
    cp -rf dist/* ~/work/dure-wasm/
    echo "Copied to ~/work/dure-wasm"
fi
