/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/
#![no_std]
#![no_main]

//! # Hello World
//! 
//! This is the initial RusPiRo tutorial. It's the bare metal version of a "Hello World" programm greeting the world
//! by blinking a LED. It's intention is - while limited in functionality - to verify the tools and programs are 
//! properly installed and configured to build a running bare metal kernel for the Raspberry Pi.
//! 
//! The Raspberry Pi contains 4 cores that will execute independently from each other. So we assigned a dedicated GPIO 
//! pin to each core. If using all 4 cores is not required adjust the `ruspiro-boot` dependency to
//! activiate the `singlecore` feature like so:
//! ```toml
//! [dependencies]
//! ruspiro-boot = { version = "0.3", features = ["ruspiro_pi3", "singlecore"] }
//! ```
//! 

#[macro_use]
extern crate ruspiro_boot;
extern crate ruspiro_allocator;

use ruspiro_gpio::GPIO;
use ruspiro_timer as timer;
use ruspiro_mmu as mmu;

// Set the function that is called on each core once it is alive and prepared to branch
// into the Rust 'world'
come_alive_with!(alive);

/// Any one-time initialization might be done here.
fn alive(core: u32) {
    // nothing to do at this time...
    // configure the mmu as we will deal with atomic operations (within the memory
    // allocator that is used by the isr channel under the hood to store the data
    // within the HEAP)
    // use some arbitrary values for VideoCore memory start and size. This is fine
    // as we will use a small lower amount of the ARM memory only.
    unsafe { mmu::initialize(core, 0x3000_0000, 0x001_000) };
}

// Set the function that is called on each core after the ``come_alive_with`` function has
// finished it's preparation. This function is intended to never return as there is nothing
// to be executed on the cores once this kernel has done what it is supposed to
run_with!(running);

/// Do the actual work on any core
fn running(core: u32) -> ! {
    // based on the core provided use a different GPIO pin to blink a different LED
    let pin = match core {
        0 => GPIO.with_mut(|gpio| gpio.get_pin(17)).unwrap().into_output(),
        1 => GPIO.with_mut(|gpio| gpio.get_pin(18)).unwrap().into_output(),
        2 => GPIO.with_mut(|gpio| gpio.get_pin(20)).unwrap().into_output(),
        3 => GPIO.with_mut(|gpio| gpio.get_pin(21)).unwrap().into_output(),
        _ => unreachable!()
    };

    // now blink the LED with an intervall based on the core number to visualize this is really the different core
    // blinking the LED
    loop {
        pin.high();
        timer::sleep(timer::Duration::from_millis(100*(core + 1) as u64));
        pin.low();
        timer::sleep(timer::Duration::from_millis(50*(core + 1) as u64));
    } // never return here...
}