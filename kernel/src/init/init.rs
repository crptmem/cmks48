extern crate lazy_static;
extern crate alloc;

use core::mem::transmute;
use crate::init::ramdisk::{self};
use crate::{mm, serial_println, task, video};
use crate::common::x86::{gdt, idt, memory};
use bootloader_api::info::MemoryRegions;
use x86_64::{structures::paging::OffsetPageTable, VirtAddr, registers::control::Cr3};
use crate::task::executor::Executor;

pub static mut MAPPER: Option<OffsetPageTable<'static>> = None;
pub static mut FRAME_ALLOCATOR: Option<memory::BootInfoFrameAllocator> = None;

pub struct Paging {
    pub frame_allocator: memory::BootInfoFrameAllocator,
    pub mapper: OffsetPageTable<'static>,
    pub memory_regions: &'static MemoryRegions
}

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
    unsafe { idt::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    video::gop::init(boot_info.framebuffer.as_ref().unwrap().info());
    
    let mut paging = Paging {
        frame_allocator: frame_allocator.clone(),
        mapper,
        memory_regions: &boot_info.memory_regions
    };
    
    serial_println!("init: ramdisk addr is {:#016x}", ramdisk_addr);
    serial_println!("init: cr3={:?}", Cr3::read());
    ramdisk::init(*ramdisk_addr, ramdisk_size, &mut paging);

    let putpix: extern "C" fn(u64, u64, u64) = unsafe { transmute(task::elf::get_symbol_ptr(b"putpix")) };
    for x in 200..500 {
        for y in 200..300 {
            (putpix)(x, y, x * y / 2 * y);
        }
    }

    let mut executor = Executor::new();
    //executor.spawn(Task::new();
    //executor.run();
    //acpi::init(rsdp_addr);
    //pci::init();
}
