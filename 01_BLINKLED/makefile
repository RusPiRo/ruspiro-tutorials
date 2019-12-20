#*********************************************************************************
# Makefile to build the kernel binary ready to be pushed to the Raspberry Pi
#*********************************************************************************
all32: export CFLAGS = -mfpu=neon-fp-armv8 -mfloat-abi=hard -march=armv8-a -Wall -O3 -nostdlib -nostartfiles -ffreestanding -mtune=cortex-a53
all32: export RUSTFLAGS = -C linker=arm-eabi-gcc.exe -C target-cpu=cortex-a53 -C target-feature=+strict-align,+a53,+fp-armv8,+v8,+vfp3,+d16,+thumb2,+neon -C link-arg=-nostartfiles -C link-arg=-T./link32.ld -C opt-level=3 -C debuginfo=0
all32: export CC = arm-eabi-gcc.exe
all32: export AR = arm-eabi-ar.exe
all32: kernel7
	cargo objcopy -- -O binary .\\target\\armv7-unknown-linux-gnueabihf\\release\\kernel .\\target\\kernel7.img

kernel7:
	cargo xbuild --target armv7-unknown-linux-gnueabihf --release --bin kernel --target-dir ./target/

all64: export CFLAGS = -march=armv8-a -Wall -O3 -nostdlib -nostartfiles -ffreestanding -mtune=cortex-a53
all64: export RUSTFLAGS = -C linker=aarch64-elf-gcc -C target-cpu=cortex-a53 -C target-feature=+strict-align,+a53,+fp-armv8,+neon -C link-arg=-nostartfiles -C link-arg=-T./link64.ld -C opt-level=3 -C debuginfo=0
all64: export CC = aarch64-elf-gcc
all64: export AR = aarch64-elf-ar
all64: kernel8
	cargo objcopy -- -O binary .\\target\\aarch64-unknown-linux-gnu\\release\\kernel .\\target\\kernel8.img

kernel8:
	cargo xbuild --target aarch64-unknown-linux-gnu --release --bin kernel --target-dir ./target/

clean:
	cargo clean