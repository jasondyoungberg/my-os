# my-os

To run the OS, you need to have QEMU installed. Then, run the following command:

```bash
cargo run
```

# Testing
To compile the kernel for tests, run the following command:

```bash
cargo test -p kernel --no-run --target x86_64-unknown-none
```

Then, run the tests with the following command:

```bash
cargo run -- --kernel <path-to-kernel-binary> --test
```
