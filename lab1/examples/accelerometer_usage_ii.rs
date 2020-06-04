//! Led Blinky Roulette example using the DWT peripheral for timing.
//!
//! Requires cargo flash
//! `cargo install cargo-flash`
//!
//! `cargo flash --example roulette --release --chip STM32F407VGTx --protocol swd`

#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};
use cortex_m;
use cortex_m_rt::entry;
use panic_halt as _;

use crate::hal::spi;
use accelerometer::RawAccelerometer;
use lis302dl;

const N: usize = 1000;

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

    let gpioa = dp.GPIOA.split();
    let gpioe = dp.GPIOE.split();

    let sck = gpioa.pa5.into_alternate_af5().internal_pull_up(false);
    let miso = gpioa.pa6.into_alternate_af5().internal_pull_up(false);
    let mosi = gpioa.pa7.into_alternate_af5().internal_pull_up(false);

    let spi_mode = spi::Mode {
        polarity: spi::Polarity::IdleLow,
        phase: spi::Phase::CaptureOnFirstTransition,
    };

    let spi = spi::Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        spi_mode,
        10.mhz().into(),
        clocks,
    );

    let mut chip_select = gpioe.pe3.into_push_pull_output();
    chip_select.set_high().ok();

    let mut lis302dl = lis302dl::Lis302Dl::new(spi, chip_select, Default::default());

    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let mut buffer = [0i8; N];
    buffer.iter_mut().for_each(|buffer_ref| {
        *buffer_ref = lis302dl.accel_raw().unwrap().x;
        delay.delay_ms(10u8);
    });

    loop {}
}
