/*
* Filename: kernel.c
* Author: corruptmemory (zmemory)
* Description: Kernel main file
*/

#include <memory/heap.h>
#include <video/gop.h>
#include <hardware/IO.h>
#include <debug/panic.h>
#include <hardware/rtc.h>
typedef struct{
  	int height;
  	int width;
    uint64_t kernelEnd;
} kernelData;

void _initkernel(Framebuffer* framebuffer, PSF1_FONT* psf1_font, kernelData KernelInfo){
    CursorPosition.X = 10; /* Set default cursor position */
    CursorPosition.Y = 10;
    datetime_t BootTime = rtc_read_datetime(); /* Kernel Startup time */
    Print(framebuffer,psf1_font, 0xFFFFFF, "AirKernel 0.1-indev");
    newline();
    kprint("Boot Time: ", 0xFFFFFF);
    kprint(datetime_to_str(&BootTime), 0xFFFFFF);
    newline();
    kprint("kernelEnd: 0x", 0xFFFFFF);
    kprint(to_hstring64(KernelInfo.kernelEnd), 0xFFFFFF);
    newline();
    mm_init(KernelInfo.kernelEnd);
    datetime_t BootTime2 = rtc_read_datetime(); /* Kernel Post-init time */
    kprint("Loaded in ", 0xFFFFFF);
    kprint(to_string64(BootTime.second - BootTime2.second), 0xFFFFFF);
    kprint("s.", 0xFFFFFF);
}


void _start(Framebuffer* framebuffer, PSF1_FONT* psf1_font, kernelData KernelInfo){
    _initkernel(framebuffer, psf1_font, KernelInfo);
    while (1);
}