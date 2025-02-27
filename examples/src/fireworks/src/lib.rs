use pax_lang::api::{ArgsClick, ArgsWheel, EasingCurve, RuntimeContext};
use pax_lang::*;
use pax_std::primitives::{Ellipse, Frame, Group, Path, Rectangle, Text};

#[derive(Pax)]
#[main]
#[file("fireworks.pax")]
pub struct Fireworks {
    pub rotation: Property<f64>,
    pub ticks: Property<usize>,
}

const ROTATION_COEFFICIENT: f64 = 0.00010;

impl Fireworks {

    pub fn handle_scroll(&mut self, ctx: RuntimeContext, args: ArgsWheel) {
        let old_t = self.rotation.get();
        let new_t = old_t - args.delta_y * ROTATION_COEFFICIENT;
        self.rotation.set(f64::max(0.0,new_t));
    }

    pub fn handle_will_render(&mut self, ctx: RuntimeContext) {
        let old_ticks = self.ticks.get();
        self.ticks.set(old_ticks + 1);
    }
}
