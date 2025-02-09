use smart_leds::RGB8;

use crate::{
    base::{IFixedPoint, LedPattern, LedStrip, STRIP_LEN},
    maths::{smoother_step, triangle},
};

pub struct SpeedChange {
    time: IFixedPoint,
    period: IFixedPoint,
    width: IFixedPoint,
    forward: bool,
}

impl Default for SpeedChange {
    fn default() -> Self {
        SpeedChange::new()
    }
}

impl SpeedChange {
    pub const fn new() -> SpeedChange {
        SpeedChange {
            time: IFixedPoint::ZERO,
            width: IFixedPoint::lit("5"),
            period: IFixedPoint::lit("10"),
            forward: true,
        }
    }

    fn update_time(&mut self, dt: IFixedPoint) -> IFixedPoint {
        self.time += dt;
        if self.time > self.period {
            self.time = IFixedPoint::ZERO;
            self.forward = !self.forward;
        }

        if self.forward {
            self.time / self.period
        } else {
            IFixedPoint::ONE - self.time / self.period
        }
    }
}

impl LedPattern for SpeedChange {
    fn update(&mut self, dt: IFixedPoint) -> LedStrip {
        let frac = self.update_time(dt);
        let frac = frac.clamp(IFixedPoint::ZERO, IFixedPoint::ONE);

        // Calculate 'smoother step' as an interpolation factor
        let ss = smoother_step(frac);

        // Now draw this position into the output as a triangle
        let tri = triangle::<STRIP_LEN>(ss, self.width, IFixedPoint::from_num(255));

        // Finally, copy the color scaled by this output
        let mut output: LedStrip = [RGB8::new(0, 0, 0); STRIP_LEN];
        for i in 0..STRIP_LEN {
            let col = smart_leds::hsv::Hsv {
                hue: 59,
                sat: 100,
                val: tri[i].to_num(),
            };
            output[i] = smart_leds::hsv::hsv2rgb(col);
        }

        output
    }
}
