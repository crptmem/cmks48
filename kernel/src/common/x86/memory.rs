extern crate alloc;
use bootloader_api::{info::{MemoryRegionKind, MemoryRegions}, BootInfo};
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame, Size4KiB
    },
    PhysAddr, VirtAddr,
};
use core::option::Option;
use core::option::Option::None;
use core::iter::Iterator;

#[derive(Clone)]
pub struct MemoryInfo {
    pub boot_info: &'static BootInfo,
    pub physical_memory_offset: VirtAddr,
    pub frame_allocator: BootInfoFrameAllocator,
    pub kernel_l4_table: PageTable,
    pub kernel_cr3: PhysFrame
}

/// Store BootInfo struct and other useful things for later use
/// This is set in the init() function and should not be
/// modified after that.
pub static mut MEMORY_INFO: Option<MemoryInfo> = None;

/// Initialize a new OffsetPageTable.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

/// Create a new page table
pub fn create_empty_pagetable() -> (*mut PageTable, u64) {
    // Need to borrow as mutable so that we can allocate new frames
    // and so modify the frame allocator
    let memory_info = unsafe {MEMORY_INFO.as_mut().unwrap()};

    // Get a frame to store the level 4 table
    let level_4_table_frame = memory_info.frame_allocator.allocate_frame().unwrap();
    let phys = level_4_table_frame.start_address(); // Physical address
    let virt = memory_info.physical_memory_offset + phys.as_u64(); // Kernel virtual address
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    // Clear all entries in the page table
    unsafe {
        (*page_table_ptr).zero();
    }

    (page_table_ptr, phys.as_u64())
}

fn copy_pagetables(level_4_table: &PageTable) -> (*mut PageTable, u64) {
    // Create a new level 4 pagetable
    let (table_ptr, table_physaddr) = create_empty_pagetable();
    let table = unsafe {&mut *table_ptr};

    fn copy_pages_rec(physical_memory_offset: VirtAddr,
                      from_table: &PageTable, to_table: &mut PageTable,
                      level: u16) {
        for (i, entry) in from_table.iter().enumerate() {
            if !entry.is_unused() {
                if (level == 1) || entry.flags().contains(PageTableFlags::HUGE_PAGE) {
                    // Maps a frame, not a page table
                    to_table[i].set_addr(entry.addr(), entry.flags());
                } else {
                    // Create a new table at level - 1
                    let (new_table_ptr, new_table_physaddr) = create_empty_pagetable();
                    let to_table_m1 = unsafe {&mut *new_table_ptr};

                    // Point the entry to the new table
                    to_table[i].set_addr(PhysAddr::new(new_table_physaddr),
                                         entry.flags());

                    // Get reference to the input level-1 table
                    let from_table_m1 = {
                        let virt = physical_memory_offset + entry.addr().as_u64();
                        unsafe {& *virt.as_ptr()}
                    };

                    // Copy level-1 entries
                    copy_pages_rec(physical_memory_offset, from_table_m1, to_table_m1, level - 1);
                }
            }
        }
    }

    let memory_info = unsafe {MEMORY_INFO.as_mut().unwrap()};
    copy_pages_rec(memory_info.physical_memory_offset, level_4_table, table, 4);

    return (table_ptr, table_physaddr)
}

pub fn create_new_user_pagetable() -> (*mut PageTable, u64) {
    let memory_info = unsafe {MEMORY_INFO.as_mut().unwrap()};

    // Copy kernel pages
    let (user_page_table_ptr, user_page_table_physaddr) =
        copy_pagetables(&memory_info.kernel_l4_table);

    // Add KernelInfo page
    let memory_info = unsafe {MEMORY_INFO.as_mut().unwrap()};
    let mut mapper = unsafe {
        OffsetPageTable::new(&mut *user_page_table_ptr,
                             memory_info.physical_memory_offset)};

    (user_page_table_ptr, user_page_table_physaddr)
}

pub fn allocate_pages_mapper(
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    mapper: &mut impl Mapper<Size4KiB>,
    start_addr: VirtAddr,
    size: u64,
    flags: PageTableFlags)
    -> Result<(), MapToError<Size4KiB>> {

    let page_range = {
        let end_addr = start_addr + size - 1u64;
        let start_page = Page::containing_address(start_addr);
        let end_page = Page::containing_address(end_addr);
        Page::range_inclusive(start_page, end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        unsafe {
            mapper.map_to(page,
                          frame,
                          flags,
                          frame_allocator)?.flush()
        };
    }

    Ok(())
}

pub fn allocate_pages(level_4_table: *mut PageTable,
                      start_addr: VirtAddr,
                      size: u64,
                      flags: PageTableFlags)
                      -> Result<(), MapToError<Size4KiB>> {

    let memory_info = unsafe {MEMORY_INFO.as_mut().unwrap()};

    let mut mapper = unsafe {
        OffsetPageTable::new(&mut *level_4_table,
                             memory_info.physical_memory_offset)};

    allocate_pages_mapper(
        &mut memory_info.frame_allocator,
        &mut mapper,
        start_addr, size, flags)
}

pub fn create_mapping(
    page: Page,
    address: u64,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    flags: PageTableFlags
) {
    let frame = PhysFrame::containing_address(PhysAddr::new(address));
    let map_to_result = unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}


/// A FrameAllocator that always returns `None`.
pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

/// A FrameAllocator that returns usable frames from the bootloader's memory map.
#[derive(Clone, Debug)]
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryRegions,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Create a FrameAllocator from the passed memory map.
    ///
    /// This function is unsafe because the caller must guarantee that the passed
    /// memory map is valid. The main requirement is that all frames that are marked
    /// as `USABLE` in it are really unused.
    pub unsafe fn init(memory_map: &'static MemoryRegions) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// Returns an iterator over the usable frames specified in the memory map.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // get usable regions from memory map
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.kind == MemoryRegionKind::Usable);
        // map each region to its address range
        let addr_ranges = usable_regions.map(|r| r.start..r.end);
        // transform to an iterator of frame start addresses
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(0x1000));
        // create `PhysFrame` types from the start addresses
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
