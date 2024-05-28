# Nuke built-in rules and variables.
override MAKEFLAGS += -rR

override IMAGE_NAME := myos

override OVMF_PATH := ovmf/OVMF.fd

# Convenience macro to reliably declare user overridable variables.
define DEFAULT_VAR =
    ifeq ($(origin $1),default)
        override $(1) := $(2)
    endif
    ifeq ($(origin $1),undefined)
        override $(1) := $(2)
    endif
endef

ifeq ($(RUST_PROFILE),)
    override RUST_PROFILE := release
endif

ifeq ($(CPUS),)
    override CPUS := 4
endif

ifeq ($(MEM),)
    override MEM := 2G
endif

QEMU_ARGS := \
	-machine q35 \
	-smp $(CPUS) \
	-m $(MEM) \
	-debugcon stdio \
	--no-reboot \
	--no-shutdown \

ifeq ($(KVM),1)
	QEMU_ARGS += -enable-kvm
endif

.PHONY: all
all: $(IMAGE_NAME).iso

.PHONY: all-hdd
all-hdd: $(IMAGE_NAME).hdd

.PHONY: debug
debug: $(IMAGE_NAME).iso
	@echo "Launching QEMU in debug mode..."
	qemu-system-x86_64 $(QEMU_ARGS) -cdrom $(IMAGE_NAME).iso -boot d \
		-d int,cpu_reset,unimp,guest_errors

.PHONY: run
run: $(IMAGE_NAME).iso
	@echo "Launching QEMU..."
	qemu-system-x86_64 $(QEMU_ARGS) -cdrom $(IMAGE_NAME).iso -boot d

.PHONY: run-uefi
run-uefi: ovmf $(IMAGE_NAME).iso
	@echo "Launching QEMU..."
	qemu-system-x86_64 $(QEMU_ARGS) -cdrom $(IMAGE_NAME).iso -boot d \
		-drive if=pflash,format=raw,readonly=on,file=$(OVMF_PATH)

.PHONY: run-hdd
run-hdd: $(IMAGE_NAME).hdd
	@echo "Launching QEMU..."
	qemu-system-x86_64 $(QEMU_ARGS) -hda $(IMAGE_NAME).hdd

.PHONY: run-hdd-uefi
run-hdd-uefi: ovmf $(IMAGE_NAME).hdd
	@echo "Launching QEMU..."
	qemu-system-x86_64 $(QEMU_ARGS) -hda $(IMAGE_NAME).hdd \
		-drive if=pflash,format=raw,readonly=on,file=$(OVMF_PATH)

ovmf:
	mkdir -p ovmf
	cd ovmf && curl -Lo OVMF.fd https://retrage.github.io/edk2-nightly/bin/RELEASEX64_OVMF.fd

limine/limine:
	rm -rf limine
	git clone https://github.com/limine-bootloader/limine.git --branch=v7.x-binary --depth=1
	$(MAKE) -C limine

.PHONY: kernel
kernel:
	@echo "Building the kernel..."
	
	cd kernel/app && find . -name '*.asm' -exec nasm {} \;

	mkdir -p kernel/dist
	cd kernel && cargo build --profile $(RUST_PROFILE) -Z unstable-options --out-dir dist

$(IMAGE_NAME).iso: limine/limine kernel
	@echo "Generating the ISO image..."
	rm -rf iso_root
	mkdir -p iso_root/boot
	cp kernel/dist/kernel iso_root/boot/
	mkdir -p iso_root/boot/limine
	cp limine.cfg limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin iso_root/boot/limine/
	mkdir -p iso_root/EFI/BOOT
	cp limine/BOOTX64.EFI iso_root/EFI/BOOT/
	cp limine/BOOTIA32.EFI iso_root/EFI/BOOT/
	cp files/* iso_root/
	xorriso -as mkisofs -quiet \
		-b boot/limine/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o $(IMAGE_NAME).iso 2> /dev/null
	./limine/limine bios-install $(IMAGE_NAME).iso --quiet
	rm -rf iso_root

$(IMAGE_NAME).hdd: limine/limine kernel
	@echo "Generating the HDD image..."
	rm -f $(IMAGE_NAME).hdd
	dd if=/dev/zero bs=1M count=0 seek=64 of=$(IMAGE_NAME).hdd status=none
	sgdisk $(IMAGE_NAME).hdd -n 1:2048 -t 1:ef00 2> /dev/null
	./limine/limine bios-install $(IMAGE_NAME).hdd --quiet
	mformat -i $(IMAGE_NAME).hdd@@1M
	mmd -i $(IMAGE_NAME).hdd@@1M ::/EFI ::/EFI/BOOT ::/boot ::/boot/limine
	mcopy -i $(IMAGE_NAME).hdd@@1M kernel/dist/kernel ::/boot
	mcopy -i $(IMAGE_NAME).hdd@@1M limine.cfg limine/limine-bios.sys ::/boot/limine
	mcopy -i $(IMAGE_NAME).hdd@@1M limine/BOOTX64.EFI ::/EFI/BOOT
	mcopy -i $(IMAGE_NAME).hdd@@1M limine/BOOTIA32.EFI ::/EFI/BOOT

.PHONY: clean
clean:
	rm -rf iso_root $(IMAGE_NAME).iso $(IMAGE_NAME).hdd
	rm -rf kernel/kernel

.PHONY: distclean
distclean: clean
	rm -rf limine ovmf
	rm -rf kernel/kernel
