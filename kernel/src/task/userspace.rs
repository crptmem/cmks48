use core::arch::asm;

pub unsafe fn init_syscalls() {
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
