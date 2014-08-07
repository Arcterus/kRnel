#!/bin/sh

BASE_URL='http://static.rust-lang.org/dist/'

RUST_BINARY_DIR="rust-nightly-x86_64-unknown-linux-gnu"
RUST_BINARY_ARCHIVE="$RUST_BINARY_DIR.tar.gz"
RUST_BINARY_URL="$BASE_URL$RUST_BINARY_ARCHIVE"
if [[ ! -f "$RUST_BINARY_ARCHIVE" ]]; then
   echo "Downloading $RUST_BINARY_ARCHIVE from $BASE_URL"
   curl -O "$RUST_BINARY_URL" || exit 1
fi
if [[ ! -d rust ]]; then
   echo "Extracting $RUST_BINARY_DIR from $RUST_BINARY_ARCHIVE"
   tar -zxf "$RUST_BINARY_ARCHIVE" || exit 1
   echo "Moving $RUST_BINARY_DIR to rust"
   mv "$RUST_BINARY_DIR" rust
fi
mkdir -p install
if [[ "`ls install | grep libcore`" == "" || "`ls install | grep librlibc`" == "" ||
      "`ls install | grep librand`" == "" || "`ls install | grep libunicode`" == "" ]]; then
   echo "Installing libcore and librlibc"
   mv rust/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore*.rlib install/
   mv rust/lib/rustlib/x86_64-unknown-linux-gnu/lib/librlibc*.rlib install/
fi
