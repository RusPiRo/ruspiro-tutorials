#!/bin/sh
#*********************************************************************************
# script to build the kernel binary ready to be pushed to Raspberry Pi
#*********************************************************************************
set +ev

# aarch64
export CFLAGS="-march=armv8-a -Wall -O3 -nostdlib -nostartfiles -ffreestanding -mtune=cortex-a53"
export RUSTFLAGS="-C target-cpu=cortex-a53 -C target-feature=+strict-align,+a53,+fp-armv8,+neon -C link-arg=-nostartfiles -C opt-level=3 -C debuginfo=0 -C link-arg=-T./link64.ld"
# if there is any 1 command line argument given don't set the CC/AR variables
# this is required to be set only when building locally, but not on travis
# so we pass ./build.sh travis to build without those variables set
if [ -z "$1" ]
    then
        export CC="aarch64-elf-gcc"
        export AR="aarch64-elf-ar"
        export RUSTFLAGS="-C linker=aarch64-elf-gcc ${RUSTFLAGS}"
    else
        CARGOFLAGS="--verbose" 
fi

cargo xbuild --target aarch64-unknown-linux-gnu --release ${CARGOFLAGS}
if [ -z "$1" ]
    then
        cargo objcopy -- -O binary ./target/aarch64-unknown-linux-gnu/release/kernel ./target/kernel8.img
fi