/*
* Filename: kernel.c
* Author: corruptmemory (zmemory)
* Description: Kernel main file
*/

#include <video/gop.h>
#include <hardware/IO.h>
#include <debug/panic.h>
#include <hardware/rtc.h>
typedef struct{
  	int height;
  	int width;
	char* text;
} kernelData;

void _start(Framebuffer* framebuffer, PSF1_FONT* psf1_font, kernelData KernelInfo){
    CursorPosition.X = 10;
    CursorPosition.Y = 10;
    datetime_t BootTime = rtc_read_datetime();
    Print(framebuffer,psf1_font, 0xFFFFFF, "AirKernel 0.1-indev");
    newline();
    kprint("Boot Time: ", 0xFFFFFF);
    kprint(datetime_to_str(&BootTime), 0xFFFFFF);
    //panic("Test Panic", dump_registers());
    while (1);
}