extern crate alloc;
use alloc::borrow::ToOwned;
use alloc::rc::Rc;
use elf::endian::AnyEndian;
use elf::ElfBytes;
use elf::abi::PT_LOAD;
use elf::segment::ProgramHeader;
use alloc::vec::Vec;
use x86_64::structures::paging::Mapper;
use x86_64::structures::paging::Size4KiB;
use x86_64::VirtAddr;
use x86_64::PhysAddr;
use x86_64::structures::paging::Page;
use x86_64::structures::paging::PhysFrame;
use crate::init::init::Paging;
use crate::common::x86::memory;
use crate::{serial_println, FRAME_ALLOCATOR, MAPPER};
use crate::BootInfoFrameAllocator;
use core::borrow::Borrow;
use core::alloc::Layout;

pub fn load_elf(data: &[u8], paging: &mut Paging) {
    serial_println!("ELF: loading executable");
    let file = ElfBytes::<AnyEndian>::minimal_parse(data).expect("chego blya");
    let common_sections = file.find_common_data().unwrap();
    let all_load_phdrs: Vec<ProgramHeader> = file.segments().unwrap()
        .iter()
        .filter(|phdr|{phdr.p_type == PT_LOAD})
        .collect();

    for section in all_load_phdrs {
        serial_println!("ELF: LOAD section: p_paddr={:#016x}, p_vaddr={:#016x}, p_offset={:#016x}, p_filesz={:#016x}, p_memsz={:#016x}",
            section.p_paddr,
            section.p_vaddr,
            section.p_offset,
            section.p_filesz,
            section.p_memsz);

        unsafe {
            let layout = Layout::array::<u64>(section.p_filesz as usize).unwrap();
            let phys = alloc::alloc::alloc(layout);
            let page = Page::containing_address(VirtAddr::new(section.p_vaddr));
            memory::create_mapping(page, phys as u64, &mut paging.mapper, &mut paging.frame_allocator);

            // write the string `New!` to the screen through the new mapping
            let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
            serial_println!("ELF: mapped {:?} to {:?}", phys, page_ptr);
            serial_println!("{:#?}", page_ptr.read());
            unsafe { page_ptr.write_volatile(0x00) };
    /*
            let page: x86_64::structures::paging::Page<Size4KiB> = Page::containing_address(VirtAddr::new(section.p_vaddr as u64));
            use x86_64::structures::paging::PageTableFlags as Flags;

            let frame: PhysFrame<Size4KiB> = PhysFrame::containing_address(PhysAddr::new(phys as u64));
            let flags = Flags::PRESENT | Flags::WRITABLE; 
            
            paging.mapper.map_to(page, frame, flags, &mut paging.frame_allocator);
            serial_println!("ELF: mapped {:#016x} to {:?}", section.p_vaddr, phys);
            core::ptr::copy(section.p_vaddr as *mut u64, core::ptr::addr_of!(data) as *mut u64, 5);*/
        }
    }
}
