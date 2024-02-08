extern crate alloc;
use core::ptr;
use core::slice::SlicePattern;
use alloc::vec::Vec;
use crate::init::ramdisk::psf::PSF1Font;
use crate::task::module;
use crate::{serial_println, video::psf::{self, PSF2Font}};
use crate::init::ramdisk::psf::print_bitmap;

use super::init::Paging;

pub static mut RAMDISK: Vec<u8> = Vec::new();

pub fn init(ramdisk_addr: u64, ramdisk_size: usize, paging: &mut Paging) {
    unsafe { 
        RAMDISK = read_ramdisk(ramdisk_addr, ramdisk_size);
        for entry in cpio_reader::iter_files(&RAMDISK.as_mut()) {
            serial_println!("ramdisk: entry name: {}", entry.name());
            if entry.name() == "zap-light16.psf" {
                serial_println!("ramdisk: font found");
                let font = PSF1Font::parse(entry.file()).unwrap();  
                serial_println!("psf: glyph count: {}", font.glyph_count());
                serial_println!("psf: glyph size: {:?}", font.glyph_size());
            }

            if entry.name().starts_with("modules/") {
                serial_println!("ramdisk: loading module {}", entry.name());
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
