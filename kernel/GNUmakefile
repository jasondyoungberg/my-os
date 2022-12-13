-include ../gcc.config

override C_FLAGS += \
	-I../limine/    \

override CFILES := $(shell find . -type f -name '*.c')
override OBJ    := $(CFILES:.c=.o)
override DEP    := $(OBJ:.o=.d)

# Don't try to auto-compile
.SUFFIXES:

.PHONY: all
all: kernel.elf

.PHONY: clean
clean:
	@rm -f kernel.elf $(OBJ) $(DEP)

kernel.elf: $(OBJ)
	@echo \| Linking objects
	@$(LD) $(OBJ) $(LD_FLAGS) -o $@

%.o: %.c
	@echo \| Compiling $<
	@$(CC) $(C_FLAGS) -c $< -o $@

-include $(DEP)