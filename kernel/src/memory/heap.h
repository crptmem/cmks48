
#ifndef __MEMORY_H_
#define __MEMORY_H_
#include <lib/stdlib.h>

typedef struct {
	uint8_t status;
	uint32_t size;
} alloc_t;

extern void mm_init(uint32_t kernel_end);

extern char* malloc(size_t size);
extern void free(void *mem); /* will add soon */

#endif
