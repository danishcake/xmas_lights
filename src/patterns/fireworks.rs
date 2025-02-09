//! Fireworks algorithm.
//!
//! Every 1s a firework is spawned from either edge. This travels to a random location
//! in the central area over the next 3-5 seconds. The second half of travel slows down.
//! When the firework arrives, it fades out and then explodes.
//! The explosion spreads to some width at full intensity, then fades out
//!
//! There can be up to 32 Fireworks
use fixed::traits::Fixed;
use smart_leds::RGB8;

use crate::{
    base::{IFixedPoint, LedPattern, LedStrip, STRIP_LEN, STRIP_LEN_FIXED},
    maths::{smoother_step, triangle, RandomIFixed, RandomItem},
};

/// Data for the travel phase of the firework
#[derive(Clone, Copy, Debug)]
struct TravelData {
    origin: IFixedPoint,
    dest: IFixedPoint,
    travel_time: IFixedPoint,
    time: IFixedPoint,
}

/// Data for the fade phase of the firework
#[derive(Clone, Copy, Debug)]
struct FadeData {
    position: IFixedPoint,
    fade_time: IFixedPoint,
    time: IFixedPoint,
}

/// Data for the explode phase of the firework
#[derive(Clone, Copy, Debug)]
struct ExplodeData {
    position: IFixedPoint,
    spread_time: IFixedPoint,
    time: IFixedPoint,
}

/// Data for the explode fade phase of the firework
#[derive(Clone, Copy, Debug)]
struct ExplodeFadeData {
    position: IFixedPoint,
    fade_time: IFixedPoint,
    time: IFixedPoint,
}

#[derive(Clone, Copy, Debug)]
enum Firework {
    NotUsed,
    Travel(TravelData),
    Fade(FadeData),
    Explode(ExplodeData),
    ExplodeFade(ExplodeFadeData),
}

/// The maximum number of fireworks
const MAX_FIREWORKS: usize = 32;
/// How often the fireworks spawn
const FIREWORK_SPAWN_PERIOD: IFixedPoint = IFixedPoint::ONE;

// The range the firework can travel
const MIN_DIST: IFixedPoint = IFixedPoint::lit("0.2").unwrapped_mul(STRIP_LEN_FIXED);
const MAX_DIST: IFixedPoint = IFixedPoint::lit("0.8").unwrapped_mul(STRIP_LEN_FIXED);

// The range of time the fireworks take to travel
const MIN_TIME: IFixedPoint = IFixedPoint::lit("3");
const MAX_TIME: IFixedPoint = IFixedPoint::lit("6");

/// Represents the Fireworks pattern
pub struct Fireworks {
    fireworks: [Firework; MAX_FIREWORKS],
    spawn_time: IFixedPoint,
    time: IFixedPoint,
    rand: oorandom::Rand32,
}

impl Fireworks {
    pub fn new() -> Fireworks {
        Fireworks {
            fireworks: [Firework::NotUsed; 32],
            spawn_time: FIREWORK_SPAWN_PERIOD,
            time: IFixedPoint::ZERO,
            // TODO: Random seed
            rand: oorandom::Rand32::new(0),
        }
    }

    fn spawn_firework(&mut self) {
        // Find an empty slot
        let slot = self
            .fireworks
            .iter_mut()
            .find(|p| matches!(p, Firework::NotUsed));

        if let Some(slot) = slot {
            // Start at either end
            let &(origin, direction) = self.rand.rand_choice(&[
                (IFixedPoint::ZERO, IFixedPoint::ONE),
                (STRIP_LEN_FIXED, IFixedPoint::NEG_ONE),
            ]);
            let dest = origin + self.rand.rand_range_ifixed(MIN_DIST..MAX_DIST) * direction;

            // Over a period of time
            let travel_time = self.rand.rand_range_ifixed(MIN_TIME..MAX_TIME);

            *slot = Firework::Travel(TravelData {
                time: IFixedPoint::ZERO,
                travel_time,
                origin,
                dest,
            });
        }
    }
}

impl LedPattern for Fireworks {
    fn update(&mut self, dt: IFixedPoint) -> LedStrip {
        // Spawn additional fireworks
        self.time += dt;
        if dt >= self.spawn_time {
            self.spawn_firework();
            self.time = IFixedPoint::ZERO;
        }

        // Tick active fireworks
        for firework in &mut self.fireworks {}

        let output = [RGB8::new(0, 0, 0); STRIP_LEN];
        output
    }
}
