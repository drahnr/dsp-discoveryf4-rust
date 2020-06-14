//! This project is used for explaining the DTFSE operation. Here, we have a
//! periodic square signal. The complex form of this signal is represented with
//! s_complex array. DTFSE coefficients are calculated, then, the signal is
//! approximated with the DTFSE function. This function returns its output in
//! real form because original signal has only real parts in this example. The
//! result is kept in the y_real array.
//!
//! Requires cargo embed `cargo install cargo-embed`
//!
//! `cargo embed --example 4_8_dtfse_calculations`

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use crate::hal::{dwt::ClockDuration, dwt::DwtExt, prelude::*, stm32};
use cortex_m_rt::entry;
use jlink_rtt;
use microfft::{complex::cfft_512, Complex32};
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

use core::f32::consts::PI;
use heapless::consts::U16;
use itertools::Itertools;
use micromath::F32Ext;

const N: usize = 16;
const K: usize = 1;

const W1: f32 = core::f32::consts::PI / 128.0;
const W2: f32 = core::f32::consts::PI / 4.0;

fn DTFSE<'a>(size: usize, coeff: &'a [f32], ksize: usize) -> impl Iterator<Item = f32> + 'a {
    (0..size).map(move |n| {
        (0..ksize)
            .zip(coeff.iter().tuples())
            .map(|(k, (coeff0, coeff1))| {
                let A = (coeff0 * coeff0 + coeff1 * coeff1).sqrt();
                let P = (coeff1).atan2(*coeff0);
                A * ((2.0 * PI * k as f32 * n as f32 / size as f32) + P).cos() / size as f32
            })
            .sum::<f32>()
    })
}

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(168.mhz())
        .freeze();

    // Create a delay abstraction based on DWT cycle counter
    let dwt = cp.DWT.constrain(cp.DCB, clocks);

    let s_imag = [0f32; N];

    let s_real = (0..N)
        .map(|idx| if idx < N / 2 { 1.0 } else { 0.0 })
        .collect::<heapless::Vec<f32, U16>>();

    let mut s_complex = [0f32; 2 * N];
    s_real
        .iter()
        .zip(s_imag.iter().zip(s_complex.iter_mut().tuples()))
        .for_each(|(s_real, (s_imag, (s0, s1)))| {
            *s0 = *s_real;
            *s1 = *s_imag;
        });

    // Coefficient calculation with CFFT function
    // let mut DTFSEcoef = s_complex.clone();
    // let mut DTFSEcoef = [Complex32::default(); 512];
    // forward transform(not inverse), enables bit reversal of output(With it set to 0 the bins are all mixed up)
    // arm_cfft_f32(&arm_cfft_sR_f32_len16, DTFSEcoef, 0, 1);
    // let result = cfft_512(&mut DTFSEcoef);

    let time: ClockDuration = dwt.measure(|| {
        let y_real = DTFSE(N, &s_complex, K).collect::<heapless::Vec<f32, U16>>();
        dbgprint!("y_real: {:?}", &y_real[..]);
    });
    dbgprint!("ticks: {:?}", time.as_ticks());

    loop {}
}
