/*
 * Copyright (c) 2014 Arcterus
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use core::prelude::*;
use core::fmt;

pub struct ScreenWriter;

impl fmt::FormatWriter for ScreenWriter {
   #[inline]
   fn write(&mut self, bytes: &[u8]) -> fmt::Result {
      super::print_bytes(bytes);
      Ok(())
   }
}
