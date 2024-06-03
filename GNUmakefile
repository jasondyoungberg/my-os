# Nuke built-in rules and variables.
override MAKEFLAGS += -rR

override IMAGE_NAME := myos

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
    override RUST_PROFILE := dev
endif

override RUST_PROFILE_SUBDIR := $(RUST_PROFILE)
ifeq ($(RUST_PROFILE),dev)
    override RUST_PROFILE_SUBDIR := debug
endif

override RUST_TARGET_SUBDIR := x86_64-myos

ifeq ($(CPUS),)
	override CPUS := 4
endif

ifeq ($(MEM),)
	override MEM := 2G
endif

QEMU_ARGS := \
	-M q35 \
	-m $(MEM) \
	-smp $(CPUS) \
	-debugcon stdio \
	-D qemu.log \
	-d int,cpu_reset,unimp,guest_errors \

ifeq ($(KVM),1)
	QEMU_ARGS += -enable-kvm
endif

ifeq ($(UEFI),1)
	QEMU_ARGS += -bios ovmf/OVMF.fd
endif

.PHONY: all
all: $(IMAGE_NAME).iso

.PHONY: all-hdd
all-hdd: $(IMAGE_NAME).hdd

.PHONY: run
run: $(IMAGE_NAME).iso ovmf
	qemu-system-x86_64 $(QEMU_ARGS) -cdrom $(IMAGE_NAME).iso -boot d

.PHONY: debug
debug: $(IMAGE_NAME).iso ovmf
	qemu-system-x86_64 $(QEMU_ARGS) -cdrom $(IMAGE_NAME).iso -boot d -s -S

.PHONY: run-hdd
run-hdd: $(IMAGE_NAME).hdd ovmf
	qemu-system-x86_64 $(QEMU_ARGS) -hda $(IMAGE_NAME).hdd

ovmf:
	mkdir -p ovmf
	cd ovmf && curl -Lo OVMF.fd https://retrage.github.io/edk2-nightly/bin/RELEASEX64_OVMF.fd

limine/limine:
	rm -rf limine
	git clone https://github.com/limine-bootloader/limine.git --branch=v7.x-binary --depth=1
	$(MAKE) -C limine

.PHONY: apps
apps:
	rm -rf apps/.dist
	mkdir -p apps/.dist
	cd apps && nasm hello.asm -o .dist/hello

.PHONY: kernel
kernel: apps
	cd kernel && cargo build --profile $(RUST_PROFILE)

.fsroot: limine/limine kernel files/*
	rm -rf .fsroot
	mkdir -p .fsroot/boot/limine
	mkdir -p .fsroot/EFI/BOOT
	mkdir -p .fsroot/bin

	cp kernel/target/$(RUST_TARGET_SUBDIR)/$(RUST_PROFILE_SUBDIR)/kernel .fsroot/boot/
	cp limine.cfg limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin .fsroot/boot/limine/
	cp limine/BOOTX64.EFI limine/BOOTIA32.EFI .fsroot/EFI/BOOT/

	cp -r files/* .fsroot/
	cp -r apps/.dist/* .fsroot/bin/

$(IMAGE_NAME).iso: .fsroot
	xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		.fsroot -o $(IMAGE_NAME).iso -quiet
	./limine/limine bios-install $(IMAGE_NAME).iso --quiet

$(IMAGE_NAME).hdd: .fsroot
	rm -f $(IMAGE_NAME).hdd
	dd if=/dev/zero bs=1M count=0 seek=64 of=$(IMAGE_NAME).hdd
	sgdisk $(IMAGE_NAME).hdd -n 1:2048 -t 1:ef00
	./limine/limine bios-install $(IMAGE_NAME).hdd
	mformat -i $(IMAGE_NAME).hdd@@1M
	mcopy -i $(IMAGE_NAME).hdd@@1M -s .fsroot/* ::/

.PHONY: clean
clean:
	rm -rf iso_root $(IMAGE_NAME).iso $(IMAGE_NAME).hdd
	$(MAKE) -C kernel clean

.PHONY: distclean
distclean: clean
	rm -rf limine ovmf
	$(MAKE) -C kernel distclean
