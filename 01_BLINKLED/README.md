# :man_teacher: The Blinking LED
The ``Hello World!!`` for bare metal environments. A **must do** from my point of view :smile: to 
get things started and it is best suited to verify the tools and the toolchain are smoothly working
and creating a run-able kernel for the Raspberry Pi.

As the Raspberry Pi that runs without any OS is usually not able to display a greeting message to a
connected screen we will use the provided GPIO pins to connect a LED to it at let the Raspberry Pi
blink this LED to greet the world.

## :ticket: Prerequisites
It is assumed for this tutorial that you have performed the initial setup as described in
[this README](../README.md)

## :running_woman: Quick start
The easiest way to get a basic project struture to begin with is by using an existing project
template. A mimimal version could be found [here](https://github.com/RusPiRo/ruspiro_templates/tree/templates/01_minimal).

To use it you use the following command:
```
$> cargo generate --git https://github.com/RusPiRo/ruspiro_templates.git --branch templates/01_minimum --name hello_led
```

This will create a new subfolder `hello-led` and create the files and folders based on the template
in the github repo.

## :building_construction: Basic folder structure
For new-comers lets start with the basic folder structure of our project (as it is structured in this
repo and how it will look like at your end, if you used the template mentionned above):
```
01_BLINKLED         // project root folder (will be the name of the project you have choosen)
├─ src              // here goes all the rust source files
│   └─ kernel.rs    // the current one and only rust source file containing our code to execute on the Raspberry Pi
├─ Cargo.toml       // the build configuration file to tell Rust how to build our bare metal binary and what we
│                   // depend on (which crates to incorporate into the build)
├─ build.rs         // specif build script to provide the correct linker script file
├─ build.sh         // a small script to execute the rust build
├─ makefile         // a makefile to execute the rust build in case you prefer make over the shell script
├─ LICENSE-*        // the license files for your crate
└─ README.md        // well, this is the file you are currently reading, at this very moment :)

```

## :package: Defining the package
The maybe first thing to do is to describe the package we would like to build adjusting the 
``Cargo.toml`` file. The first *chapter* `[package]` specifies some meta-data. The minimum required entries here
are already provided by the template. The ``publish = false`` prevents accidently publishing of this
crate to the public crate library of Rust [crates.io](https://crates.io).
You may want to adjust the description of your crate:
```toml
[package]
description = """
This is a RusPiRo template
"""
```

The second *chapter* `[[bin]]` defines the requested output Rust should create while building this
crate. This could either be a library or a binary. In our case we will build a binary that will be
deployed to the Raspberry Pi. The configuration defines the binary name and the entry rust file to
be used for compilation. Any functionality that should be compiled into the binary must be referenced
in this main file.

```toml
[[bin]]
name = "kernel"         # name of the binary to build
path = "./src/kernel.rs"  # main source file to build the binary from
```

Finally in the third *chapter* `[dependencies]` we list all dependend crates (libraries so to speek)
we would like to use functionality from in our own crate. Those crates can be found on 
[crates.io](https://crates.io). You will also find all the ``ruspiro-*`` crates there with links to
their documentation and the github repo's.

To start a lightweight first bare metal kernel for Raspberry Pi we would need the following
dependencies to be configured:
```toml
[dependencies]
ruspiro-boot = { version = "0.3", features = ["ruspiro_pi3"] }
ruspiro-allocator = { version = "0.3" }
ruspiro-gpio = { version = "0.2", features = ["ruspiro_pi3"] }
ruspiro-timer = { version = "0.1", features = ["ruspiro_pi3"] }
```

Except the `ruspiro-gpio`and the `ruspiro-timer` the dependency section should be prefilled from the
template.

What does those dependencies provide:

| Dependent&nbsp;crate&nbsp;&nbsp;&nbsp;| Description |
|------------------|-------------|
| [``ruspiro-boot``](https://crates.io/crates/ruspiro-boot) | Booting the Raspberry Pi in a baremetal setup without an OS requires some initial assembly and preparation. This crate provides all the required boot strapping code and is responsible to kick off the cores of the Raspberry Pi and branch into the code written in Rust. |
| [``ruspiro-allocator``](https://crates.io/crates/ruspiro-allocator) | Providing a lightweight HEAP memory allocator |
| [``ruspiro-gpio``](https://crates.io/crates/ruspiro-gpio) | This is the API crate to access the GPIO pins available with the Raspberry Pi. It hides the complexity of the setup and usage of the different pin's behind easy to consume function calls. |
| [``ruspiro-timer``](https://crates.io/crates/ruspiro-timer) | Simple timing functions to allow to pause execution for a specific amount of time. The timing is done based on the internal free running counter of the Raspberry Pi system timer that is incremented each micro second. |

All the dependend crates provide a feature ``ruspirp_pi3`` that, when active, ensures the correct
base address for MMIO mapped registers that allows access to the peripherals is used while compiling.

## :mailbox: The Kernel File

Even though the functionality we will implement in Rust could be organized accross different files
and folders ( so called modules), there is one main file, given in the ``Cargo.toml`` section
``[[bin]]`` that is carrying our entry point into our bare metal kernel. Let's take a look into the
structure of this file.

At the very beginning we provide two compiler attributes:
```rust
#![no_std]
#![no_main]
```
> :bulb: Both are quite important:<br>
> The first one tells the Rust compiler that we do not want to use the Rust standard library. With
> this we get only access to the ``core`` functions and features of Rust (check-out the 
> [documentation](https://doc.rust-lang.org/core/) for details). The required functions the
> documentation assumes to be there are provided by the ``ruspiro-boot`` crate. The second one 
> ensures that the Rust compiler and linker should not expect a ``main`` function to be present.
> The ``ruspiro-boot`` crate provides the necessary entry point.

The next part is to refer to the dependent crates to have access to the functions they provide:
```rust
#[macro_use]
extern crate ruspiro_boot;          // link in the bootstrap functions
extern crate ruspiro_allocator;     // link in the custom heap allocator
use ruspiro_gpio::GPIO;             // provide access to the GPIO api
use ruspiro_timer as timer;         // provide access to the timer function with an alias 'timer' 
```

After all the declarations its now time to implement the functionality we would like our kernel to
perform. To provide the implementations for the entry points the bootstrapper is calling the
``ruspiro-boot`` comes with 2 macros. So you define 2 functions, one for a one-time initialization
and a second one for the main processing loop. Then you use the afformentioned macros to "mark" those
functions as the required entry points. This looks like this in code:
```rust
// Set the function that is called on each core once it is alive and prepared to branch
// into the Rust 'world'
come_alive_with!(alive);

/// Any one-time initialization might be done here.
fn alive(_core: u32) {
    // nothing to do at this time...
}

// Set the function that is called on each core after the ``come_alive_with`` function has
// finished it's preparation. This function is intended to never return as there is nothing
// to be executed on the cores once this kernel has done what it is supposed to
run_with!(running);

/// Do the actual work on any core
fn running(core: u32) -> ! {
    // based on the core provided use a different GPIO pin to blink a different LED
    let pin = match core {
        0 => GPIO.take_for(|gpio| gpio.get_pin(17)).unwrap().to_output(),
        1 => GPIO.take_for(|gpio| gpio.get_pin(18)).unwrap().to_output(),
        2 => GPIO.take_for(|gpio| gpio.get_pin(20)).unwrap().to_output(),
        3 => GPIO.take_for(|gpio| gpio.get_pin(21)).unwrap().to_output(),
        _ => unreachable!()
    };

    // now blink the LED with an intervall based on the core number to visualize this is really the different core
    // blinking the LED
    loop {
        pin.high();
        timer::sleep(10000 + 10000*core as u64);
        pin.low();
        timer::sleep(15000 + 5000*core as u64);
    } // never return here...
}
```

This code basically aquires - based on the core id the functions enters with - a dedicated GPIO pin
and configures the same as ``Output``. This allows to set the pin to ``high()`` which will lit the 
LED and to ``low()`` which will clear the LED respectively. The loop will apply some core number 
specific sleep intervall to let the LED for each core blink in a bit different intervall.

> :warning: **HINT**: Please ensure to put a resistor between the LED and ground when connecting the
> LED to the GPIO pin.


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
Now you could put this card into the Raspberry Pi and power it up. If you properly connected the
LED's to the GPIO pins 17, 18, 20 and 21 they should blink. **HEUREKA**. This way of deploying
requires you to remove the SD card, update the kernel image file and then insert it again into the
Raspberry Pi for a next round of testing. This "SD-Card-Dance" will become cumbersome quite soon.

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

## :control_knobs: To many cores ?
The bootstrapping of any bare metal kernel is by default kicking off all 4 cores of the Raspberry Pi.
If you'd rather like to use only a single core as this might make testing easier you could activate
the `singlecore` feature for the `ruspiro-boot` crate in the `[dependencies]` section of the
`Cargo.toml`file.
```toml
[dependencies]
ruspiro-boot = { version = "0.3", features = ["ruspiro_pi3", "singlecore"] }
```