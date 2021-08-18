#include <video/gop.h>
#include <debug/stack.h>
void panic(const char* message, registers* regs){
    newline();
    kprint("--[ Kernel panic: ", 0xFFFFFF);
    kprint(message, 0xFFFFFF);
    kprint(" --]", 0xFFFFFF);
    newline();
    kprint("Context: ", 0xFFFFFF);
    newline();
    kprint("EAX32: 0x", 0xFFFFFF);
    kprint(to_hstring32(regs->eax32), 0xFFFFFF);
    kprint("  EBX32: 0x", 0xFFFFFF);
    kprint(to_hstring32(regs->ebx32), 0xFFFFFF);
    kprint("  ECX32: 0x", 0xFFFFFF);
    kprint(to_hstring32(regs->ecx32), 0xFFFFFF);
    kprint("  EDX32: 0x", 0xFFFFFF);
    kprint(to_hstring32(regs->edx32), 0xFFFFFF);
    for(;;);
}