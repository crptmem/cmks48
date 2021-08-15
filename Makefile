

all:
	@echo Building bootloader...
	make -C ./gnu-efi
	make -C ./gnu-efi/lib
	make -C ./gnu-efi/gnuefi
	make -C ./gnu-efi/bootloader
	mv gnu-efi/bootloader/main.efi gnu-efi/x86_64/bootloader
	@echo Building kernel...
	make -C ./kernel
	@echo Building image...
	make -C ./kernel buildimg
	@echo Done

run:
	make -C ./kernel run
