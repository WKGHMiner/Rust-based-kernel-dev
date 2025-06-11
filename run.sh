nls="\n"
nlts="\n\t"

help_doc="

Run the script as following:
$nls$nlts
    ./run.sh [option]?
$nls$nls
Supported parameters:
$nls$nlts
    --check: Check for environment.$nlts
    --clean: Clean up build files.$nlts
    --help: Check for available options.$nlts
    --temp: Clear up build files after launching.$nlts

"

osr_help() {
    echo -e $help_doc
}

osr_run() {
    cd ./user
    ./build.sh --build-only
    cd ../

    cargo build --release

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
        -device loader,file=${release_dir}os.bin,addr=0x80200000
}

osr_clean() {
    cargo clean
    cd user
    cargo clean
    cd ../
}

osr_check() {
    echo -e "Version of Rust toolchain."
    rustc --version

    echo -e "\nVersion of Cargo."
    cargo --version

    echo -e "\nVersion of Qemu simulator."
    qemu-system-riscv64 --version
}

# Optional parameter.
if [ $# == 0 ]
then
    # Run by default for no extra options.
    osr_run
elif [ $# == 1 ]
then
    if [ $1 == "--help" ]
    then
        osr_help
    elif [ $1 == "--temp" ]
    then
        osr_run
        osr_clean
    elif [ $1 == "--check" ]
    then
        osr_check
    elif [ $1 == "--clean" ]
    then
        osr_clean
    else
        echo Unresolved parameter.
    fi
else
    echo Unresolved parameter.
fi