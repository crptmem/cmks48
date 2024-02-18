#include <stdint.h>
#include <stdarg.h>
#include <string.h>
#include "lib.h"

void gop_putpix(uint32_t x, uint32_t y, uint32_t colour) {
  uint64_t *video_memory = 0x000f80000000;;
  *(uint64_t *)((uint64_t)video_memory + (x * 4) + (y * 1280 * 4)) = colour;
}

void gop_clear() {
  for(int x = 0; x <= 1280; x++) {
    for(int y = 0; y <= 720; y++) {
      gop_putpix(x, y, 0x0);
    }
  }
}

void _start(void (*symreg)(char[24], uint64_t *), uint64_t *(*getsym)(char[24])) {
  mprintf("gop: gop module v0.1 by corruptmemory  %s %s\n", __DATE__, __TIME__);
  gop_clear();
  for(int x = 0; x <= 1280; x++) {
    for(int y = 0; y < 720; y++) {
      gop_putpix(x, y, x);
    }
  }
}
