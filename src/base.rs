//! Defines common traits and types that all patterns use

use smart_leds::RGB8;

/// The number of LEDs in the strip
pub const STRIP_LEN: usize = 50;

/// The number of LEDS as a fixed point number
pub const STRIP_LEN_FIXED: IFixedPoint = IFixedPoint::const_from_int(STRIP_LEN as i32);

/// A strip of LEDs
pub type LedStrip = [RGB8; STRIP_LEN];

/// A fixed point type with roughly -1023 to 1024 range
pub type IFixedPoint = fixed::types::I10F22;
pub type UFixedPoint = fixed::types::U10F22;

/// A trait that all pattern implement
pub trait LedPattern {
    fn update(&mut self, dt: IFixedPoint) -> LedStrip;
}
