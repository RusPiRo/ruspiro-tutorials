# RusPiRo Minimal Template

This is the bare minimum template to get started with RusPiRo based bare metal kernel development for
the Raspberry Pi 3(B+).

## Features
This template uses the `ruspiro-boot` crate for the initial boot strapping and hand over to the rust
code line. From here it's up to you to implement the stuff you need in the `src/kernel.rs` file.

Without changing the `src/kernel.rs` file you could build this project into a valid Raspberry Pi
64Bit kernel. When run on the actual device the Uart console will display:

```
########## RusPiRo ----- Bootstrapper v0.3 @ Aarch64 ----- ##########
Kernel alive on core 0
Kernel alive on core 1
Kernel running on core 0
Kernel running on core 1
Kernel alive on core 2
Kernel alive on core 3
Kernel running on core 2
Kernel running on core 3
```

For help on how to build kernel binaries for the Raspberry Pi with rust checkout the 
[RusPiRo Tutorials](https://github.com/RusPiRo/ruspiro-tutorials)

## License
Licensed under Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0) or MIT ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)) at your choice.