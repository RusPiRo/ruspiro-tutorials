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
        # use the right compiler toolchain prefix when building on travis
        if [ -z "$2" ]
            then
                PREFIX=aarch64-none-elf-
            else
                PREFIX=aarch64-linux-gnu-
        fi
        CFLAGS="-march=armv8-a -Wall -O3 -nostdlib -nostartfiles -ffreestanding -mtune=cortex-a53"
        RUSTFLAGS="-C linker=${PREFIX}gcc -C target-cpu=cortex-a53 -C link-arg=-nostartfiles -C link-arg=-T./link64.ld"
        TARGET="aarch64-unknown-none"
        KERNEL="kernel8.img"
elif [ $1 = "32" ]
    then
        # aarch32
        # use the right compiler toolchain prefix when building
        PREFIX=arm-none-eabi-
        CFLAGS="-mcpu=cortex-a53 -march=armv7-a -mfpu=neon -mfloat-abi=softfp -Wall -O3 -nostdlib -nostartfiles -ffreestanding -mtune=cortex-a53"
        RUSTFLAGS="-C linker=${PREFIX}gcc -C target-cpu=cortex-a53 -C link-arg=-nostartfiles -C link-arg=-T./link32.ld"
        TARGET="armv7a-none-eabi"
        KERNEL="kernel7.img"
else
    echo 'provide the archtitecture to be build. Use either "build.sh 32" or "build.sh 64"'
    exit 1
fi

export CFLAGS="${CFLAGS}"
export RUSTFLAGS="${RUSTFLAGS}"
export CC="${PREFIX}gcc"
export AR="${PREFIX}ar"

cargo xbuild --target ${TARGET} --release && ${PREFIX}objcopy -O binary ./target/${TARGET}/release/kernel ./target/${KERNEL}
# cargo objcopy tries to re-build the crates without the propper cross compile target since whatever version
# so don't use it but rather use the arm toolchain objcopy to achieve the same
#cargo objcopy -- -O binary ./target/${TARGET}/release/kernel ./target/${KERNEL}

