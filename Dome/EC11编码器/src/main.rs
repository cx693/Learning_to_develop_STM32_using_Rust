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
    rcc,
    timer::{pwm_input::QeiOptions, Timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("EC11 编码器 QEI 测试");

    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .adcclk(14.MHz()),
        &mut flash.acr,
    );

    let gpiob = dp.GPIOB.split(&mut rcc);

    let c1 = gpiob.pb6;
    let c2 = gpiob.pb7;

    let qei = Timer::new(dp.TIM4, &mut rcc).qei((c1, c2), QeiOptions::default());
    let mut delay = cp.SYST.delay(&rcc.clocks);

    loop {
        let before = qei.count();
        delay.delay_ms(1_000_u16);
        let after = qei.count();

        let elapsed = after.wrapping_sub(before) as i16;
        rprintln!("脉冲: {} 方向: {:?}", elapsed, qei.direction());
    }
}