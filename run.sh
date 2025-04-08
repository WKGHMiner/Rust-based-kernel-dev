cargo build --release

# Clears the metadata.
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin

# stat target/riscv64gc-unknown-none-elf/release/os

# stat target/riscv64gc-unknown-none-elf/release/os.bin

# Launch qemu.
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
