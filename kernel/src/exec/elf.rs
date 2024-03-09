extern crate alloc;

use elf::endian::AnyEndian;
use elf::ElfBytes;
use elf::abi::PT_LOAD;
use elf::segment::ProgramHeader;
use alloc::vec::Vec;
use x86_64::instructions::tlb::flush_all;
use x86_64::registers::control::{Cr3, Cr3Flags};
use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::paging::{FrameAllocator, Page, PageTableFlags, PhysFrame};
use crate::exec::exec;
use crate::init::init::Paging;
use crate::serial_println;
use crate::common::x86::memory::{self, create_new_user_pagetable};

/// Load ELF file in userspace
///
/// # Safety
/// Caller must ensure that `data` contains a valid ELF file and `paging`
/// contains a valid mutable reference to Paging struct
pub unsafe fn load_elf(data: &[u8], paging: &mut Paging) {
    let file = ElfBytes::<AnyEndian>::minimal_parse(data).expect("chego blya");
    let all_load_phdrs: Vec<ProgramHeader> = file.segments().unwrap()
        .iter()
        .filter(|phdr|{phdr.p_type == PT_LOAD})
        .collect();
    let (user_page_table_ptr, user_page_table_physaddr) = create_new_user_pagetable();
    serial_println!("elf: new directory address is {:#016x}, pointer is {:?}", user_page_table_physaddr, user_page_table_ptr);
    Cr3::write(PhysFrame::containing_address(PhysAddr::new(user_page_table_physaddr)), Cr3Flags::empty());
    flush_all();
    let stack = paging.frame_allocator.allocate_frame().unwrap();

    let first_page = paging.frame_allocator.allocate_frame().unwrap();
    if memory::allocate_pages(
        user_page_table_ptr,
        VirtAddr::new(first_page.start_address().as_u64()),
        0x1000u64, // Size (bytes)
        PageTableFlags::PRESENT |
        PageTableFlags::WRITABLE |
        PageTableFlags::USER_ACCESSIBLE).is_err() {
        serial_println!("ой");
    }
    /*
    memory::create_mapping(
        Page::containing_address(VirtAddr::new(first_page.start_address().as_u64())),
        first_page.start_address().as_u64(),
        &mut paging.mapper, 
        &mut paging.frame_allocator,
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE);*/

    let stack = paging.frame_allocator.allocate_frame().unwrap();
    if memory::allocate_pages(
        user_page_table_ptr,
        VirtAddr::new(stack.start_address().as_u64()),
        0x1000u64, // Size (bytes)
        PageTableFlags::PRESENT |
        PageTableFlags::WRITABLE |
        PageTableFlags::USER_ACCESSIBLE).is_err() {
        serial_println!("ой");
    }

    serial_println!("mod: loading to {:#016x}", first_page.start_address().as_u64());

    for _ in 0..all_load_phdrs.len() + 10 {
        let phys = paging.frame_allocator.allocate_frame().unwrap();
        if memory::allocate_pages(
            user_page_table_ptr,
            VirtAddr::new(phys.start_address().as_u64()),
            0x1000u64, // Size (bytes)
            PageTableFlags::PRESENT |
            PageTableFlags::WRITABLE |
            PageTableFlags::USER_ACCESSIBLE).is_err() {
            serial_println!("ой");
        }
    }

    for i in 0..all_load_phdrs.len() {
        let page: x86_64::structures::paging::Page<x86_64::structures::paging::Size4KiB> = Page::containing_address(
            VirtAddr::new(first_page.start_address().as_u64() + (0x1000 * i as u64))); 
        let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
        let data_ptr: *const u64 = data.as_ptr() as *const u64;
        page_ptr.copy_from(data_ptr.offset((all_load_phdrs[i].p_offset / 8) as isize) as *const u64, (all_load_phdrs[i].p_filesz) as usize);
    }
    exec(VirtAddr::new(first_page.start_address().as_u64() + file.ehdr.e_entry - 0x400000), paging, stack.start_address().as_u64());
}
