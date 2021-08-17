#include <debug/stack.h>
#include <video/gop.h>
uint64_t make64(uint32_t high, uint32_t low) {
    return (uint64_t) high << 32 | low;
}

uint32_t high32(uint64_t value) {
   return value >> 32;
}

uint32_t low32(uint64_t value) {
    return value;
}


registers* dump_registers() {
    // Structs
    registers* reg;

    // Setting registers
    asm (
         "movl %%eax, %[a1] ;"
         "movl %%ebx, %[b1] ;"
         "movl %%ecx, %[c1] ;"
         "movl %%edx, %[d1] ;"

         "movq %%rax, %[a2] ;"
         "movq %%rbx, %[b2] ;"
         "movq %%rcx, %[c2] ;"
         "movq %%rdx, %[d2] ;"
         :
         [a1] "=m" (reg->eax32), [b1] "=m" (reg->ebx32), [c1] "=m" (reg->ecx32), [d1] "=m" (reg->edx32), 
         [a2] "=m" (reg->rax64), [b2] "=m" (reg->rbx64), [c2] "=m" (reg->rcx64), [d2] "=m" (reg->rdx64)
         );

    asm (
         "pushfq        ;"
         "pop %[f1] ;"
         :
         [f1] "=m" (reg->flags)
         );

    if(reg->flags & (1 << 0)) return; // Carry
    if(reg->flags & (1 << 2)) return; // Parity
    if(reg->flags & (1 << 4)) return; // Adjust
    if(reg->flags & (1 << 6)) return; // Zero
    if(reg->flags & (1 << 7)) return; // Sign
    if(reg->flags & (1 << 11)) return; // Overflow

    return reg;
}

void StackTrace(registers* regs){
    newline();
    kprint("EAX32: 0x", 0xFFFFFF);
    kprint(to_hstring32(regs->eax32), 0xFFFFFF);
    kprint(" EBX32: 0x", 0xFFFFFF);
    kprint(to_hstring32(regs->ebx32), 0xFFFFFF);
    kprint(" ECX32: 0x", 0xFFFFFF);
    kprint(to_hstring32(regs->ecx32), 0xFFFFFF);
    kprint(" EDX32: 0x", 0xFFFFFF);
    kprint(to_hstring32(regs->edx32), 0xFFFFFF);
}