/*
 * Copyright (c) 2014 Arcterus
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

pub static SCREEN_ADDR: uint = 0xb8000;
pub static MAX_ROW: uint = 25;
pub static MAX_COLUMN: uint = 80;

pub fn move_cursor(row: uint, col: uint) {
   let pos = row * MAX_COLUMN + col;
   unsafe {
      asm!("
         mov al, 0xF
         mov dx, 0x3D4
         out dx, al

         mov ax, bx
         mov dx, 0x3D5
         out dx, al

         mov al, 0xE
         mov dx, 0x3D4
         out dx, al

         mov ax, bx
         shr ax, 8
         mov dx, 0x3D5
         out dx, al
      " : : "{bx}" (pos) : "al", "dx": "intel");
   }
}
