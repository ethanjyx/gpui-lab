#!/bin/zsh
set -euo pipefail
cd -- "${0:A:h}"
export PATH="$HOME/.cargo/bin:$PATH"
cargo build --locked
mkdir -p 'GPUI Lab.app/Contents/MacOS'
cp target/debug/gpui-lab 'GPUI Lab.app/Contents/MacOS/gpui-lab'
cp Info.plist 'GPUI Lab.app/Contents/Info.plist'
open 'GPUI Lab.app'
