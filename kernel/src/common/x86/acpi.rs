use crate::serial_println;
use crate::init::init::{FRAME_ALLOCATOR,MAPPER};
use core::borrow::Borrow;
use core::format_args;
use core::concat;
use acpi::AcpiTables;
use acpi::PhysicalMapping;
use x86_64::structures::paging::Mapper;
use x86_64::structures::paging::Size4KiB;
use core::ptr::NonNull;
use x86_64::PhysAddr;
use x86_64::VirtAddr;
use x86_64::structures::paging::Page;
use x86_64::structures::paging::PhysFrame;
use acpi::AcpiHandler;
use crate::BootInfoFrameAllocator;

#[derive(Clone)]
pub struct CMKSAcpiHandler;

impl AcpiHandler for CMKSAcpiHandler {
    unsafe fn map_physical_region<T>(
        &self,
        addr: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        
        let page: x86_64::structures::paging::Page<Size4KiB> = Page::containing_address(VirtAddr::new(addr as u64));
        use x86_64::structures::paging::PageTableFlags as Flags;

        let frame = PhysFrame::containing_address(PhysAddr::new(addr.try_into().unwrap()));
        let flags = Flags::PRESENT | Flags::WRITABLE;

        unsafe {
            MAPPER.as_mut().unwrap().map_to(page, frame, flags, &mut <Option<BootInfoFrameAllocator> as Clone>::clone(&FRAME_ALLOCATOR.borrow()).unwrap());
        };
        PhysicalMapping::new(addr, NonNull::new(addr as *mut _).unwrap(), size, size, Self)
    }

    fn unmap_physical_region<T>(_region: &PhysicalMapping<Self, T>) {}
}

pub fn init(rsdp_addr: u64) {
    serial_println!("ACPI: rsdp address is {:#016x}", rsdp_addr);
    let _ = unsafe { AcpiTables::from_rsdp(CMKSAcpiHandler, rsdp_addr.try_into().unwrap()); };
}
