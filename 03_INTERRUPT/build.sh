#!/bin/sh
#*********************************************************************************
# script to build the kernel binary ready to be pushed to Raspberry Pi
#*********************************************************************************
set +ev

if [ $# -eq 0 ] 
    then 
        echo "provide the target architecture to build for - 32 or 64"
        exit 1
fi

if [ $1 = "64" ]
    then
        # aarch64
        _CFLAGS="-march=armv8-a -Wall -O3 -nostdlib -nostartfiles -ffreestanding -mtune=cortex-a53"
        _RUSTFLAGS="-C linker=aarch64-elf-gcc -C target-cpu=cortex-a53 -C target-feature=+strict-align,+a53,+fp-armv8,+neon -C link-arg=-nostartfiles -C opt-level=3 -C debuginfo=0 -C link-arg=-T./link64.ld"
        _CC="aarch64-elf-gcc"
        _AR="aarch64-elf-ar"
        _TARGET="aarch64-unknown-linux-gnu"
        _KERNEL="kernel8.img"
elif [ $1 = "32" ]
    then
        _CFLAGS="-mfpu=neon-fp-armv8 -mfloat-abi=hard -march=armv8-a -Wall -O3 -nostdlib -nostartfiles -ffreestanding -mtune=cortex-a53"
        _RUSTFLAGS="-C linker=arm-eabi-gcc -C target-cpu=cortex-a53 -C target-feature=+strict-align,+a53,+fp-armv8,+v8,+vfp3,+d16,+thumb2,+neon -C link-arg=-nostartfiles -C link-arg=-T./link32.ld -C opt-level=3 -C debuginfo=0"
        _CC="arm-eabi-gcc"
        _AR="arm-eabi-ar"
        _TARGET="armv7-unknown-linux-gnueabihf"
        _KERNEL="kernel7.img"
else
    echo 'provide the archtitecture to be build. Use either "build.sh 32" or "build.sh 64".'
    exit 1
fi

export CFLAGS="${_CFLAGS}"
export RUSTFLAGS="${_RUSTFLAGS}"
export CC="${_CC}"
export AR="${_AR}"
export TARGET="${_TARGET}"
export KERNEL="${_KERNEL}"

cargo xbuild --target ${_TARGET} --release
cargo objcopy -- -O binary ./target/${_TARGET}/release/kernel ./target/${_KERNEL}