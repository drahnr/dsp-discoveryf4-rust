//! This project is used for creating five different basic digital signals: unit
//! pulse, unit step, unit ramp, exponential and sinusoidal.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over no no_std without alloc.
//!
//! `cargo run --example 2_1_basic_signals`

use itertools::Itertools;
use textplots::{Chart, Plot, Shape};
use typenum::Unsigned;

type N = heapless::consts::U10;

const A: f32 = 0.8;
const W0: f32 = core::f32::consts::PI / 5.0;

fn main() {
    // d[n]
    let unit_pulse = (0..(N::to_usize())).map(|n| if n == 0 { 1.0 } else { 0.0 });
    display::<N, _>("unit_pulse", unit_pulse);

    // u[n]
    let unit_step = core::iter::repeat(1.0).take(N::to_usize());
    display::<N, _>("unit_step", unit_step);

    // r[n]
    let unit_ramp = (0..(N::to_usize())).map(|n| n as f32);
    display::<N, _>("unit_ramp", unit_ramp);

    // e[n]
    let exponential = (0..(N::to_usize())).map(|n| A.powf(n as f32));
    display::<N, _>("exponential", exponential);

    // s[n]
    let sinusoidal = (0..(N::to_usize())).map(|n| (W0 * n as f32).sin());
    display::<N, _>("sinusoidal", sinusoidal);
}

// Points isn't a great representation as you can lose the line in the graph,
// however while Lines occasionally looks good it also can be terrible.
// Continuous requires to be in a fn pointer closure which cant capture any
// external data so not useful without lots of code duplication.
fn display<N, I>(name: &str, input: I)
where
    N: Unsigned,
    I: Iterator<Item = f32> + core::clone::Clone + std::fmt::Debug,
{
    println!("{:?}: {:.4?}", name, input.clone().format(", "));
    let display = input
        .enumerate()
        .map(|(n, y)| (n as f32, y))
        .collect::<Vec<(f32, f32)>>();
    Chart::new(120, 60, 0.0, N::to_usize() as f32)
        .lineplot(Shape::Points(&display[..]))
        .display();
}
