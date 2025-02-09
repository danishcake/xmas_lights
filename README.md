# Christmas Lights Rust

## The board

We're using a Pimoroni Plasma 2040, which is an RP2040 derive board with useful circuitry for driving LEDs.

- Cortex-M0+ core
- thumbv6m instruction set
- 264kB of RAM
- 133MHz Dual Core CPU
- 2MB of flash

## Dev env setup

- Install Rust 1.79.0+
- Add the required toolchain `rustup target add thumbv6m-none-eabi`
- Install probe-rs

```bash
# Install cargo-binstall to avoid having to compile probe-rs-tools, which otherwise requires cmake
cargo install cargo-binstall
cargo binstall probe-rs-tools
```

- Install cargo-binutils

```bash
# Install binutils
cargo binstall cargo-binutils
rustup component add llvm-tools
```

- Connect the Raspberry Pi debug probe.
- Wire the probe to the debugged board.
  - Black to GND
  - Orange to CLK/SWC
  - Yellow to DATA/SWD
- Test it's all working OK with `probe-rs list`. You should see a debug probe

# Project setup

Add the basic crates

```bash
# Basic stuff
cargo add embedded-hal
cargo add cortex-m
cargo add cortex-m-rt
cargo add pimoroni-plasma-2040
cargo add rp2040-boot2
cargo add rp2040-hal
cargo add panic_halt

# Debugging support
cargo add defmt
cargo add defmt-rtt

```

Create a .cargo/config.toml to define the platform

```toml
[build]
target = "thumbv6m-none-eabi"

[target.thumbv6m-none-eabi]
rustflags = [
    "-C", "link-arg=--nmagic",
    "-C", "link-arg=-Tlink.x",
    "-C", "link-arg=-Tdefmt.x",
    "-C", "no-vectorize-loops",
]
runner = "probe-rs run --chip RP2040"

[env]
DEFMT_LOG = "trace"
```

Create a stub `main.rs`.

```rust
//! Blinks the 3 colour LEDs on a Pimoroni Plasma 2040 in sequence
#![no_std]
#![no_main]


// defmt is used for efficient logging
use defmt::*;
use defmt_rtt as _;
// panic halt causes panics to ... halt the CPU
use panic_halt as _;

// The BSP is required for reasons I don't understand
use pimoroni_plasma_2040 as bsp;

/// The `#[rp2040_hal::entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables and the spinlock are initialised.
#[rp2040_hal::entry]
fn main() -> ! {
    info!("Start");
    loop {
    }
}
```

## Deploying and debugging

You can deploy using `cargo run` or `probe-rs download --chip RP2040 target\thumbv6m-none-eabi\debug\xmas_2024` followed by `probe-rs reset --chip RP2040`. You probably only want to do this for release builds.

```bash
probe-rs download --chip RP2040 target/thumbv6m-none-eabi/release/xmas_2024
probe-rs reset --chip RP2040
```

To debug, simply hit F5. You'll start paused, so hit F5 again to start executing code
