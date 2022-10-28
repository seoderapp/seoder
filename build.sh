#!/bin/sh

# prep images locally

mac=""

if [ "$(uname)" == "Darwin" ]; then
    mac="true"
fi

MAC_OS=$mac SEODER_PROGRAM=app cargo tauri build --target universal-apple-darwin
echo "Created mac universal build"
MAC_OS=$mac SEODER_PROGRAM=app cargo tauri build
echo "Created m1 build"

cp ./target/universal-apple-darwin/release/bundle/dmg/Seoder_*_universal.dmg ./seoder_marketing/public/releases
cp ./target/release/bundle/dmg/Seoder_*_aarch64.dmg ./seoder_marketing/public/releases

echo "builds completed"