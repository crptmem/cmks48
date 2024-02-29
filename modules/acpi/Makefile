all:
	gcc lib.c -c -o lib.o -nostdlib -ffreestanding
	gcc main.c -c -o main.o -nostdlib -ffreestanding
	ld -melf_x86_64 main.o lib.o -o main.elf -T../link.ld

install: all
	cp main.elf ../../ramdisk/modules/acpi
