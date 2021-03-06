# :woman_teacher: RusPiRo Tutorial Corner

This repository contains different tutorials that provide an easy to follow guide to get started 
with the development of ``Rust`` bare metal systems for the ``Raspberry Pi``. The main focus for 
sure is the usage of the ``ruspiro-*`` crates and utilies to build may be a  **Robot** or similar or
just anything else that comes to your mind.

## :stop_sign: First things first

When you are completely new to Rust I would recommend to visit their [homepage](https://www.rust-lang.org)
to learn a bit, what the language is and what it's syntax looks like as well as what ecosystem is 
you could expect. Keep in mind, while reading, that we will target the **Raspberry Pi** as our system
plattform so we will not be able to use the full set of features provided by the standard libraries
of Rust. We will mainly focus the ``Embedded`` world and thus relying on the ``core`` features.

## :hammer_and_wrench: Initial setup

### Rust
To get things started you first need to install Rust on your development machine. The easiest way to 
do so is by installing the ``rustup`` installer from [https://rustup.rs/](https://rustup.rs/).
> ::bulb: **HINT** run the ``rustup-init.exe`` on a windows machine and choose to install ``x86_64-pc-windows-msvc`` as toolchain, the ``nightly`` rust version and the ``minimal`` rust package. This will not install the documentation locally but it has been seen this sometimes causes issues while installing on windows.

``Rustup`` is mainly a command line interface (CLI) to help you installing and configuring your
**Rust** environment on your machine. The first thing to do is to install the required tools to 
build the bare metal kernel we are about to develop. This is done using your prefered CLI like 
*git bash*, *powershell* on ``Windows``.

The tool used to build/compile our Rust code is called *cargo*. This is installed as part of the 
Rust environment. However, as we would like to crosscompile (from a windows host machine in my case)
we need to install two additional tools ``xbuild`` and ``binutils``. Also very helpful for a convinient build pipeline is the
``cargo make`` tool which we will use in conjuction with a ``Makefile.toml`` to build the examples given in each tutorial.

```shell
$> cargo install cargo-xbuild
$> cargo install cargo-binutils
$> cargo install cargo-make
```

After installing the cross-build tool we need to also install the crosscompile target to enable Rust
to build for this target. This is done by adding the following target which fits our target system 
``Raspberry Pi 3`` quite well:

Aarch32 build target | Aarch64 build target
---------------------|----------------------
``$> rustup target add armv7a-none-eabi`` | ``$> rustup target add aarch64-unknown-none``

We finish the Rust installation by adding the source code component as it needs to be available for
the cross compilation:

```shell
$> rustup component add rust-src
```

### :gear: Cross compiler

After finishing all the rust configurations we would need a cross compilation toolchain available for our host machine and able to compile to the desired target system architecture. For the windows host machine this could be donwloaded here: https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-a/downloads .

Architecture | Windows Toolchain | Linux Toolchain
-------------|-------------------|-------------------
Aarch32 | download ``i686-mingw64 hosetd: AArch32 bare-metal target (arm-none-eabi))`` | ``$>sudo apt-get install gcc-arm-none-abi`` 
Aarch64 | download ``i686-mingw64 hosetd: AArch64 bare-metal target (aarch64-none-elf)`` | download ``x86_64-linux hosetd: AArch64 bare-metal target (aarch64-none-elf)``

After installing the toolchain it is recommended to adjust the ``PATH`` environment variable to
point to the ``bin`` and the ``lib`` subfolders of the toolchain installed.

### :pager: IDE

To write the Rust code you would need an IDE that supports you in writing this code and also giving code completion and early hints on the syntax. For this purpose I use and recommend [Visual Studio Code](https://code.visualstudio.com/). Once downloaded and installed you should at least install the [Rust Language Server](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) extension. This will, when used the first time, automatically install the rust language server (RLS) for you. If you'd like to install it on your own use this command:

```shell
$> rustup component add rls --toolchain nightly
```

Another great extension to be used with Visual Studio Code is the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) that can be used as alternative to the `Rust Language Server` and does provide some additional useful features.

### :computer: Deployment

The result of a successfull build will be a binary kernel file. The easiest way to get this executed on the Raspberry Pi is to copy it onto a fresh FAT32 formatted SD card. This SD card need to contain additional files like ``bootcode.bin``, ``start.elf`` and ``fixup.dat``. They can be found [here](../RPi) or the lates official versions in this [Github Repo](https://github.com/raspberrypi/firmware/tree/master/boot). The files with ``_x`` suffix indicate extended versions of the Raspberry Pi Firmware that enable access to additional hardware like the built-in bluetooth controller and the like. So if you foresee to use all the peripherals of the Raspberry Pi in future projects I recommnd to use those files.

Building the kernel binary file and repeatedly put it onto the SD card of the Raspberry Pi will  pretty soon get cumbersome. To reduce this "SD card dance" you will find an bootloader image file in the [RPi](../RPi) subfolder to be put on the SD card instead of your build kernel file. Once you  power on the Raspberry Pi this bootloader waits on the miniUART to receive a new kernel binary to get executed. For this to work the Raspberry Pi need to be connected to a serial port of the development machine. This is usually achieved with a simple TTL-USB dongle (use GPIO 14, 15 and GROUND on Raspberry Pi). If this is done you could use a cargo subcommand to push your new built kernel file to the Raspberry Pi.

Install the subcommand with:

```shell
$> cargo install cargo-ruspiro-push
```

And then execute it from your projects root folder like so:

```shell
$> cargo ruspiro-push -k ./target/kernel8.img -p COM5
```

Adjust the name of your kernel file and the serial port name of this command to your needs.
> :bulb: **HINT** The ``ruspiro-push`` tool determines based on the kernel file name whether to run
> in Aarch32 (kernel7.img) or Aarch64 (kernel8.img) mode. If you use any other file name provide the
> ``-a`` parameter to selct the target architecture.


> :bulb: **HINT** The `Makefile.toml` of the examples of this tutorial series also provides a build step to build and deploy the kernel to the Raspberry Pi connected to the hostinng machine. Simply execute it with `cargo make --profile a64 deploy`. The console of the host machine will mirror the console/UART1 output of the Raspberry Pi.

## :tada: Ready to go...

If all tools are installed then you are ready to go and check the different tutorials to get your hands on Rust for Raspberry Pi.

See you there ...

| Tutorial&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;| Description |
|--------------------|-------------|
|[01 Blinking LED](01_BLINKLED) | The initial tutorial providing the bare metal version of a "Hello World" program. It aims to help you validate with this easy example that your tools are properly installed and configured. |
|[02 Console](02_CONSOLE) | This time the Raspberry Pi really writes "Hello World" to a connected terminal console. |
|[03 Interrupt Handling](03_INTERRUPT) | Introducing the usage of interrupt handler at the excample of the Arm-Timer raising interrupts. |
|[04 The I²C Bus](04_I2C) | Using the ``ruspiro_i2c`` crate to discover and access I²C devices connected to the Raspberry Pi.  |
