# RusPiRo Tutorial Corner

This repository contains different tutorials that provide an easy to follow guide to get started with the development of **Rust** bare metal systems for the **Raspberry Pi**. The main focus for sure is the usage of the **ruspiro** crates and utilies to build a **robot** or anything else that comes to your mind.

## First things first

When you are completely new to Rust I would recommend to visit their [homepage](https://www.rust-lang.org) to learn a bit, what the language is and what it's syntax looks like and what ecosystem you could expect. Keep in mind, while reading, that we will target the *Raspberry Pi* as our system plattform so we will not be able to use the full set of features provided be the standard crate of Rust. We will more focus the **Embedded** world just relying on the **core** features.

## Initial setup

### Rust
To get things started you first need to install Rust on your machine. The easiest way to do so is by installing the **rustup** installer from [https://rustup.rs/](https://rustup.rs/). (run the *rustup-init.exe* on a windows machine)

**Rustup** is mainly a CLI to help you installing and configuring your **Rust** environment on your machine. The first thing to is to install the required tool chains to build/compile the bare metal kernel we are about to develop. This is done using your prefered CLI like *git bash*, *powershell* on **Windows**.
```
> rustup install nightly-gnu
```
This will install the nightly toolchain for your hosting machine. For my windows machine this would be *nightly-x86_64-pc-windows-gnu*. The exact toolchain name could differ based on your host machine.

The tool used to build/compile our Rust code is called *cargo*. This is installed as part of the Rust environment. However, as we would like to crosscompile (from a windows host machine in my case) we need to install an additional tool called *xbuild*.
```
> cargo install cargo-xbuild
```

After installing the cross-build tool we need to also install the crosscompile target to enable Rust to build for this target. This is done by adding the following target which fits our target system *Raspberry Pi* quite well:
```
> rustup target add armv7-unknown-linux-gnueabihf
```

We finish the Rust installation by adding the source code component as it needs to be available for the cross compilation:
```
> rustup component add rust-src
```

### Cross compiler

After finishing all the rust configurations we would need a cross compilation toolchain available for our host machine and able to compile to the desired target system architecture. For the windows host machine this could be donwloaded here:
https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-a/downloads .
Choose the version: ``i686-mingw32 hosetd: AArch32 bare-metal target (arm-eabi))``.
After installing the toolchain it is recommended to adjust the ``PATH`` environment variable to point to the ``bin`` and the ``lib`` subfolders of the toolchain installed. On my machine these folders are relative to my user home dir:
```
BIN = $USERPROFILE/arm-gcc/gcc-arm-eabi/bin
LIB = $USERPROFILE/arm-gcc/gcc-arm-eabi/lib/gcc/arm-eabi/8.3.0
```

If you are running **Mac OS** or **Linux OS** the following CLI command might also
install the necessary cross compile toolchain for you:
```
> apt-get install gcc-arm-linux-gnueabihf g++-arm-linux-gnueabihf
```

### IDE

To write the Rust code you would need an IDE that supports you in writing this code and also giving code completion and early hints on the syntax. For this purpose I use and recommend [Visual Studio Code](https://code.visualstudio.com/). Once downloaded and installed you should at least install the [Rust Language Server](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) extension. This will, when used the first time automatically install the rust language server (RLS) for you. If you'd like to install it on your own use this command:
```
> rustup component add rls --toolchain nightly
```

### Deployment

The result of a successfull build is a ``kernel7.img`` file. The easiest way to get this executed on th Raspberry Pi is to copy it onto a fresh FAT32 formatted SD card. This SD card need t contain additional files like ``bootcode.bin``, ``start.elf`` and ``fixup.dat``. The newest versions of them could be found at the official [Github Repo](https://github.com/raspberrypi/firmware/tree/master/boot). The files with ``_x`` suffix indicate extended versions of the GPU boot code that enable access to additional hardware like the buildin bluetooth controler and the like. So if you foresee to use all the peripherals of the Raspberry Pi I recommnd to use those files.

Building the ``kernel7.img`` file and repeatedly put it onto the SD card of the Raspberry Pi pretty soon gets cumbersome. To reduce this SD card "dance" you will find in the [RPi] folder the files you might want to put on your SD card of the Raspberry Pi. This contains already a ``kernel7.img`` which is nothing but a small boot loader. Once you power on the Raspberry Pi this get's executed and waits on the miniUART to receive a new kernel binary to restart with. Use a simple USB-TTL connection dongle to connect your PC/Mac to the miniUart pins (14/15) of the Raspberry Pi. Then use a serial terminal program to send the new kernel binary to the Raspberry Pi through this ``Comm port``. It should be configured for ``115200`` baud rate, ``8 bit`` data, ``no`` parity and ``1`` stop bit without flow control.

On a Windows machine I'm using ``Teraterm`` as this also comes with a small macro execution engine to automate the transfer.

When using the bootloader as provided you first need to send the string ``DEADBEEF`` to the Raspberry Pi that is waiting for this token to know that now the kernel binary is following. After the whole file has been transmitted send again a ``DEADBEEF`` token and the Raspberry Pi will reboot and start with the kernel you just loaded.
Once done with testing just switch off the raspberry Pi and back on and you could load another version of your kernel if you like with the same procedure.

Check out the [deploy.sh] script which does the preparation of the kernel binary file by prefixing and suffixing with ``DEADBEEF`` and then transferring to the com-port using ``Teraterm``. The sript takes a subfolder name as argument from were to take the ``kernel.img`` file. Assuming a build of our first example this could be deployed by calling:
```
> ./deploy.sh 01_BLINKLED
```

## Ready to go...

If all tools are installed then you are ready to go and check the different tutorials to get your hands on Rust for Raspberry Pi.

See you there ...

| Tutorial&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;| Description |
|--------------------|-------------|
|[01 Blinking LED](01_BLINKLED) | The initial tutorial providing the bare metal version of a "Hello World" program. It aims to help you validate with this easy example that your tools are properly installed and configured. |

