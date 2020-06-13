//! This project is used for creating two different digital signals. One of
//! these signals is a periodic cosine wave and other one is aperiodic cosine
//! wave.
//!
//! Requires cargo embed
//! `cargo install cargo-embed`
//!
//! `cargo embed --example 2_7_periodic_signals`

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};
use cortex_m_rt::entry;
use jlink_rtt;
use panic_rtt as _;

macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut out = $crate::jlink_rtt::NonBlockingOutput::new();
            writeln!(out, $($arg)*).ok();
        }
    };
}

use micromath::F32Ext;

const N: usize = 100;
const W1: f32 = core::f32::consts::PI / 10.0;
const W2: f32 = 3.0 / 10.0;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let _cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();

    let _clocks = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(168.mhz())
        .freeze();

    let mut sinusoidal1 = [0f32; N];
    sinusoidal1
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = (W1 * (idx as f32)).cos());
    dbgprint!("sinusoidal1: {:?}", &sinusoidal1[..]);

    let mut sinusoidal2 = [0f32; N];
    sinusoidal2
        .iter_mut()
        .enumerate()
        .for_each(|(idx, val)| *val = (W2 * (idx as f32)).cos());
    dbgprint!("sinusoidal2: {:?}", &sinusoidal2[..]);

    loop {}
}