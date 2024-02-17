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
use crate::serial_println;
use crate::exec::symbol::{get_symbol_ptr, symbol_register};
use core::mem::transmute;

static mut MODULES_COUNT: usize = 0;

fn load_elf(data: &[u8], paging: &mut Paging) {
    let file = ElfBytes::<AnyEndian>::minimal_parse(data).expect("chego blya");
    let all_load_phdrs: Vec<ProgramHeader> = file.segments().unwrap()
        .iter()
        .filter(|phdr|{phdr.p_type == PT_LOAD})
        .collect();

    let first_page = paging.frame_allocator.allocate_frame().unwrap();
    memory::create_mapping(Page::containing_address(VirtAddr::new(first_page.start_address().as_u64())), first_page.start_address().as_u64(), &mut paging.mapper, &mut paging.frame_allocator);
    serial_println!("mod: loading to {:#016x}", first_page.start_address().as_u64());
    for _ in 0..all_load_phdrs.len() + 10 {
        let phys = paging.frame_allocator.allocate_frame().unwrap();
        memory::create_mapping(Page::containing_address(VirtAddr::new(phys.start_address().as_u64())), phys.start_address().as_u64(), &mut paging.mapper, &mut paging.frame_allocator);
    }

    for i in 0..all_load_phdrs.len() {
        /* serial_println!("mod: LOAD sections: p_paddr={:#016x}, p_vaddr={:#016x}, p_offset={:#016x}, p_filesz={:#016x}, p_memsz={:#016x}",
            all_load_phdrs[i].p_paddr,
            all_load_phdrs[i].p_vaddr,
            all_load_phdrs[i].p_offset,
            all_load_phdrs[i].p_filesz,
            all_load_phdrs[i].p_memsz); */
        
        unsafe {
            let page: x86_64::structures::paging::Page<x86_64::structures::paging::Size4KiB> = Page::containing_address(
                VirtAddr::new(first_page.start_address().as_u64() + (0x1000 * i as u64))); 
            let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
            let data_ptr: *const u64 = data.as_ptr() as *const u64;
            /*serial_println!("mod: {} section copying {:#08x} ({}) bytes from {:?}-{:?} to {:?}",
                i,
                all_load_phdrs[i].p_filesz,
                all_load_phdrs[i].p_filesz,
                data_ptr.offset((all_load_phdrs[i].p_offset / 8) as isize),
                data_ptr.offset((all_load_phdrs[i].p_offset + all_load_phdrs[i].p_filesz) as isize),
                page_ptr);*/
            page_ptr.copy_from(data_ptr.offset((all_load_phdrs[i].p_offset / 8) as isize) as *const u64, (all_load_phdrs[i].p_filesz) as usize);
        }
    }

    let code: extern "C" fn(fn([u8; 24], u64), fn(&[u8]) -> *mut u64) = unsafe { transmute(first_page.start_address().as_u64() + file.ehdr.e_entry - 0x400000) };
    unsafe { MODULES_COUNT += 1; };
    (code)(symbol_register, get_symbol_ptr);
}

pub fn load(name: &str, data: &[u8], paging: &mut Paging) {
    serial_println!("mod: loading, name={}", name);  
    let file = ElfBytes::<AnyEndian>::minimal_parse(data).expect("chego blya");
    let common_sections = file.find_common_data().unwrap();
    load_elf(data, paging)
}
