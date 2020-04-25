#*********************************************************************************
# Makefile to build the kernel binary ready to be pushed to the Raspberry Pi
#*********************************************************************************
all32: export CFLAGS = -mcpu=cortex-a53 -march=armv7-a -mfpu=neon -mfloat-abi=softfp -Wall -O3 -nostdlib -nostartfiles -ffreestanding -mtune=cortex-a53
all32: export RUSTFLAGS = -C linker=arm-none-eabi-gcc.exe -C target-cpu=cortex-a53 -C link-arg=-nostartfiles -C link-arg=-T./link32.ld
all32: export CC = arm-none-eabi-gcc.exe
all32: export AR = arm-none-eabi-ar.exe
all32: kernel7
	arm-none-eabi-objcopy -O binary .\\target\\armv7a-none-eabi\\release\\kernel .\\target\\kernel7.img

kernel7:
	cargo xbuild --target armv7a-none-eabi --release --bin kernel --target-dir ./target/

all64: export CFLAGS = -march=armv8-a -Wall -O3 -nostdlib -nostartfiles -ffreestanding -mtune=cortex-a53
all64: export RUSTFLAGS = -C linker=aarch64-none-elf-gcc -C target-cpu=cortex-a53 -C link-arg=-nostartfiles -C link-arg=-T./link64.ld
all64: export CC = aarch64-none-elf-gcc
all64: export AR = aarch64-none-elf-ar
all64: kernel8
	aarch64-none-elf-objcopy -O binary .\\target\\aarch64-unknown-linux-gnu\\release\\kernel .\\target\\kernel8.img

kernel8:
	cargo xbuild --target aarch64-unknown-linux-gnu --release --bin kernel --target-dir ./target/

clean:
	cargo clean