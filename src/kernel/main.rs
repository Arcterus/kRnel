/*
 * Copyright (c) 2014 Arcterus
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![crate_id(name = "kRnel",
            vers = "0.0.1",
            author = "Arcterus",
            license = "MPL v2.0")]

#![allow(ctypes)]
#![no_std]
#![feature(asm, globs, macro_rules, phase)]

#[phase(plugin, link)]
extern crate core;
extern crate rand;
extern crate rlibc;
extern crate unicode;

pub use core::prelude::*;

pub use drivers::console;
pub use platform::*;

pub use std = core;

use core::fmt::FormatWriter;

#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64/mod.rs"]
pub mod platform;

#[path = "../drivers/mod.rs"]
pub mod drivers;
pub mod memory;
pub mod error;
pub mod support;

#[no_mangle]
pub fn main(mem: *const memory::BootMemMap) {
   let mem: &memory::BootMemMap = unsafe { &(*mem) };
   console::clear_screen();
   console::print("iiiiiiiiiiiiiiiiiiiiiiiiiii\niiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii\x08\x08\x08\x08\x08test");
   console::println("");
   let usable = mem.usable();
   let mut len = usable.len();
   while len > 0 {
      console::print_bytes([(len % 10 + '0' as uint) as u8]);
      len /= 10;
      console::println("");
   }
   //error::panic("End of kernel", file!(), line!());
   //write!(console::default_screen(), "{}", 1u);
}
