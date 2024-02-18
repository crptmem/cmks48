#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(slice_pattern)]
#![feature(ascii_char)]
extern crate lazy_static;

use core::{borrow::Borrow, panic::PanicInfo};
use bootloader_api::config::Mapping;
use common::x86::memory::BootInfoFrameAllocator;
use x86_64::{structures::paging::OffsetPageTable, VirtAddr};
use crate::common::x86::serial;

pub mod common;
pub mod mm;
pub mod init;
pub mod task;
pub mod exec;

const FRAMEBUFFER: u64 = 0x000f_8000_0000;

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config.mappings.framebuffer = bootloader_api::config::Mapping::FixedAddress(FRAMEBUFFER);
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};
bootloader_api::entry_point!(kernel_main, config = &CONFIG);

pub static mut MAPPER: Option<OffsetPageTable<'static>> = None;
pub static mut FRAME_ALLOCATOR: Option<BootInfoFrameAllocator> = None;

fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    init::init::kernel_init(boot_info);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{:#?}", info);
    loop {}
}
