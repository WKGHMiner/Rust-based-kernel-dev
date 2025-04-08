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

    qemu-riscv64 ${target}
done
