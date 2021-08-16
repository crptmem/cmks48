#include <video/gop.h>
#include <hardware/IO.h>
#include <debug/stack.h>

typedef struct{
  	int height;
  	int width;
	char* text;
} kernelData;
char hexTo_StringOutput32[128];
const char* to_hstring(uint32_t value){
    uint32_t* valPtr = &value;
    uint8_t* ptr;
    uint8_t tmp;
    uint8_t size = 4 * 2 - 1;
    for (uint8_t i = 0; i < size; i++){
        ptr = ((uint8_t*)valPtr + i);
        tmp = ((*ptr & 0xF0) >> 4);
        hexTo_StringOutput32[size - (i * 2 + 1)] = tmp + (tmp > 9 ? 55 : '0');
        tmp = ((*ptr & 0x0F));
        hexTo_StringOutput32[size - (i * 2)] = tmp + (tmp > 9 ? 55 : '0');
    }
    hexTo_StringOutput32[size + 1] = 0;
    return hexTo_StringOutput32;
}
void _start(Framebuffer* framebuffer, PSF1_FONT* psf1_font, kernelData KernelInfo){
    CursorPosition.X = 10;
    CursorPosition.Y = 10;
    registers* regs = dump_registers();
    Print(framebuffer,psf1_font, 0xFFFFFF, "AirKernel 0.1-indev");
    newline();
    newline();
    Print(framebuffer,psf1_font, 0xFFFFFF, "registers:");
    newline();
    kprint("EAX32: 0x", 0xFFFFFF, "_start - kernel.c");
    kprint(to_hstring(regs->eax32), 0xFFFFFF, "_start - kernel.c");
    newline();
    kprint("EBX32: 0x", 0xFFFFFF, "_start - kernel.c");
    kprint(to_hstring(regs->ebx32), 0xFFFFFF, "_start - kernel.c");
    newline();
    kprint("ECX32: 0x", 0xFFFFFF, "_start - kernel.c");
    kprint(to_hstring(regs->ecx32), 0xFFFFFF, "_start - kernel.c");
    newline();
    kprint("EDX32: 0x", 0xFFFFFF, "_start - kernel.c");
    kprint(to_hstring(regs->edx32), 0xFFFFFF, "_start - kernel.c");
    newline();
    while (1);
}