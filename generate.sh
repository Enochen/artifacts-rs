#!/bin/bash
set -e

URL="https://api.artifactsmmo.com/openapi.json"

echo "Downloading spec..."
curl -L "$URL" -o spec/openapi.json --create-dirs

echo "Generating client..."
openapi-generator-cli generate -i spec/openapi.json -c generator-config.yaml

echo "Fixing generated code..."
cargo clippy --fix --allow-dirty
cargo fmt

echo "Done!"
