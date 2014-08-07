#!/bin/sh

BINUTILS_VERSION='2.23.2'
BASE_URL='http://ftp.gnu.org/gnu/binutils/'

BINUTILS_DIR="binutils-$BINUTILS_VERSION"
BINUTILS_ARCHIVE="$BINUTILS_DIR.tar.bz2"
BINUTILS_URL="$BASE_URL$BINUTILS_ARCHIVE"
if [ ! -f "$BINUTILS_ARCHIVE" ]; then
   echo "Downloading $BINUTILS_ARCHIVE from $BASE_URL"
   curl -O "$BINUTILS_URL" || exit 1
fi
if [ ! -d binutils ]; then
   echo "Extracting $BINUTILS_DIR from $BINUTILS_ARCHIVE"
   tar -jxf "$BINUTILS_ARCHIVE" || exit 1
   echo "Moving $BINUTILS_DIR to binutils"
   mv "$BINUTILS_DIR" binutils
fi
if [ ! -f install/bin/x86_64-linux-elf-ld ]; then
   cd binutils
   echo "Configure binutils"
   CC=clang CFLAGS='-Wno-string-plus-int -Wno-empty-body -Wno-self-assign' \
      ./configure --prefix=/ --target=x86_64-linux-elf --enable-ld \
      --disable-gold --disable-werror || exit 1
   echo "Building binutils"
   make -j4 || exit 1
   echo "Installing binutils"
   make DESTDIR="$PWD"/../install install || exit 1
fi
