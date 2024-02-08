extern crate alloc;

use elf::endian::AnyEndian;
use elf::ElfBytes;
use elf::abi::PT_LOAD;
use elf::segment::ProgramHeader;
use alloc::vec::Vec;
use x86_64::VirtAddr;
use x86_64::structures::paging::{FrameAllocator, Page};
use crate::init::init::Paging;
use crate::common::x86::memory;
use crate::{serial_println, FRAME_ALLOCATOR, MAPPER};
use core::alloc::Layout;
use core::mem::transmute;
use core::ops::Sub;
use pretty_hex::*;

pub fn load_elf(data: &[u8], paging: &mut Paging) {
    let file = ElfBytes::<AnyEndian>::minimal_parse(data).expect("chego blya");
    let common_sections = file.find_common_data().unwrap();
    let all_load_phdrs: Vec<ProgramHeader> = file.segments().unwrap()
        .iter()
        .filter(|phdr|{phdr.p_type == PT_LOAD})
        .collect();

    serial_println!("ELF: {} LOAD sections", all_load_phdrs.len());
    for section in all_load_phdrs.clone() { 
        unsafe {
            let phys = paging.frame_allocator.allocate_frame().unwrap();
            let page = Page::containing_address(VirtAddr::new(section.p_vaddr)); 
            let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
            
            memory::create_mapping(page, phys.start_address().as_u64(), &mut paging.mapper, &mut paging.frame_allocator);
        }
    }
    for section in all_load_phdrs {
        serial_println!("ELF: LOAD section: p_paddr={:#016x}, p_vaddr={:#016x}, p_offset={:#016x}, p_filesz={:#016x}, p_memsz={:#016x}",
            section.p_paddr,
            section.p_vaddr,
            section.p_offset,
            section.p_filesz,
            section.p_memsz);
        
        unsafe {
            let page: x86_64::structures::paging::Page<x86_64::structures::paging::Size4KiB> = Page::containing_address(VirtAddr::new(section.p_vaddr)); 
            let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
            let data_ptr: *const u64 = data.as_ptr() as *const u64;
            serial_println!("ELF: copying {:#08x} ({}) bytes from {:?} to {:?}", section.p_filesz, section.p_filesz, data_ptr.offset((section.p_offset / 8) as isize), page_ptr);
            page_ptr.copy_from(data_ptr.offset((section.p_offset / 8) as isize) as *const u64, (section.p_filesz) as usize);
        }
    }
    serial_println!("ELF: entry is {:#016x}", file.ehdr.e_entry);
    let code: extern "C" fn() = unsafe { transmute(file.ehdr.e_entry) };
    (code)();
}
