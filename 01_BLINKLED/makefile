#*********************************************************************************
# Makefile to build the kernel binary ready to be pushed to the Raspberry Pi
#*********************************************************************************

all: export CFLAGS = -march=armv8-a -Wall -O3 -nostdlib -nostartfiles -ffreestanding -mtune=cortex-a53
all: export RUSTFLAGS = -C linker=aarch64-elf-gcc -C target-cpu=cortex-a53 -C target-feature=+strict-align,+a53,+fp-armv8,+neon -C link-arg=-nostartfiles -C link-arg=-T./link64.ld -C opt-level=3 -C debuginfo=0
all: export CC = aarch64-elf-gcc
all: export AR = aarch64-elf-ar
all: kernel8
	cargo objcopy -- -O binary .\\target\\aarch64-unknown-linux-gnu\\release\\kernel .\\target\\kernel8.img

kernel8:
	cargo xbuild --target aarch64-unknown-linux-gnu --release --bin kernel --target-dir ./target/

clean:
	cargo clean
	