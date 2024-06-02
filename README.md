# MyOS

## How to run

Simply run `make run` to build and run the OS.

## Memory layout

|              Address                |   Description    |
|-------------------------------------|------------------|
| `D000 0000 0000` - `DFFF FFFF FFFF` | TSS Stacks       |
| `E000 0000 0000` - `EFFF FFFF FFFF` | Memory Mapped IO |
| `FFFF 8000 0000` - `FFFF FFFF FFFF` | Kernel Code      |


## Supported syscalls (planned)

| syscall | rax | rdi  | rsi | rdx | r10 | r8 | r9 |
|---------|-----|------|-----|-----|-----|----|----|
| write   |   1 | 1    | buf | len |     |    |    |
| yield   |  24 |      |     |     |     |    |    |
| exit    |  60 | code |     |     |     |    |    |
