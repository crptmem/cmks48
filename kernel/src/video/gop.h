#pragma once
#include <lib/stdlib.h>
typedef struct {
	void* BaseAddress;
	size_t BufferSize;
	unsigned int Width;
	unsigned int Height;
	unsigned int PixelsPerScanLine;
} Framebuffer;

typedef struct {
	unsigned char magic[2];
	unsigned char mode;
	unsigned char charsize;
} PSF1_HEADER;

typedef struct {
	PSF1_HEADER* psf1_Header;
	void* glyphBuffer;
} PSF1_FONT;

typedef struct {
    unsigned int X;
    unsigned int Y;
} Point;

extern Point CursorPosition;
extern void Print(Framebuffer* framebuffer, PSF1_FONT* psf1_font, unsigned int colour, char* str);
extern void Clear();
extern void kprint(char* str[], unsigned int color);
extern void newline();
extern void PutChar(char* chr, unsigned int colour, unsigned int x, unsigned int y);
extern const char* to_string(uint64_t value);
extern const char* to_hstring64(uint64_t value);
extern const char* to_hstring32(uint32_t value);
extern const char* to_hstring16(uint16_t value);
extern const char* to_hstring8(uint8_t value);