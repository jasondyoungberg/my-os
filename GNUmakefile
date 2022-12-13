-include qemu.config

.PHONY: all
all: os.iso

.PHONY: run
run: os.iso
	qemu-system-x86_64 $(QEMU_FLAGS) -cdrom os.iso

.PHONY: clean
clean:
	make -C kernel clean
	rm -rf .iso os.iso

### Compile images ###

os.iso: kernel/kernel.elf
	rm -rf .iso
	mkdir -p .iso/boot/

	cp kernel/kernel.elf         .iso/kernel.elf
	cp limine/limine-cd-efi.bin  .iso/boot/limine-cd-efi.bin
	cp limine/limine-cd.bin      .iso/boot/limine-cd.bin
	cp limine/limine.sys         .iso/boot/limine.sys
	cp limine.cfg                .iso/boot/limine.cfg

	xorriso -as mkisofs -b boot/limine-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine-cd-efi.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		.iso -o $@

	limine/limine-deploy $@

kernel/kernel.elf: kernel/*
	make -C kernel