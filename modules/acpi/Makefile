all:
	gcc main.c -O0 -c -o main.o -nostdlib -ffreestanding -Os
	ld -melf_x86_64 main.o -o main.elf -T../link.ld

install: all
	cp main.elf ../../ramdisk/modules/acpi
