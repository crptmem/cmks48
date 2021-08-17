#include <lib/stdlib.h>
#include <video/gop.h>
#include <string.h>
#include <memory/heap.h>

#define MAX_PAGE_ALIGNED_ALLOCS 32

uint32_t last_alloc = 0;
uint32_t heap_end = 0;
uint32_t heap_begin = 0;
uint32_t pheap_begin = 0;
uint32_t pheap_end = 0;
uint8_t *pheap_desc = 0;
uint32_t memory_used = 0;

void mm_init(uint32_t kernel_end)
{
	last_alloc = kernel_end + 0x1000;
	heap_begin = last_alloc;
	pheap_end = 0x400000;
	pheap_begin = pheap_end - (MAX_PAGE_ALIGNED_ALLOCS * 4096);
	heap_end = pheap_begin;
        pheap_desc = (uint8_t *)malloc(MAX_PAGE_ALIGNED_ALLOCS);
        newline();
        kprint("Kernel last_alloc: ", 0xFFFFFF);
	kprint(to_hstring32(last_alloc), 0xFFFFFF);
        newline();
}

void free(void *mem)
{
	alloc_t *alloc = (mem - sizeof(alloc_t));
	memory_used -= alloc->size + sizeof(alloc_t);
	alloc->status = 0;
}

char* malloc(size_t size)
{
	if(!size) return 0;
	uint8_t *mem = (uint8_t *)heap_begin;
	while((uint32_t)mem < last_alloc)
	{
		alloc_t *a = (alloc_t *)mem;
		if(!a->size)
			goto nalloc;
		if(a->status) {
			mem += a->size;
			mem += sizeof(alloc_t);
			mem += 4;
			continue;
		}
		if(a->size >= size)
		{
			a->status = 1;

			memset(mem + sizeof(alloc_t), 0, size);
			memory_used += size + sizeof(alloc_t);
			return (char *)(mem + sizeof(alloc_t));
		}
		mem += a->size;
		mem += sizeof(alloc_t);
		mem += 4;
	}

	nalloc:;
	alloc_t *alloc = (alloc_t *)last_alloc;
	alloc->status = 1;
	alloc->size = size;

	last_alloc += size;
	last_alloc += sizeof(alloc_t);
	last_alloc += 4;
   	kprint("Allocated ", 0xFFFFFF);
   	kprint(to_string64(size), 0xFFFFFF);
   	kprint(" bytes from 0x", 0xFFFFFF);
   	kprint(to_hstring32((uint32_t)alloc + sizeof(alloc_t)), 0xFFFFFF);
   	kprint(" to 0x", 0xFFFFFF);
   	kprint(to_hstring32(last_alloc), 0xFFFFFF);
	memory_used += size + 4 + sizeof(alloc_t);
	memset((char *)((uint32_t)alloc + sizeof(alloc_t)), 0, size);
	return (char *)((uint32_t)alloc + sizeof(alloc_t));
}
