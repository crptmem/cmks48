#pragma once
#include <lib/stdlib.h>

typedef struct _registers {
    // Extra registers (how to get):
    // flags - make64(low32(flags), high32(flags))
    // rax   - make64(low32(rax64), high32(rax64))
    // bax   - make64(low32(rbx64), high32(rbx64))
    // cax   - make64(low32(rcx64), high32(rcx64))
    // dax   - make64(low32(rdx64), high32(rdx64))
    uint32_t eax32, ebx32, ecx32, edx32;
    uint64_t rax64, rbx64, rcx64, rdx64;
    uint64_t flags;
} registers;

extern registers* dump_registers();
extern void StackTrace(registers* regs);