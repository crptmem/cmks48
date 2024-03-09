use core::arch::asm;
use crate::common::x86::gdt;
use crate::serial_println;
use x86_64::VirtAddr;

const MSR_STAR: usize = 0xC0000081;

pub unsafe fn init_syscalls() {
    asm!("\
        xor rax, rax
        mov rdx, 0x230008 // use seg selectors 8, 16 for syscall and 43, 51 for sysret
        wrmsr",
        in("rcx") MSR_STAR);

    asm!("\
        mov rcx, 0xC0000082
        wrmsr
        mov rcx, 0xC0000080
        rdmsr
        or rax, 1
        wrmsr
        mov rcx, 0xC0000081
        rdmsr
        mov rdx, 0x00180008
        wrmsr
         ")
}

pub unsafe fn syscall_handler() {
    let rax: u64;
    let rbx: u64;
    let rcx: u64;
    asm!("mov {:r}, rax", out(reg) rax);
    asm!("mov {:r}, rbx", out(reg) rbx);
    asm!("mov {:r}, rcx", out(reg) rcx);
    //serial_println!("syscall: rax={:#016x} rbx={:#016x} rcx={:#016x}", rax, rbx, rcx);
    match rbx {
        0x00 => {
            serial_println!("exiting user process with code {}", rcx);
        },
        0x01 => { // u8 memory write
            let memory_ptr = rcx as *mut u8;
            *memory_ptr = rax as u8;
        },
        0x02 => { // u16 memory write
            let memory_ptr = rcx as *mut u16;
            *memory_ptr = rax as u16;
        },
        0x03 => { // u32 memory write
            let memory_ptr = rcx as *mut u32;
            *memory_ptr = rax as u32;
        },
        0x04 => { // u64 memory write
            let memory_ptr = rcx as *mut u64;
            *memory_ptr = rax as u64;
        },
        1u64..u64::MAX => {},
        u64::MAX => {},
    }
}

pub unsafe fn jmp_to_usermode(code: VirtAddr, stack_end: VirtAddr) {
    let (cs_idx, ds_idx) = gdt::set_usermode_segs();
    
    x86_64::instructions::tlb::flush_all(); // flush the TLB after address-space switch
    asm!("\
        push rax   // stack segment
        push rsi   // rsp
        push 0x200 // rflags (only interrupt bit set)
        push rdx   // code segment
        push rdi   // ret to virtual addr
        iretq",
        in("rdi") code.as_u64(),
        in("rsi") stack_end.as_u64(),
        in("dx") cs_idx,
        in("ax") ds_idx);
    //:: "{rdi}"(code.addr()), "{rsi}"(stack_end.addr()), "{dx}"(cs_idx), "{ax}"(ds_idx) :: "intel", "volatile");
}

