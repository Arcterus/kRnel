# kRnel #

A small 64-bit, Multiboot kernel written in Rust.

## Dependencies ##
#### Build ####
* `clang` (capable of cross-compiling for x86-64)
* `nasm`
* `rust` (nightly)
* `ninja`
* `xorriso`

#### Runtime ####
* a Multiboot-compliant bootloader

#### Emulation ####
* `qemu`
* `ninja`

## Build Instructions ##

#### Linux ####
Some distributions have compilers that will work just fine out-of-the-box, while some don't.

```bash
$ yaourt -S clang nasm qemu rust-nightly-bin ninja libisoburn
$ cd path/to/kRnel
$ ./configure
$ ninja
```

#### OSX ####
You should use Homebrew for as much as possible.  However, you should install
the Rust nightly via the [installer](http://static.rust-lang.org/dist/rust-nightly-x86_64-apple-darwin.pkg).
The configure script autogenerates `binutils` for you because OSX has a broken
version by default.  Please note that this assumes you already have `clang`
installed via Xcode.

```bash
$ brew install nasm qemu ninja xorriso
$ cd path/to/kRnel
$ ./configure
$ ninja
```

## Emulation Instructions ##
```bash
$ ninja run
```

## Legal ##
Copyright (C) 2014 Arcterus.
All rights reserved.

See License.txt for description of this project's license.
