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
//! by blinking a LED. It's intention is - while limited in functionality - to verify the tools and programs are properly
//! installed and configured to build a running bare metal kernel for the Raspberry Pi.
//!
//! The Raspberry Pi contains 4 cores that will execute independently from each other. So we assigned a dedicated GPIO pin
//! to each core. If using all 4 cores is not required adjust the `ruspiro-boot`dependency to
//! activiate the `singlecore`feature like so:
//! ```toml
//! [dependencies]
//! ruspiro-boot = { version = "0.3", features = ["ruspiro_pi3", "singlecore"] }
//! ```
//!

#[macro_use]
extern crate ruspiro_boot;
extern crate ruspiro_allocator;

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
