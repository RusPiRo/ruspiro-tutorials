#!/bin/bash
set -ev
#*****************************************************************
# build script to build the RusPiRo kernel
# setting the necessary environment for cross compiling
#
# Copyright (c) 2019 by the authors
# 
# Author: André Borrmann 
# License: Apache License 2.0
#******************************************************************

# get the folder of script execution
CURRENT="$( cd "$( dirname "$(readlink -f "$0")" )" >/dev/null 2>&1 && pwd )"
# build the absolute path to the linker script file
LINKFILE="$CURRENT/link.ld"
# convert posix path into windows path
LINK="$( echo "$LINKFILE" | sed -e 's/^\///' -e 's/\//\\/g' -e 's/^./\0:/' )"

# export C compile flags required by rustc
export CFLAGS='-mfpu=neon-fp-armv8 -mfloat-abi=hard -march=armv8-a -Wall -O3 -nostdlib -nostartfiles -ffreestanding -mtune=cortex-a53'
# export RUST compile and linker flags required by rustc
export RUSTFLAGS="-C linker=arm-eabi-gcc -C target-cpu=cortex-a53 -C target-feature=+a53,+fp-armv8,+v8,+vfp3,+d16,+thumb2,+neon -C link-arg=-nostartfiles -C link-arg=-T$LINK -C opt-level=3 -C debuginfo=0"
# export the exact names for the compiler executables to be used
export CC="arm-eabi-gcc"
export AR="arm-eabi-ar"

# build the binary
cargo xbuild --target armv7-unknown-linux-gnueabihf --release --bin kernel7 --target-dir ./target/

# dump the binary into kernel image file
cargo objcopy -- -O binary ./target/armv7-unknown-linux-gnueabihf/release/kernel7 ./target/kernel7.img