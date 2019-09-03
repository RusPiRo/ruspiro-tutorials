# The Blinking LED
The ``Hello World!!`` for bare metal environments. A **must** from my point to get things started and best suited to
verify the tools and the toolchain are smoothly working and creatding a run-able kernel for the Raspberry Pi. As the
Raspberry Pi that runs without any OS is not able to display a greeting message to a connected screen we use the provided
GPIO pins to connect a LED to it at let the Raspberry Pi blink this LED to greet the world.

## Basic folder structure
For new-comers lets start with the basic folder structure structure of our project:
```
- 01_BLINKLED           // project root folder
    |- src              // here goes all the rust source files
        |- kernel.rs    // the current one and only rust source file containing our code to execute on the Raspberry Pi
    |- Cargo.toml       // the build configuration file to tell Rust how to build our bare metal binary and what we
                        // depend on (which crates to incorporate into the build)
    |- build.sh         // a small script to execute the rust build
    |- makefile         // a makefile to execute the rust build in case you prefer make over the shell script
    |- link.ld          // the linker script needed when building the kernel with ruspiro
    |- README.md        // well, this is the file you are currently reading, at this very moment :)

```

## Defining the package
The maybe first thing to do is to describe the package we would like to build using the ``Cargo.toml`` file. The first *chapter* specifies some meta-data. The minimum required entries here are the name, the version and the edition. The ``publish = false`` prevents accidently publishing of this package/crate to the public crate library of Rust.
```
[package]
name = <any name you would like to give>
version = "0.0.0"
edition = "2018"
publish = false
```

The ``Cargo.toml`` of this repository does provide some more meta-data attributes to give you a clue what might be helpful to consider when defining your own packages/crates and have plans to publish them.

The second *chapter* defines the requested output Rust should create while building/compiling our package. This could either be a library or a binary. In our case we tent to build a binary to be deployed to the Raspberry Pi, so the configuration reflects this.

```
[[bin]]
name = "kernel7"        # name of the binary to build
path = "src/kernel.rs"  # main source file to build the binary from
```

Finally in the third *chapter* we list all dependencies to crates we would like to use with our package. Those packages/crates can be found on [crates.io](https://crates.io). You will also find all the ``ruspiro`` crates there with links to their repositories and more important to their documentation.

To start a lightweight first bare metal kernel for Raspberry Pi we would need the following dependencies to be configured:
```
[dependencies]
ruspiro-boot = { version = "0.2", features = ["ruspiro_pi3", "with_panic", "with_exception"] }
ruspiro-gpio = { version = "0.2", features = ["ruspiro_pi3"] }
ruspiro-timer = { version = "0.1", features = ["ruspiro_pi3"] }
```

What does those dependencies provide:

| Dependent crate<img width=200/>| Description |
|------------------|-------------|
| [``ruspiro-boot``](https://crates.io/crates/ruspiro-boot) | Booting the Raspberry Pi in a bare metal setup without an OS requires some initial assembly and preparation. This crate provides all the required boot code and is responsible to kick off the cores of the Raspberry Pi and branch into the code written in Rust. For implementing the functions in Rust the boot sequence is calling, macros are available. |
| [``ruspiro-gpio``](https://crates.io/crates/ruspiro-gpio) | This is the API crate to access the GPIO pins available with the Raspberry Pi. It hides the complexity of the setup and usage of the different pin's behind easy to consume function calls. |
| [``ruspiro-timer``](https://crates.io/crates/ruspiro-timer) | Simple timing functions to allow to pause execution for a specif amount of time. The timing is done based on the internal free running counter of the Raspberry Pi system timer that is incremented each micro second. |

All the dependend crates provide a feature ``ruspirp_pi3`` that, when active, ensures the proper base address for MMIO mapped registers that allows access to the peripherals
is used while compiling.

## The Kernel File

Even though the functionality we will implement in Rust could be organized accross different files and folders (modules), there is one main file, given in the ``Cargo.toml`` section ``[[bin]]`` that is carrying our entry point into our bare metal kernel. Let's take a look into the structure of this file.

At the very beginning we provide two compiler attributes:
```
#![no_std]
#![no_main]
```
They are quite important:
The first ine tells the Rust compiler that we do not want to use the Rust standard library. With this we get only access to the ``core`` functions and features of Rust (check-out the [documentation](https://doc.rust-lang.org/core/)). The required functions the documentation assumes to be there are provided by the ``ruspiro-boot`` crate.
The second one ensures that the Rust compiler and linker should not expect a ``main`` function to be present. The ``ruspiro-boot`` crate provides the necessary entry points.

The next part is to refer to the dependent crates to have access to the functions they provide:
```
use ruspiro_boot::*;
use ruspiro_gpio::GPIO;
use ruspiro_timer as timer;
```

The final bit's to implement the functionality we would like our kernel to cover is by defining the two fuctions that shall be called for one-time initialization and for running the kernel and tell the ``ruspiro-boot`` crate what those functions are. May sound complicated but isn't thanks tht provided macros from the ``ruspiro-boot`` crate.

First the function that is called once for each core (one core after another) as they have been kicked off by the boot sequence.
```
// Set the function that is called on each core once it is alive and prepared to branch
// into the Rust 'world'
come_alive_with!(alive);

/// Any one-time initialization might be done here.
fn alive(_core: u32) {
    // nothing to do at this time...
}
```

The second function is the code that runn's endlessly (until we switch off the Raspberry Pi):
```
// Set the function that is called on each core after the ``come_alive_with`` function has
// finished it's preparation. This function is intended to never return as there is nothing
// to be executed on the cores once this kernel has done what it is supposed to
run_with!(running);

/// Do the actual work on any core
fn running(core: u32) -> ! {
    // based on the core provided use a different GPIO pin to blink a different LED
    let pin = match core {
        0 => GPIO.take_for(|gpio| gpio.get_pin(17)).expect("pin 17 already in use?").to_output(),
        1 => GPIO.take_for(|gpio| gpio.get_pin(18)).expect("pin 18 already in use?").to_output(),
        2 => GPIO.take_for(|gpio| gpio.get_pin(20)).expect("pin 20 already in use?").to_output(),
        3 => GPIO.take_for(|gpio| gpio.get_pin(21)).expect("pin 21 already in use?").to_output(),
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

This code basically aquires - based on the core id the functions enters with - a dedicated GPIO pin and configures the same as ``Output``. This allows to set the pin to ``high()`` which will lit the LED and to ``low()`` which will clear the LED respectively. The loop will apply some core number specific sleep intervall to let the LED for each core blink in a bit different intervall.

**HINT**: Please ensure to put a resistor between the LED and ground when connecting the LED to the GPIO pin.


## Building the kernel

If all tools has been successfully configured ( as described here: [../README.md] ), bulding the kernel could be done by executing the script contained in this repository:
```
> ./build.sh
```

As an alternative, if you have ``make`` installed on your machine you could also call
```
> make all
```
from within the project folder to build the kernel image file.

This might take a while at the first attempt as it does download the dependend crates from [crates.io](https://crates.io) and does cross compile the Rust core library. As the build process is incremental by default the next times the build will be much more faster.

The result of a successful execution of the ``build.sh`` script is a file ``kernel7.img`` in the ``target`` subfolder of this package. This file could be put onto the SD card of your Raspberry Pi alongside the ``bootcode.bin`` and ``start.elf``. Putting this card then into the Raspberry Pi and powering on the same should blink the LED's if they are connected to the right GPIO pins.

