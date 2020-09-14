# Changelog
## :banana: v0.5.2
  - ### :wrench: Maintenance
    Update the tutorials to use the latest versions of the dependent RusPiRo crates to run woth the current version of the Rust nightly compiler.

## :banana: v0.5.1
  - ### :wrench: Maintenance
    Use ``cargo-make`` cargo plugin to conviniently build the tutorials cross system. This reduces the maintenance efforts
    of the existing build scripts / makefiles.
    
## :banana: v0.5.0
  - ### :wrench: Maintenance
    It has been seen that the current Rust versions don't like crate names to start with numbers. So the tutorial cratenames where updated. In dditon - where applicable - the versions to the dependend ``ruspiro`` crates where updated and necessary code adopted to those versions inside the tutorials.
  - ### :book: Dokumentation
    Minor updates on the main README.md to reflect a better suited toolchain to be use. The ``aarch64-none-elf`` and ``arm-none-eabi`` with their corresponding Rust targets ``aarch64-unknown-none`` and ``armv7-none-eabi``

## :pizza: v0.4.0
  - ### :bulb: Features
    Added the following tutorials:<br>
    03_INTERRUPT<br>
    04_I2C<br>

## :carrot: v0.3.0
  - ### :bulb: Features
    Provide initial tutorials, able to be build in aarch32 and aarch64:<br>
    01_BLINKLED<br>
    02_CONSOLE<br>

  - ### :book: Documentation
    Provide actual documentation on Tutorial main level as well as specific to each tutorial
