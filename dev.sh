#!/bin/sh

mac=""

if [ "$(uname)" == "Darwin" ]; then
    mac="true"
fi

MAC_OS=$mac DEV=true SEODER_PROGRAM=app RUST_LOG=info cargo tauri dev