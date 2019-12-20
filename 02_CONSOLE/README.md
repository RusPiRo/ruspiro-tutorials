# :man_teacher: The Console
Having a LED blinking is fine but quite often it is useful to provide some human readable text to
allow proper "debugging" in the bare metal environment. So this one is about a real ``Hello World!!``.

## :ticket: Prerequisites
It is assumed for this tutorial that you have performed the initial setup as described in
[this README](../README.md) and that you have read the first tutorial [01_BLINKLED](../01_BLINKLED/README.md).
The Raspberry Pi is connected from it's miniUART GPIO pins to the development machine (usually using
a serial TTLB-USB dongle).

## :running_woman: Quick start
The easiest way to get a basic project struture to begin with is by using an existing project
template. A mimimal version could be found [here](https://github.com/RusPiRo/ruspiro_templates/tree/templates/01_minimal).

To use it you use the following command:
```
$> cargo generate --git https://github.com/RusPiRo/ruspiro_templates.git --branch templates/01_minimum --name hello_world
```

This will create a new subfolder `hello-world` and create the files and folders based on the template
in the github repo.

For the ``Hello World!!`` excample we will use mainly the same dependencies as in the first tutorial.
However, as we do not want to blink any LED's this time we don't need the `ruspiro-gpio` or
`ruspiro-timer` crates.
```toml
[dependencies]
ruspiro-boot = { version = "0.3", features = ["ruspiro_pi3"] }
ruspiro-allocator = { version = "0.3" }
```

| Dependent&nbsp;crate&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;| Description |
|------------------|-------------|
| [``ruspiro-boot``](https://crates.io/crates/ruspiro-boot) | Booting the Raspberry Pi in a baremetal setup without an OS requires some initial assembly and preparation. This crate provides all the required boot strapping code and is responsible to kick off the cores of the Raspberry Pi and branch into the code written in Rust. |
| [``ruspiro-allocator``](https://crates.io/crates/ruspiro-allocator) | Providing a lightweight HEAP memory allocator |

As the ``ruspiro-boot`` crate already initializes the miniUART of the Raspberry  Pi we could immidiately
use this to print text to the "console". 

## :mailbox: The Kernel File

Let's focus on the important part of the kernel source file:
```rust
#[macro_use]
extern crate ruspiro_boot;          // link in the bootstrap functions
extern crate ruspiro_allocator;     // link in the custom heap allocator

// Set the function that is called on each core once it is alive and prepared to branch
// into the Rust 'world'
come_alive_with!(alive);

/// Any one-time initialization might be done here.
fn alive(core: u32) {
    println!("Hello World from core: {}", core);
}

// Set the function that is called on each core after the ``come_alive_with`` function has
// finished it's preparation. This function is intended to never return as there is nothing
// to be executed on the cores once this kernel has done what it is supposed to
run_with!(running);

/// Do the actual work on any core
fn running(_core: u32) -> ! {
    // this time there is nothing to do on the "processing" part, just ensure
    // we will never return
    loop {}
}
```

## :bulb: How does this work ?
The ``ruspiro-boot`` crate does configure the miniUART of the Raspberry Pi with a baud rate of
115200 by default. Also this crate uses the ``ruspiro-console`` crate and configures this conosle
abstraction to use the miniUART as output channel. The ``ruspiro-console`` crate provides the 
commonly used macros ``print!`` and ``println!`` to conviniently display formatted text to a console.
The onces provided have the same functionality as the ones contained in the ``std`` library of Rust.

As we are referring to the ``ruspiro-boot`` crate with ``#[macro_use]`` and ``extern crate`` this gives
us immediate access to the publicly available macross the ``ruspiro-boot`` crate is using. Thus we
could simply call ``println!`` in our kernel code.

## :hammer_and_wrench: Building the kernel

If all tools has been successfully configured ( as described [here](../README.md)), building the
kernel could be done by executing one the following scripts in the projects root folder:
Windows                | Linux
-----------------------|---------------------------
<pre>$> make all</pre> | <pre>$> ./build.sh</pre>

This might take a while at the first attempt as it does download the dependend crates from
[crates.io](https://crates.io) and does cross compile the Rust core library. As the build process is
incremental by default the next times the build will be much more faster.

The result of a successful execution of the build is the binary file ``kernel8.img`` in the ``target``
subfolder.

## :computer: Deploy the kernel
There are two options available to deploy the kernel to your Raspberry Pi:
### :floppy_disk: 1. Manual
Put this file to the SD card of your Raspberry Pi alongside
with the ``bootcode.bin`` and ``start.elf``. Those files could be found [here](../RPi) or the latest
version on the official Raspberry Pi [firmware repo](https://github.com/raspberrypi/firmware/tree/master/boot).
Now you could put this card into the Raspberry Pi and power it up.
If you have a terminal application open and connected to the serial port of the Raspberry Pi, the
console should print the following:
```
########## RusPiRo ----- Bootstrapper v0.3 @ Aarch64 ----- ##########
Hello World from core: 0
Hello World from core: 1
Hello World from core: 2
Hello World from core: 3
```

What a surprise :blush: the bootstrapper also wrote something to the console already...

### :fax: 2. Bootloader
This approach eliminates the "SD-Card-Dance". You will put all files contained in the [RPi](../RPi)
subfolder of this repo to your SD card. Including the ``kernel8.img`` which actually is the bootloader.
Connect your Raspberry Pi miniUART GPIO's to the serial Port of your development machine (usually done
through a serial TTLB-USB dongle) and power up your raspberry Pi.
Once a new kernel has been build and is present in the ``target`` subfolder of your project just execute
```
$> cargo ruspiro-push -k ./target/kernel8.img -p COM5
```
The serial port identifier may be different on your machine - ``COM5`` is the one on my Windows one.
For each new test cycle, just power of/on the Raspberry Pi and use the afformentioned command to push
a new version to the device.

> :bulb: **HINT:** As in this scenario the serial connection is used for the transfer of the kernel
> image you need to open the terminal from the development machine after the transfer has finished.
> This could be done with a small script or by chaning the commands like so (assuming ``terra term``
> is installed on Windows)
> ```
> $> cargo ruspiro-push -k ./target/kernel8.img -p COM5 && ttermpro /C=5 /BAUD=115200
> ```

## :control_knobs: To many cores ?
The bootstrapping of any bare metal kernel is by default kicking off all 4 cores of the Raspberry Pi.
If you'd rather like to use only a single core as this might make testing easier you could activate
the `singlecore` feature for the `ruspiro-boot` crate in the `[dependencies]` section of the
`Cargo.toml`file.
```toml
[dependencies]
ruspiro-boot = { version = "0.3", features = ["ruspiro_pi3", "singlecore"] }
```