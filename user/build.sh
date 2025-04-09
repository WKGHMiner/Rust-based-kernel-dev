build() {
    cargo build --release

    out_dir="./target/riscv64gc-unknown-none-elf/release/"

    # Read target files from ./src/bin.
    files=$(ls ./src/bin)

    # Iterate over the `bin` directory to obtain binary files.
    for file in $files; do
        length=${#file}
        # Redirect to the target elf file by slicing.
        target=${out_dir}${file:0:length - 3}

        # Then do the strip work.
        rust-objcopy --strip-all $target -O binary ${target}.bin

        if [ $1 == 1 ]
        then
            qemu-riscv64 ${target}
        fi
    done
}

if [ $# == 1 ]
then
    if [ $1 == "--build-only" ]
    then
        build 0
    else
        echo "Unresolved parameter."
    fi
elif [ $# == 0 ]
then
    build 1
else
    echo "Unresolved parameters."
fi
