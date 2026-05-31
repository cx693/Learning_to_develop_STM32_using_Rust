#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    dac::{DacExt, DacOut, DacPin},
    rcc,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("DAC数模转换测试");

    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .adcclk(14.MHz()),
        &mut flash.acr,
    );

    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let pa4 = gpioa.pa4.into_analog(&mut gpioa.crl);
    let pa5 = gpioa.pa5.into_analog(&mut gpioa.crl);

    let (mut ch1, _ch2) = dp.DAC.constrain((pa4, pa5), &mut rcc);
    ch1.enable();

    let val_0_3v: u16 = (0.3_f32 / 3.3_f32 * 4095.0_f32) as u16;
    let val_1_6v: u16 = (1.6_f32 / 3.3_f32 * 4095.0_f32) as u16;
    let val_3_1v: u16 = (3.1_f32 / 3.3_f32 * 4095.0_f32) as u16;

    let delay_cycles: u32 = 72_000_000 * 3;

    loop {
        ch1.set_value(val_0_3v);
        rprintln!("DAC CH1: 0.3V (raw: {})", val_0_3v);
        cortex_m::asm::delay(delay_cycles);

        ch1.set_value(val_1_6v);
        rprintln!("DAC CH1: 1.6V (raw: {})", val_1_6v);
        cortex_m::asm::delay(delay_cycles);

        ch1.set_value(val_3_1v);
        rprintln!("DAC CH1: 3.1V (raw: {})", val_3_1v);
        cortex_m::asm::delay(delay_cycles);
    }
}