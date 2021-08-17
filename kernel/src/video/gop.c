#include <video/gop.h>
#include <lib/stdlib.h>
#include <stdarg.h>
#include <limits.h>

Framebuffer* fb;
PSF1_FONT* font;
void putChar(Framebuffer* framebuffer, PSF1_FONT* psf1_font, unsigned int colour, char chr, unsigned int xOff, unsigned int yOff)
{
    unsigned int* pixPtr = (unsigned int*)framebuffer->BaseAddress;
    char* fontPtr = psf1_font->glyphBuffer + (chr * psf1_font->psf1_Header->charsize);
    for (unsigned long y = yOff; y < yOff + 16; y++){
        for (unsigned long x = xOff; x < xOff+8; x++){
            if ((*fontPtr & (0b10000000 >> (x - xOff))) > 0){
                    *(unsigned int*)(pixPtr + x + (y * framebuffer->PixelsPerScanLine)) = colour;
                }

        }
        fontPtr++;
    }
}

void PutChar(char* chr, unsigned int colour, unsigned int x, unsigned int y){
    putChar(fb, font, colour, *chr, x, y);
 }

Point CursorPosition;
void Print(Framebuffer* framebuffer, PSF1_FONT* psf1_font, unsigned int colour, char* str)
{
    fb = framebuffer;
    font = psf1_font;
    char* chr = str;
    while(*chr != 0){
        putChar(framebuffer, psf1_font, colour, *chr, CursorPosition.X, CursorPosition.Y);
        CursorPosition.X+=8;
        if(CursorPosition.X + 8 > framebuffer->Width)
        {
            CursorPosition.X = 0;
            CursorPosition.Y += 18;
        }
        chr++;
    }
}


void Clear(){
    uint64_t fbBase = (uint64_t)fb->BaseAddress;
    uint64_t bytesPerScanline = fb->PixelsPerScanLine * 4;
    uint64_t fbHeight = fb->Height;
    uint64_t fbSize = fb->BufferSize;

    for (int verticalScanline = 0; verticalScanline < fbHeight; verticalScanline ++){
        uint64_t pixPtrBase = fbBase + (bytesPerScanline * verticalScanline);
        for (uint32_t* pixPtr = (uint32_t*)pixPtrBase; pixPtr < (uint32_t*)(pixPtrBase + bytesPerScanline); pixPtr ++){
            *pixPtr = 0xFFFFFF;
        }
    }
}

void newline(){
    CursorPosition.X = 10;
    CursorPosition.Y += 16;
}

void kprint(char* str[], unsigned int color){
    Print(fb,font, color, str);
}

