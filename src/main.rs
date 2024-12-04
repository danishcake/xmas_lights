#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_halt as _;
//use pimoroni_plasma_2040 as bsp;

// Pull in any important traits
use pimoroni_plasma_2040::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use pimoroni_plasma_2040::hal::pac;

// Import the Timer for Ws2812:
use pimoroni_plasma_2040::hal::timer::Timer;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use pimoroni_plasma_2040::hal;

// PIOExt for the split() method that is needed to bring
// PIO0 into useable form for Ws2812:
use pimoroni_plasma_2040::hal::pio::PIOExt;

// Import useful traits to handle the ws2812 LEDs:
use smart_leds::{SmartLedsWrite, RGB8};

// Import the actual crate to handle the Ws2812 protocol:
use ws2812_pio::Ws2812;

// Currently 3 consecutive LEDs are driven by this example
// to keep the power draw compatible with USB:
const STRIP_LEN: usize = 3;

/// The `#[rp2040_hal::entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables and the spinlock are initialised.
#[rp2040_hal::entry]
fn main() -> ! {
    info!("Start");

    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        pimoroni_plasma_2040::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = pimoroni_plasma_2040::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Setup a delay for the LED blink signals:
    let mut frame_delay =
        cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Create a count down timer for the Ws2812 instance:
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // Split the PIO state machine 0 into individual objects, so that
    // Ws2812 can use it:
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    // Instantiate a Ws2812 LED strip:
    let mut ws = Ws2812::new(
        pins.data.into_function(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    let mut leds: [RGB8; STRIP_LEN] = [(0, 0, 0).into(); STRIP_LEN];
    let mut frame = 0usize;

    loop {
        for i in 0..STRIP_LEN {
            if i == frame % STRIP_LEN {
                leds[i] = RGB8::new(255, 255, 255);
            } else {
                leds[i] = RGB8::new(0, 0, 0);
            }
        }
        ws.write(leds.clone().into_iter()).unwrap();

        // Advance frame
        frame = frame.wrapping_add(1);

        // Wait for 10ms
        frame_delay.delay_ms(500);
    }
}
