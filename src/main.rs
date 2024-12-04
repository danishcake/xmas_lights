//! Blinks the 3 colour LEDs on a Pimoroni Plasma 2040 in sequence
#![no_std]
#![no_main]


use defmt::*;
use defmt_rtt as _;
use panic_halt as _;
use pimoroni_plasma_2040 as bsp;

/// The `#[rp2040_hal::entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables and the spinlock are initialised.
#[rp2040_hal::entry]
fn main() -> ! {
    info!("Start");
    loop {
    }
}
