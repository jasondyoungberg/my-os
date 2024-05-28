# MyOS

## How to run

Simply run `make run` to build and run the OS.

## Supported syscalls

| syscall | rax | rdi  | rsi | rdx | r10 | r8 | r9 |
|---------|-----|------|-----|-----|-----|----|----|
| write   |   1 | 1    | buf | len |     |    |    |
| yield   |  24 |      |     |     |     |    |    |
| exit    |  60 | code |     |     |     |    |    |
