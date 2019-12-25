/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/
#![no_std]
#![no_main]

//! # The I2C Bus
//! 

#[macro_use]
extern crate ruspiro_boot;
extern crate ruspiro_allocator;

use ruspiro_i2c::*;

come_alive_with!(kernel_alive);
run_with!(kernel_run);

pub fn kernel_alive(core: u32) {
    // your one-time initialization goes here
    println!("Kernel alive on core {}", core);

    // if the main core is kicked off we can initialize the I2C bus and check for any device
    // connected to the I2C bus using it's [scan()] function
    if core == 0 {
        I2C.take_for(|i2c| {
            // initializing the I2C Bus assuming the default core speed of 250MHz
            i2c.initialize(250_000_000, true).unwrap();
            println!("scan I2C devices connected to RPi");
            let devices = i2c.scan().unwrap();
            for d in devices {
                // using the [info!] macro to write to the console will also print the module name
                // from where the message originates as a prefix to the text
                info!("device detected at 0x{:2X}", d);
            }
        });
    }
}

pub fn kernel_run(_core: u32) -> ! {
    // your processing logic goes here
    // never return from here...
    loop {}
}