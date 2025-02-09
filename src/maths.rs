use crate::base::IFixedPoint;
use core::ops::{Add, Mul, Range, Sub};

/// Smootherstep
pub fn smoother_step<T>(input: T) -> T
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<i32, Output = T> + Copy,
{
    let input2 = input * input;
    let input3 = input2 * input;
    input3 * input2 * 6i32 - input2 * input2 * 15 + input3 * 10
}

/// Outputs a triangle function centered at input
/// Generates a output slice that is OUTPUT_WIDTH wide containing a triangle function
pub fn triangle<const OUTPUT_WIDTH: usize>(
    center: IFixedPoint,
    half_width: IFixedPoint,
    value: IFixedPoint,
) -> [IFixedPoint; OUTPUT_WIDTH] {
    // Draw a triangle function
    let mut output: [IFixedPoint; OUTPUT_WIDTH] = [IFixedPoint::ZERO; OUTPUT_WIDTH];

    let x0: i32 = (center - half_width).floor().to_num();
    let x1: i32 = (center + half_width).ceil().to_num();

    // Triangle function = value * (1-dist)/width
    for x in x0..=x1 {
        // Skip if not in output range
        if x < 0 || x >= OUTPUT_WIDTH as i32 {
            continue;
        }

        let xi = IFixedPoint::from_num(x);
        let mut dist = xi - center;
        if dist < 0 {
            dist *= -1;
        }
        output[x as usize] = (IFixedPoint::ONE - (dist / half_width)) * value;
    }

    output
}

/// Provides random number in range generation for IFixedPoint
pub trait RandomIFixed {
    fn rand_range_ifixed(&mut self, range: Range<IFixedPoint>) -> IFixedPoint;
}

/// Provides random item selection
pub trait RandomItem {
    fn rand_choice<'a, T>(&mut self, items: &'a [T]) -> &'a T;
}

impl RandomIFixed for oorandom::Rand32 {
    /// Generates a random value in a range
    fn rand_range_ifixed(&mut self, range: Range<IFixedPoint>) -> IFixedPoint {
        // Generate a number between 0 and 1 in the fixed point format
        // TBD: Is from_bits portable to use in this way?
        let max_frac = 2u32.pow(IFixedPoint::FRAC_NBITS);
        let frac = self.rand_range(0..max_frac) as i32;
        let frac = IFixedPoint::from_bits(frac);

        // Output a value in the specified range using that random
        range.start + (range.end - range.start) * frac
    }
}

impl RandomItem for oorandom::Rand32 {
    fn rand_choice<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        let index = self.rand_range(0..items.len() as u32) as usize;
        &items[index]
    }
}
