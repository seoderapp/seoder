#!/bin/sh

# prep images locally

SEODER_PROGRAM=app RUST_LOG=info cargo tauri build --target universal-apple-darwin
echo "Created mac universal build"
SEODER_PROGRAM=app RUST_LOG=info cargo tauri build
echo "Created m1 build"

cp ./target/universal-apple-darwin/release/bundle/dmg/Seoder_*_universal.dmg ./seoder_marketing/public/releases
cp ./target/release/bundle/dmg/Seoder_*_aarch64.dmg ./seoder_marketing/public/releases

echo "builds completed"