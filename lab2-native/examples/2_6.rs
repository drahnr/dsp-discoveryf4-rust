//! This project is used for creating a digital signal which is sum of two
//! sinusoidal signals with different frequencies.
//!
//! Runs entirely locally without hardware. Rounding might be different than on
//! device. Except for when printing you must be vigilent to not become reliant
//! on any std tools that can't otherwise port over no no_std without alloc.
//!
//! `cargo run --example 2_6`

use textplots::{Chart, Plot, Shape};

use core::f32::consts::{FRAC_PI_4, PI};
use heapless::consts::U512;
// use itertools::Itertools;
use typenum::Unsigned;

fn main() {
    let w0 = (0..U512::to_usize()).map(|idx| (PI * idx as f32 / 128.0).sin());

    let w1 = (0..U512::to_usize()).map(|idx| (FRAC_PI_4 * idx as f32).sin());

    let y = w0
        .clone()
        .zip(w1.clone())
        .map(|(inny1, inny2)| inny1 + inny2);

    display::<U512, _>("w1:", y);
}

// Points isn't a great representation as you can lose the line in the graph,
// however while Lines occasionally looks good it also can be terrible.
// Continuous requires to be in a fn pointer closure which cant capture any
// external data so not useful without lots of code duplication.
fn display<N, I>(_name: &str, input: I)
where
    N: Unsigned,
    I: Iterator<Item = f32> + core::clone::Clone + std::fmt::Debug,
{
    // println!("{:?}: {:?}", name, input.clone().format(", "));
    let display = input
        .enumerate()
        .map(|(idx, y)| (idx as f32, y))
        .collect::<Vec<(f32, f32)>>();
    Chart::new(120, 60, 0.0, N::to_usize() as f32)
        .lineplot(Shape::Points(&display[..]))
        .display();
}