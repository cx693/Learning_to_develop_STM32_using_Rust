#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use stm32f1xx_hal::{
    adc,
    pac,
    prelude::*,
    rcc,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("内部 ADC 温度传感器测试");
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
    rprintln!("sysclk freq: {}", rcc.clocks.sysclk());
    rprintln!("adc freq: {}", rcc.clocks.adcclk());

    let mut adc1 = adc::Adc::new(dp.ADC1, &mut rcc);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, rcc.clocks.sysclk().raw());

    loop {
        let temp = adc1.read_temp();
        rprintln!("temp: {} C", temp);
        delay.delay_ms(1000u32);
    }
}