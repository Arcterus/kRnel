use core::iter::range_step;
use core::mem::{transmute, size_of};

static USABLE: u32 = 1;
static ACPI: u32 = 3;
static HIBERNATE: u32 = 4;

#[packed]
pub struct BootMemMap {
   stype: u32,
   size: u32,
   entry_size: u32,
   entry_version: u32,
   entries: ()
}

#[packed]
pub struct Entry {
   base_addr: u64,
   length: u64,
   mtype: u32,
   reserved: u32
}

impl BootMemMap {
   pub fn usable<'a>(&'a self) -> super::MemoryMap<'a> {
      let mut addr = (unsafe { transmute::<&BootMemMap, uint>(self) } + self.size as uint) as *mut &'a Entry;
      let mut count = 0;
      for i in range_step(0, self.size as uint, self.entry_size as uint) {
         unsafe {
            let entry = (transmute::<&(), uint>(&self.entries) + i) as *const Entry;
            if (*entry).mtype == USABLE {
               *addr = transmute(entry);
               addr = (addr as uint + size_of::<&'a Entry>()) as *mut &'a Entry;
               count += 1;
            }
         }
      }
      super::MemoryMap::new(addr, count)
   }
}

impl Entry {
   pub fn base_addr(&self) -> u64 {
      self.base_addr
   }

   pub fn length(&self) -> u64 {
      self.length
   }
}
