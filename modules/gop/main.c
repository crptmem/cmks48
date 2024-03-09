#include <stdint.h>
#include <stdarg.h>
#include <string.h>
#include "lib.h"

void put_pixel(uint64_t x, uint64_t y, uint64_t colour) {
  uint64_t *video_memory = 0x0000000080000000;
  *(uint64_t *)((uint64_t)video_memory + (x * 4) + (y * 1280 * 4)) = colour;
}

void _start() { 
  for(int x = 0; x < 1280; x++) {
    for(int y = 0; y < 799; y++) {
      put_pixel(x, y, 0x000000);
    }
  }
  exit(80);
}
