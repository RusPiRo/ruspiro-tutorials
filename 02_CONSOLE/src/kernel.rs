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
//! This is the second RusPiRo tutorial. This time it's a real version of a "Hello World" programm.
//!
//!

#[macro_use]
extern crate ruspiro_boot;
extern crate ruspiro_allocator;

use ruspiro_uart::Uart1;
use ruspiro_console::*;
use ruspiro_mmu as mmu;

// Set the function that is called on each core once it is alive and prepared to branch
// into the Rust 'world'
come_alive_with!(alive);

/// Any one-time initialization might be done here.
fn alive(core: u32) {
    // configure the mmu as we will deal with atomic operations (within the memory
    // allocator that is used by the isr channel under the hood to store the data
    // within the HEAP)
    // use some arbitrary values for VideoCore memory start and size. This is fine
    // as we will use a small lower amount of the ARM memory only.
    unsafe { mmu::initialize(core, 0x3000_0000, 0x001_000) };

    if core == 0 {
        // setup UART and console
        let mut uart = Uart1::new();
        let _ = uart.initialize(250_000_000, 115_200);
        CONSOLE.with_mut(|console| console.replace(uart));
    }
    
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
