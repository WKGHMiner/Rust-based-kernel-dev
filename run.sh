help() {
    echo "Run the script as following:"
    echo
    echo "    ./run.sh [option]?"
    echo
    echo "Supported parameters:"
    echo "    --temp: Clear up build files after launching."
}

run() {
    qemu-system-riscv64 --version

    cd ./user
    ./build.sh --build-only
    cd ../

    cp src/linker-qemu.ld src/linker.ld
    cargo build --release
    rm src/linker.ld

    release_dir="target/riscv64gc-unknown-none-elf/release/"

    # Clears the metadata.
    rust-objcopy --strip-all ${release_dir}os -O binary ${release_dir}os.bin
    rust-objdump --arch-name=riscv64 -x ${release_dir}os > disasm.asm

    # stat target/riscv64gc-unknown-none-elf/release/os
    # stat target/riscv64gc-unknown-none-elf/release/os.bin

    # Launch qemu.
    qemu-system-riscv64 \
        -machine virt \
        -nographic \
        -bios ../bootloader/rustsbi-qemu.bin \
        -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000
}

clean() {
    cargo clean
    cd user
    cargo clean
    cd ../
}

# Optional parameter.
if [ $# == 0 ]
then
    run
elif [ $# == 1 ]
then
    if [ $1 == "--help" ]
    then
        help
    elif [ $1 == "--temp" ]
    then
        run
        clean
    else
        echo "Unresolved parameter."
    fi
fi