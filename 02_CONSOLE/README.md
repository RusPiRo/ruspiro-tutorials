# The Console
This time we will do a real ``Hello World!!`` - even in a bare metal environment. This time we will use the pre-packaged
``ruspiro-sdk`` crate to get access to the ``uart`` as well the ``console`` abstraction.

## Basic folder structure
The folder structure is similar to the one of the first example:
```
- 02_CONSOLE           // project root folder
    |- src              // here goes all the rust source files
        |- kernel.rs    // the current one and only rust source file containing our code to execute on the Raspberry Pi
    |- Cargo.toml       // the build configuration file to tell Rust how to build our bare metal binary and what we
                        // depend on (which crates to incorporate into the build)
    |- build.sh         // a small script to execute the rust build
    |- makefile         // a makefile to execute the rust build in case you prefer make over the shell script
    |- link.ld          // the linker script needed when building the kernel with ruspiro
    |- README.md        // well, this is the file you are currently reading, at this very moment :)

```

## Defining the dependencies

To build this kernel we will only refer to a single dependency. The [``ruspiro-sdk`` crate](https://crates.io/crates/ruspiro-sdk)
bundels the different RusPiRo crates for a more convinient usage. However, the ``ruspiro-sdk`` comes with some features
to conditional compile refrenced features in, based on the need by the using kernel. The ``ruspiro-sdk`` crate comes with the 
``ruspiro-boot`` crate as active by default so the rules how the Raspberry Pi is kicked of by our new kernel also apply.

```
[dependencies.ruspiro-sdk]
version = "0.2"
features = ["ruspiro_pi3", "with_console", "with_uart"]
```

## The Kernel File

At the very beginning we provide two compiler attributes (as usual):
```
#![no_std]
#![no_main]
```
They are quite important:
The first ine tells the Rust compiler that we do not want to use the Rust standard library. With this we get only access to the ``core`` functions and features of Rust (check-out the [documentation](https://doc.rust-lang.org/core/)). The required functions the documentation assumes to be there are provided by the ``ruspiro-sdk`` crate.
The second one ensures that the Rust compiler and linker should not expect a ``main`` function to be present. The ``ruspiro-sdk`` crate provides the necessary entry points.

The next part is to refer to the dependent crates to have access to the functions they provide:
```
use ruspiro_sdk::*;
```

The final bit's to implement the functionality we would like our kernel to cover is by defining the two fuctions that shall be called for one-time initialization and for running the kernel and tell the ``ruspiro-sdk`` crate what those functions are. May sound complicated but isn't thanks tht provided macros from the ``ruspiro-sdk`` crate.

First the function that is called once for each core (one core after another) as they have been kicked off by the boot sequence.
```
// lets do the initialization to use the console when the first core starts
    if core == 0 {
        // the following initialization part could be written in many different ways as working with ``Result``
        // return type is really flexible in Rust. I'd like to chain them as they give clear to follow flow what happens
        // after something was successfully executed, even if this kind of style is a bit verbose. There shouldn't be 
        // any performanc impact expected on the final binary regardless of the way the results are handled...

        // first use the mailbox to get the current core clock rate which is needed for the initialization
        // of the miniUART
        MAILBOX.take_for(|mb| mb.get_clockrate(ArmClockId::Core))
            .and_then(|core_rate| {
                // use the core rate for uart initialization
                let mut uart = uart::Uart1::new();
                // initialize the uart and in case of success map the uart itself into the Ok() return value
                uart.initialize(core_rate, 115_200).map(|_| uart)
            }).and_then(|uart| {
                // use the uart to attach it to the console abstraction for further usage by calls to ``println!`` and the like
                CONSOLE.take_for(|console| console.replace(uart));
                // if we got here, everything has worked out! So let's print something to the console
                println!("Hello World");
                // return Ok as everything went fine.
                Ok(())
            }).map_err(|_| {
                // well in case of any issue with this setup use a LED to visualize this
                GPIO.take_for(|gpio| gpio.get_pin(17))
                    .expect("unable to get pin 17")
                    .to_output()
                    .high();
            }).expect("unable to initialize the console");
    } else {
        // for any other core - assuming the initial setup went fine - we could just use the console abstraction
        // to print to it
        println!("Hello from core {}", core);
    }
```

The second function is the code that run's endlessly (until we switch off the Raspberry Pi):
```
// Set the function that is called on each core after the ``come_alive_with`` function has
// finished it's preparation. This function is intended to never return as there is nothing
// to be executed on the cores once this kernel has done what it is supposed to
run_with!(running);

/// Do the actual work on any core
fn running(core: u32) -> ! {
    // we would like to say that we have entered the "running" state of out kernel on each core
    // this time we use the ``info!`` macro. This will prefix the message with an 'I' and the name of the
    // module (mainly the rust file) the info has been printed from. This is very useful to easely know the source
    // of a console output when used for root cause analysis for example.
    info!("Core {} has entered running state :D", core);

    // there is nothing more to do this time, but feel free to add your own stuff
    loop { } // never return here...
}
```

**HINT**: Please ensure to put a resistor between the LED and ground when connecting the LED to the GPIO pin.


## Building the kernel

To build the kernel run the ``build.sh`` shell script or ``make all``:
```
> ./build.sh
```

```
> make all
```
from within the project folder to build the kernel image file.

This might take a while at the first attempt as it does download the dependend crates from [crates.io](https://crates.io) and does cross compile the Rust core library. As the build process is incremental by default the next times the build will be much more faster.

The result of a successful execution of the build is a file ``kernel7.img`` in the ``target`` subfolder of this package. This file could be put onto the SD card of your Raspberry Pi. For easy deployment refer to the tutorials "master" [README.md](../README.md).

## Expected result

When this kernel is successfully executed the terminal connected to the miniUART of the Raspberry Pi should display this:
```
Hello World
I: kernel7 - Core 0 has entered running state :D
Hello from core 1
I: kernel7 - Core 1 has entered running state :D
Hello from core 2
I: kernel7 - Core 2 has entered running state :D
Hello from core 3
I: kernel7 - Core 3 has entered running state :D
```

