use core::arch::asm;
use crate::common::x86::gdt;
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

