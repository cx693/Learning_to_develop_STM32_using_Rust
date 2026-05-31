#![allow(clippy::empty_loop)]
#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{pac, prelude::*, rcc};
use cortex_m_rt::{entry}; 

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain(); // Flash 等待周期配置

    // 外部晶振
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()) // 使用 8MHz 外部晶振
            .sysclk(72.MHz()) // PLL 倍频到 72MHz
            .pclk1(36.MHz()) // APB1 分频到 36MHz
            .pclk2(72.MHz()) // APB2 不分频
            .adcclk(14.MHz()), // ADC 14MHz
        &mut flash.acr,
    );

    let mut gpioc = dp.GPIOC.split(&mut rcc);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut delay = dp.TIM2.delay_us(&mut rcc);

    loop {
        rprintln!("TIM2 定时器");
        led.set_high();
        delay.delay_ms(1_800_u16);

        led.set_low();
        delay.delay(1.secs());
    }
}