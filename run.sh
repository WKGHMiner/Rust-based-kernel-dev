cd ./user
./build.sh --build-only
cd ../

cp src/linker-qemu.ld src/linker.ld
cargo build --release
rm src/linker.ld

# Clears the metadata.
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin
rust-objdump --arch-name=riscv64 -x target/riscv64gc-unknown-none-elf/release/os > disasm.asm

# stat target/riscv64gc-unknown-none-elf/release/os
# stat target/riscv64gc-unknown-none-elf/release/os.bin

# Launch qemu.
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \

if [ $# == 1 ]
then
    if [ $1 == "--temp" ]
    then
        cargo clean
        cd user
        cargo clean
        cd ../
    fi
fi