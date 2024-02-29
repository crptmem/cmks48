extern crate alloc;

use crate::init::init::Paging;
use crate::serial_println;
use crate::exec::elf;

/// Load a module in userspace
///
/// # Safety
/// Caller must ensure that `data` contains a valid ELF file and `paging`
/// contains a valid mutable reference to Paging struct
pub unsafe fn load(name: &str, data: &[u8], paging: &mut Paging) {
    serial_println!("mod: loading, name={}", name);
    elf::load_elf(data, paging)
}
