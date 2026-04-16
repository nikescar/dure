#!/bin/bash
set -e

# Update AsyncAPI messages by building asyncapi-gen and copying to mobile/src/site/messages
#
# This replaces the old "cargo run --bin generate-asyncapi" approach

echo "🚀 Updating AsyncAPI messages..."

# Navigate to asyncapi-gen directory
cd "$(dirname "$0")/../crates/asyncapi-gen"

# Build the asyncapi-gen crate
echo "📦 Building asyncapi-gen..."
cargo build --release

# Run the generator to create JSON/YAML specs
echo "📝 Generating AsyncAPI specifications..."
cargo run --release

# Create destination directory if it doesn't exist
DEST_DIR="../../mobile/src/site/messages"
echo "📂 Creating destination directory: $DEST_DIR"
mkdir -p "$DEST_DIR"

# Copy messages folder to mobile/src/site/messages
SRC_DIR="./src/messages"
if [ -d "$SRC_DIR" ]; then
    echo "📋 Copying messages from $SRC_DIR to $DEST_DIR..."
    cp -r "$SRC_DIR"/* "$DEST_DIR/"
    echo "✅ Messages copied successfully!"
else
    echo "❌ Error: Source messages directory not found: $SRC_DIR"
    exit 1
fi

echo "✨ AsyncAPI update complete!"
echo ""
echo "Generated files:"
echo "  - docs/asyncapi.json"
echo "  - docs/asyncapi.yaml"
echo "  - $DEST_DIR/"
echo "  - $MOBILE_MESSAGES/"
