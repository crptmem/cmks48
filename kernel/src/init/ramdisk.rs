extern crate alloc;
use core::ptr::{self, addr_of};
use core::slice::SlicePattern;
use alloc::vec::Vec;
use crate::exec::module;
use crate::serial_println;

use super::init::Paging;

pub static mut RAMDISK: Vec<u8> = Vec::new();

pub fn init(ramdisk_addr: u64, ramdisk_size: usize, paging: &mut Paging) {
    unsafe { 
        RAMDISK = read_ramdisk(ramdisk_addr, ramdisk_size);
        for entry in cpio_reader::iter_files(&RAMDISK.as_mut()) {
            if entry.name().starts_with("modules/") {
                module::load(entry.name()
                             .strip_prefix("modules/")
                             .unwrap(), entry.file().as_slice(), paging);
            }
        }
    }
}

pub fn read_ramdisk(ramdisk_addr: u64, ramdisk_size: usize) -> Vec<u8> {
    // Convert u64 to raw pointer;w
    let ramdisk_ptr: *const u8 = ramdisk_addr as *const u8;
    let mut ret: Vec<u8> = Vec::new();

    // Use the pointer to read the ramdisk data
    
    unsafe {
        // Iterate through the memory and print the first 16 bytes as hexadecimal
        let mut current_ptr = ramdisk_ptr;
        for _ in 0..ramdisk_size {
            let byte = ptr::read_volatile(current_ptr);
            current_ptr = current_ptr.add(1);
            ret.push(byte);
        }
    }
    return ret
}
