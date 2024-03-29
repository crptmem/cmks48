extern crate lazy_static;
extern crate alloc;

use crate::common::x86::memory::{MemoryInfo, MEMORY_INFO};
use crate::init::ramdisk::{self};
use crate::task::userspace;
use crate::{mm, serial_println};
use crate::common::x86::{gdt, idt, memory};
use alloc::boxed::Box;
use bootloader_api::info::MemoryRegions;
use x86_64::structures::paging::{Page, PageTableFlags};
use x86_64::{structures::paging::OffsetPageTable, VirtAddr, registers::control::Cr3};

#[derive(Debug)]
pub struct Paging {
    pub frame_allocator: memory::BootInfoFrameAllocator,
    pub mapper: OffsetPageTable<'static>,
    pub memory_regions: &'static MemoryRegions,
    pub rsdp_addr: u64,
    pub phys_mem_virtaddr: VirtAddr
}

pub static mut PAGING: Option<Paging> = None;

/// Kernel initialization function
///
/// # Safety
/// Initialization of many things is unsafe, so kernel_init is unsafe too
pub unsafe fn kernel_init(boot_info: &'static mut bootloader_api::BootInfo) {
    let rsdp_addr: u64 = *boot_info.rsdp_addr.as_ref().unwrap();
    let phys_mem_offset = *boot_info.physical_memory_offset.as_ref().unwrap();
    let phys_mem_virtaddr = VirtAddr::new(phys_mem_offset);
    let ramdisk_addr = boot_info.ramdisk_addr.as_ref().unwrap();
    let ramdisk_size = boot_info.ramdisk_len as usize;
    let mut mapper = unsafe { memory::init(phys_mem_virtaddr) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_regions) };
    serial_println!("init: phys_mem_virtaddr={:?}", phys_mem_virtaddr);
    mm::allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
     
    gdt::init();    
    idt::init();
    userspace::init_syscalls();
    idt::PICS.lock().initialize();
    x86_64::instructions::interrupts::enable();
    
    for i in 0..1000 {
        memory::create_mapping(
            Page::containing_address(VirtAddr::new(0x80000000 + i * 0x1000)),
            0x80000000 + i * 0x1000, 
            &mut mapper,
            &mut frame_allocator,
            PageTableFlags::USER_ACCESSIBLE | PageTableFlags::WRITABLE | PageTableFlags::PRESENT);
    }   

    unsafe {
        MEMORY_INFO = Some(MemoryInfo {
            boot_info,
            physical_memory_offset: phys_mem_virtaddr,
            frame_allocator: frame_allocator.clone(),
            kernel_l4_table: mapper.level_4_table().clone(),
            kernel_cr3: Cr3::read().0
        }) 
    };

    let mut paging = Paging {
        mapper,
        frame_allocator,
        memory_regions: &boot_info.memory_regions,
        rsdp_addr,
        phys_mem_virtaddr
    };

    serial_println!("init: ramdisk addr is {:#016x}", ramdisk_addr);
    serial_println!("init: cr3={:?}", Cr3::read());
    serial_println!("init: rsdp_addr={:#016x}", rsdp_addr);
    ramdisk::init(*ramdisk_addr, ramdisk_size, &mut paging);
}
