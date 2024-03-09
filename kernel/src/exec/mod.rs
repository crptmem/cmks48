use core::{ops::Index, ptr::{self, addr_of}};

use x86_64::{instructions::tlb::flush_all, registers::control::{Cr3, Cr3Flags}, structures::paging::{FrameAllocator, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame, Translate}, PhysAddr, VirtAddr};

use crate::{common::x86::memory::{self, create_new_user_pagetable}, init::init::Paging, serial_println, task::userspace::jmp_to_usermode};

pub mod elf;
pub mod module;

pub fn exec(addr: VirtAddr, paging: &mut Paging, stack: u64) {  
    let new_directory = paging.frame_allocator.allocate_frame().unwrap();
    memory::create_mapping(
        Page::containing_address(VirtAddr::new(new_directory.start_address().as_u64())),
        new_directory.start_address().as_u64(),
        &mut paging.mapper, 
        &mut paging.frame_allocator,
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE);
 
    serial_println!("exec: new pt addr is {:?}", new_directory);
    unsafe {
        serial_println!("exec: jumping to {:#016x} in usermode, stack is located on {:#016x}", addr.as_u64(), stack);
        jmp_to_usermode(VirtAddr::new(addr.as_u64()), VirtAddr::new(stack));
    }  
}
