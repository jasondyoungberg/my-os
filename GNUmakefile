# Nuke built-in rules and variables.
override MAKEFLAGS += -rR

override IMAGE_NAME := template

# Convenience macro to reliably declare user overridable variables.
define DEFAULT_VAR =
    ifeq ($(origin $1),default)
        override $(1) := $(2)
    endif
    ifeq ($(origin $1),undefined)
        override $(1) := $(2)
    endif
endef

QEMU_ARGS := \
	-M q35 \
	-m 2G \
	-debugcon stdio \

ifeq ($(UEFI),1)
	QEMU_ARGS += -bios ovmf/OVMF.fd
endif

ifeq ($(KVM),1)
	QEMU_ARGS += -enable-kvm
endif

.PHONY: all
all: $(IMAGE_NAME).iso

.PHONY: all-hdd
all-hdd: $(IMAGE_NAME).hdd

.PHONY: run
run: $(IMAGE_NAME).iso
	qemu-system-x86_64 $(QEMU_ARGS) -cdrom $(IMAGE_NAME).iso -boot d

.PHONY: debug
debug: $(IMAGE_NAME).iso
	qemu-system-x86_64 $(QEMU_ARGS) -cdrom $(IMAGE_NAME).iso -boot d -s -S

.PHONY: run-hdd
run-hdd: $(IMAGE_NAME).hdd
	qemu-system-x86_64 -M q35 -m 2G -hda $(IMAGE_NAME).hdd

ovmf:
	mkdir -p ovmf
	cd ovmf && curl -Lo OVMF.fd https://retrage.github.io/edk2-nightly/bin/RELEASEX64_OVMF.fd

limine/limine:
	rm -rf limine
	git clone https://github.com/limine-bootloader/limine.git --branch=v7.x-binary --depth=1
	$(MAKE) -C limine

.PHONY: kernel
kernel:
	$(MAKE) -C kernel

.fsroot: limine/limine kernel
	rm -rf .fsroot
	mkdir -p .fsroot/boot/
	mkdir -p .fsroot/EFI/BOOT

	cp -v kernel/bin/kernel limine.cfg .fsroot/
	cp -v limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin .fsroot/boot/
	cp -v limine/BOOTX64.EFI limine/BOOTIA32.EFI .fsroot/EFI/BOOT/

$(IMAGE_NAME).iso: .fsroot
	xorriso -as mkisofs -b boot/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		.fsroot -o $(IMAGE_NAME).iso
	./limine/limine bios-install $(IMAGE_NAME).iso

$(IMAGE_NAME).hdd: .fsroot
	rm -f $(IMAGE_NAME).hdd
	dd if=/dev/zero bs=1M count=0 seek=64 of=$(IMAGE_NAME).hdd
	sgdisk $(IMAGE_NAME).hdd -n 1:2048 -t 1:ef00
	./limine/limine bios-install $(IMAGE_NAME).hdd
	mformat -i $(IMAGE_NAME).hdd@@1M
	mcopy -i $(IMAGE_NAME).hdd@@1M -s .fsroot ::

.PHONY: clean
clean:
	rm -rf iso_root $(IMAGE_NAME).iso $(IMAGE_NAME).hdd .fsroot
	$(MAKE) -C kernel clean

.PHONY: distclean
distclean: clean
	rm -rf limine ovmf
	$(MAKE) -C kernel distclean
