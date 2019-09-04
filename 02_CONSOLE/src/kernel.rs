/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: MIT
 **********************************************************************************************************************/
#![no_std]
#![no_main]

//! # Console output
//! 
//! 
//! 
//! 
//! 

use ruspiro_sdk::*;

// Set the function that is called on each core once it is alive and prepared to branch
// into the Rust 'world'
come_alive_with!(alive);

/// Any one-time initialization might be done here.
fn alive(core: u32) {
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
}

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