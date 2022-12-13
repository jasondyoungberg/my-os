-include qemu.config
-include gcc.config

.PHONY: all
all: os.iso

.PHONY: run
run: os.iso
	@echo Running qemu with $<
	@qemu-system-x86_64 $(QEMU_FLAGS) -cdrom os.iso

.PHONY: run-fast
run-fast: os.iso
	@echo Running qemu with $< and kvm
	@qemu-system-x86_64 $(QEMU_FLAGS) --enable-kvm -cdrom os.iso

.PHONY: clean
clean:
	@echo Cleaning up files
	@rm -rf .iso os.iso
	@make -C kernel clean --no-print-directory


### Dependencies ###

limine/:
	@echo Downloading limine
	@git clone https://github.com/limine-bootloader/limine.git \
		--branch=v4.x-branch-binary --depth=1 --quiet

	@echo Compiling limine
	@make -C limine --no-print-directory --quiet
	@echo


### Compile images ###

os.iso: kernel/kernel.elf limine.cfg limine/
	@echo Setting up iso directory
	@rm -rf .iso
	@mkdir -p .iso/boot/

	@cp kernel/kernel.elf         .iso/kernel.elf
	@cp limine/limine-cd-efi.bin  .iso/boot/limine-cd-efi.bin
	@cp limine/limine-cd.bin      .iso/boot/limine-cd.bin
	@cp limine/limine.sys         .iso/boot/limine.sys
	@cp limine.cfg                .iso/boot/limine.cfg

	@echo Creating iso
	@xorriso -as mkisofs -b boot/limine-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine-cd-efi.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		.iso -o $@ 1>/dev/null 2>/dev/null

	@echo Deploying limine
	@limine/limine-deploy $@ 1>/dev/null 2>/dev/null
	@echo

kernel/kernel.elf: kernel/*.c kernel/*.h limine/
	@echo Compiling kernel.elf
	@make -C kernel --no-print-directory
	@echo