#include <video/gop.h>

void _start(Framebuffer* framebuffer, PSF1_FONT* psf1_font){
    CursorPosition.X = 10;
    CursorPosition.Y = 10;
    
    Print(framebuffer,psf1_font, 0xFFFFFF, "AirKernel 0.1-indev");
    kprint(" - nothing to do...", 0xFFFFFF, "_start - kernel.c");
    return ;
}