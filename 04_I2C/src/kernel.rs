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

use ruspiro_console::*;
use ruspiro_i2c::*;
use ruspiro_mmu as mmu;
use ruspiro_uart::Uart1;

come_alive_with!(kernel_alive);
run_with!(kernel_run);

pub fn kernel_alive(core: u32) {
  // your one-time initialization goes here
  // configure the mmu as we will deal with atomic operations (within the memory
  // allocator that is used by the isr channel under the hood to store the data
  // within the HEAP)
  // use some arbitrary values for VideoCore memory start and size. This is fine
  // as we will use a small lower amount of the ARM memory only.
  unsafe { mmu::initialize(core, 0x3000_0000, 0x001_000) };
  // if the main core is kicked off we can initialize the I2C bus and check for any device
  // connected to the I2C bus using it's [scan()] function
  if core == 0 {
    // setup UART and console
    let mut uart = Uart1::new();
    let _ = uart.initialize(250_000_000, 115_200);
    CONSOLE.with_mut(|console| console.replace(uart));

    I2C.with_mut(|i2c| {
      // initializing the I2C Bus assuming the default core speed of 250MHz
      i2c.initialize(250_000_000, true).unwrap();
      println!("scan I2C devices connected to RPi");
      let devices = i2c.scan().unwrap();
      for d in devices {
        // using the [info!] macro to write to the console will also print the module name
        // from where the message originates as a prefix to the text
        println!("device detected at 0x{:2X}", d);
      }
    });
  }

  println!("Kernel alive on core {}", core);
}

pub fn kernel_run(_core: u32) -> ! {
  // your processing logic goes here
  // never return from here...
  loop {}
}
