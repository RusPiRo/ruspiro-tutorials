#*****************************************************************
# Makefile to build the RusPiRo kernel
# setting the necessary environment for cross compiling
#
# Copyright (c) 2019 by the authors
# 
# Author: André Borrmann 
# License: Apache License 2.0
#******************************************************************

export CFLAGS = -mfpu=neon-fp-armv8 -mfloat-abi=hard -march=armv8-a -Wall -O3 -nostdlib -nostartfiles -ffreestanding -mtune=cortex-a53
export RUSTFLAGS = -C linker=arm-eabi-gcc.exe -C target-cpu=cortex-a53 -C target-feature=+a53,+fp-armv8,+v8,+vfp3,+d16,+thumb2,+neon -C link-arg=-nostartfiles -C link-arg=-T$(PWD)/link.ld -C opt-level=3 -C debuginfo=0
export CC = arm-eabi-gcc.exe
export AR = arm-eabi-ar.exe


all: kernel7
	cargo objcopy -- -O binary .\\target\\armv7-unknown-linux-gnueabihf\\release\\$< .\\target\\kernel7.img

kernel7:
	cargo xbuild --target armv7-unknown-linux-gnueabihf --release --bin kernel7 --target-dir ./target/
