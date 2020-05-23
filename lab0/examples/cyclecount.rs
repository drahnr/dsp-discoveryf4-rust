//! Led Blinky Roulette example using the DWT peripheral for timing.
//!
//! With cargo flash
//! `cargo install cargo-flash`
//!
//! `cargo flash --example roulette --release --chip STM32F407VGTx --protocol swd`

#![no_std]
#![no_main]

use cortex_m;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{dwt::ClockDuration, dwt::DwtExt, prelude::*, stm32};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();

    // Clock configuration is critical for RNG to work properly; otherwise
    // RNG_SR CECS bit will constantly report an error (if RNG_CLK < HCLK/16)
    // here we pick a simple clock configuration that ensures the pll48clk,
    // from which RNG_CLK is derived, is about 48 MHz
    let clocks = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(128.mhz())
        .freeze();

    // Create a delay abstraction based on DWT cycle counter
    let dwt = cp.DWT.constrain(cp.DCB, clocks);
    let mut delay = dwt.delay();

    //set a breakpoint and inspect
    let time: ClockDuration = dwt.measure(|| delay.delay_ms(100_u32));

    loop {}
}
