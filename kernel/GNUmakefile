CC = x86_64-elf-gcc
LD = x86_64-elf-ld

C_FLAGS  = -g -O2 -pipe -Wall -Wextra
LD_FLAGS = 

override C_FLAGS +=       \
	-std=gnu17           \
	-ffreestanding       \
	-fno-stack-protector \
	-fno-stack-check     \
	-fno-lto             \
	-fno-pie             \
	-fno-pic             \
	-m64                 \
	-march=x86-64        \
	-mabi=sysv           \
	-mno-80387           \
	-mno-mmx             \
	-mno-sse             \
	-mno-sse2            \
	-mno-red-zone        \
	-mcmodel=kernel      \
	-MMD                 \
	-I../limine/

override LD_FLAGS +=         \
	-nostdlib               \
	-static                 \
	-m elf_x86_64           \
	-z max-page-size=0x1000 \
	-T linker.ld

override CFILES := $(shell find . -type f -name '*.c')
override OBJ    := $(CFILES:.c=.o)
override DEP         := $(OBJ:.o=.d)

# Don't try to auto-compile
.SUFFIXES:

.PHONY: all
all: kernel.elf

.PHONY: clean
clean:
	rm -f kernel.elf $(OBJ) $(DEP)

kernel.elf: $(OBJ)
	$(LD) $(OBJ) $(LD_FLAGS) -o $@

%.o: %.c
	$(CC) $(C_FLAGS) -c $< -o $@

-include $(DEP)