extern crate lazy_static;
extern crate alloc;

use crate::init::ramdisk::{self};
use crate::task::userspace;
use crate::{mm, serial_println};
use crate::common::x86::{gdt, idt, memory};
use bootloader_api::info::MemoryRegions;
use x86_64::structures::paging::FrameAllocator;
use x86_64::{structures::paging::OffsetPageTable, VirtAddr, registers::control::Cr3, structures::paging::Page};

#[derive(Debug)]
pub struct Paging {
    pub frame_allocator: memory::BootInfoFrameAllocator,
    pub mapper: OffsetPageTable<'static>,
    pub memory_regions: &'static MemoryRegions,
    pub rsdp_addr: u64
}

pub static mut PAGING: Option<Paging> = None;

pub fn kernel_init(boot_info: &'static mut bootloader_api::BootInfo) {
    let rsdp_addr: u64 = *boot_info.rsdp_addr.as_ref().unwrap();
    let phys_mem_offset = *boot_info.physical_memory_offset.as_ref().unwrap();
    let phys_mem_virtaddr = VirtAddr::new(phys_mem_offset);
    let ramdisk_addr = boot_info.ramdisk_addr.as_ref().unwrap();
    let ramdisk_size = boot_info.ramdisk_len as usize;
    let mut mapper = unsafe { memory::init(phys_mem_virtaddr) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_regions) };

    mm::allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
     
    gdt::init();    
    idt::init();
    unsafe { userspace::init_syscalls(); }
    unsafe { idt::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    let paging = Paging {
        mapper,
        frame_allocator,
        memory_regions: &boot_info.memory_regions,
        rsdp_addr
    };

    unsafe { PAGING = Some(paging); }
    unsafe {
        x86_64::instructions::port::PortWrite::write_to_port(0xb2, 0xffu8);
    }
    serial_println!("init: ramdisk addr is {:#016x}", ramdisk_addr);
    serial_println!("init: cr3={:?}", Cr3::read());
    serial_println!("init: rsdp_addr={:#016x}", rsdp_addr);
    unsafe { ramdisk::init(*ramdisk_addr, ramdisk_size, &mut PAGING.as_mut().unwrap()); }
}
