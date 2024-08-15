#!/bin/bash
set -e

URL="https://api.artifactsmmo.com/openapi.json"

echo "Downloading specs..."
curl -L "$URL" -o openapi.json

echo "Generating client..."
openapi-generator-cli generate -i openapi.json -g rust -t template --additional-properties=useSingleRequestParameter=true,preferUnsignedInt=true

echo "Fixing generated code..."
cargo clippy --fix --allow-dirty
cargo fmt

echo "Done!"
