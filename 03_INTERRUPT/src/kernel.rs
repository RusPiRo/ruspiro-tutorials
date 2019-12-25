/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/
#![no_std]
#![no_main]

//! # Interrupt handling
//! 
//! This is demonstrating how to implement an interrupt handler for a specific interrupt that could
//! be raised by either peripheral or built in device like the ARM timer.
//! We will configure the system timer to raise an interrupt in specific intervalls
//! and implement a handler to do stuff whenever the interrupt was raised.
//! To properly configure the system timer we would need several MMIO registers that will be
//! defined with the help of the macros available in the [ruspiro-register crate](https://crates.io/crates/ruspiro-register)

#[macro_use]
extern crate ruspiro_boot;
extern crate ruspiro_allocator;

// use the functions of the interrupt crate
use ruspiro_interrupt::*;
// use the macro to conviniently define a MMIO register
use ruspiro_register::define_mmio_register;

come_alive_with!(kernel_alive);
run_with!(kernel_run);

pub fn kernel_alive(core: u32) {
    // your one-time initialization goes here
    println!("Kernel alive on core {}", core);
    if core == 0 {
        // when entering here with the main core we setup the timer
        TIMERLOAD::Register.set(0x30_000); // timer re-load value
        // configure the timer being enabled and raising interrupts
        TIMERCTRL::Register.write_value(
            TIMERCTRL::WIDTH::_32Bit
                | TIMERCTRL::IRQ::ENABLED
                | TIMERCTRL::TIMER::ENABLED
                | TIMERCTRL::FREERUN::ENABLED
        );
        // if the timer has been configured we could activate the timer interrupt to be handled
        // by the global interrupt manager and globally activate interrupts
        IRQ_MANAGER.take_for(|irq_mgr| irq_mgr.activate(Interrupt::ArmTimer));
        enable_interrupts();
    }
}

pub fn kernel_run(core: u32) -> ! {
    // your processing logic goes here
    println!("Kernel running on core {}", core);
    // never return from here...
    loop {}
}

/// Implement the interrupt handler with a specific function attribute/decorator
/// Checkout the [ruspiro-interrupt](https://docs.rs/ruspiro-interrupt/0.3.0/ruspiro_interrupt/irqtypes/enum.Interrupt.html) documentation
/// for all available interrupt types a handler could be implemented for
#[IrqHandler(ArmTimer)]
fn my_timer_handler() {
    // first thing to do is to acknowledge the timer interrupt to clear the interrupt line
    // this is done by writing any value to the acknowledge register
    TIMERACKN::Register.set(0x1);
    // now we are able to perform whatever we want at this interrupt, keeping in mind to return
    // from the handler as soon as possible
    // for the sake of simplicity we just write stuff to the console. BUT be careful:
    // println! will lock the console to be used which might lead to deadlocks in case this interrupt
    // was raised in the middle of another println! However, as we are sure there is no other core
    // doing this, and this is the only active code line at this moment we are on the safe side...
    println!("timer raised");
}

// Define the MMIO registers to be used with the timer, those might go into the
// [ruspiro-timer crate](https://crates.io/crates/ruspiro-timer) in one of the next releases.
define_mmio_register!(
    // the timer load value register, the base address depends on the raspberry model this
    // binare is build for. We assume the model 3(B, B+) here.
    TIMERLOAD<ReadWrite<u32>@(0x3F00_B400)>,
    // the timer control register
    TIMERCTRL<ReadWrite<u32>@(0x3F00_B408)> {
        // with of the timer counting value
        WIDTH OFFSET(1) [
            _16Bit: 0,
            _32Bit: 1
        ],
        // flag to enable interrupts raised by the timer
        IRQ OFFSET(5) [
            ENABLED: 1,
            DISABLED: 0
        ],
        // flag to enable the timer
        TIMER OFFSET(7) [
            ENABLED: 1,
            DISABLED: 0
        ],
        // flag to enable free-running counter of the timer
        FREERUN OFFSET(9) [
            ENABLED: 1,
            DISABLED: 0
        ]
    },
    // timer interrupt acknowledge register
    TIMERACKN<WriteOnly<u32>@(0x3F00_B40C)>
);