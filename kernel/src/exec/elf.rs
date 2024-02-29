extern crate alloc;

use elf::endian::AnyEndian;
use elf::ElfBytes;
use elf::abi::PT_LOAD;
use elf::segment::ProgramHeader;
use alloc::vec::Vec;
use x86_64::VirtAddr;
use x86_64::structures::paging::{FrameAllocator, Page, PageTableFlags};
use crate::init::init::Paging;
use crate::{serial_println, task};
use crate::common::x86::memory::{self};

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

    let first_page = paging.frame_allocator.allocate_frame().unwrap();
    memory::create_mapping(
        Page::containing_address(VirtAddr::new(first_page.start_address().as_u64())),
        first_page.start_address().as_u64(),
        &mut paging.mapper, 
        &mut paging.frame_allocator,
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE);

    let stack = paging.frame_allocator.allocate_frame().unwrap();
    memory::create_mapping(
        Page::containing_address(VirtAddr::new(stack.start_address().as_u64())),
        stack.start_address().as_u64(),
        &mut paging.mapper, 
        &mut paging.frame_allocator,
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE);
    serial_println!("mod: loading to {:#016x}", first_page.start_address().as_u64());

    serial_println!("mod: loading to {:#016x}", first_page.start_address().as_u64());
    for _ in 0..all_load_phdrs.len() + 10 {
        let phys = paging.frame_allocator.allocate_frame().unwrap();
        memory::create_mapping(
            Page::containing_address(VirtAddr::new(phys.start_address().as_u64())),
            phys.start_address().as_u64(),
            &mut paging.mapper,
            &mut paging.frame_allocator,
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE);
    }

    for i in 0..all_load_phdrs.len() {
        let page: x86_64::structures::paging::Page<x86_64::structures::paging::Size4KiB> = Page::containing_address(
            VirtAddr::new(first_page.start_address().as_u64() + (0x1000 * i as u64))); 
        let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
        let data_ptr: *const u64 = data.as_ptr() as *const u64;
        page_ptr.copy_from(data_ptr.offset((all_load_phdrs[i].p_offset / 8) as isize) as *const u64, (all_load_phdrs[i].p_filesz) as usize);
    }
    task::userspace::jmp_to_usermode(VirtAddr::new(first_page.start_address().as_u64() + file.ehdr.e_entry - 0x400000), VirtAddr::new(stack.start_address().as_u64()));
}
