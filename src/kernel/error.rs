/*
 * Copyright (c) 2014 Arcterus
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use core::fmt::FormatWriter;
use super::console;

pub fn panic(reason: &str, file: &str, line: uint) -> ! {
   console::color_println("Ohs noes!  You've been harpooned!", console::Red, console::BACKGROUND_COLOR);
   // print fail whale (kraken?)
   console::print("Reason: ");
   console::println(reason);   // FIXME: this fails when any of the bytes are accessed
                               //        (like the .as_bytes() call in color_println).
                               //        Perhaps the memory map is overwriting the data?
   console::print("File: ");
   console::println(file);
   console::print("Line: ");
   //writeln!(console::default_screen(), "{}", line);   // FIXME: also broken
   // TODO: print line later
   // wait 10 seconds
   abort();
}

#[no_mangle]
#[inline]
pub extern "C" fn abort() -> ! {
   unsafe { super::reset::immediate_reset(); }
}

#[no_mangle]
#[inline]
pub fn rust_begin_unwind(_: &::core::fmt::Arguments, file: &'static str, line: uint) -> ! {
   panic("", file, line);
}
