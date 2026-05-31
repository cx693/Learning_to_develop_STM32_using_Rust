#![deny(unsafe_code)]
#![no_std]
#![no_main]

use nb::block;
use panic_halt as _;

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{flash, pac, prelude::*, rcc, timer::Timer};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain(); // Flash 等待周期配置

    // 方式 1: 简洁的配置方法
    // let mut rcc = dp.RCC.freeze(
    //     rcc::Config::hsi() // 使用内部 8MHz RC
    //         .sysclk(64.MHz()) // 系统时钟 64MHz
    //         .pclk1(32.MHz()) // APB1 时钟 32MHz
    //         .pclk2(64.MHz()) // APB2 时钟 64MHz
    //         .adcclk(8.MHz()), // ADC 时钟 8MHz
    //     &mut flash.acr,
    // );

    // 方式 2: 使用外部晶振 + PLL
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()) // 使用 8MHz 外部晶振
            .sysclk(72.MHz()) // PLL 倍频到 72MHz
            .pclk1(36.MHz()) // APB1 分频到 36MHz
            .pclk2(72.MHz()) // APB2 不分频
            .adcclk(14.MHz()), // ADC 14MHz
        &mut flash.acr,
    );

    // let rcc = dp
    //     .RCC
    //     .freeze(rcc::Config::hse(8.MHz()).sysclk(72.MHz()), &mut flash.acr);

    // 获取实际的时钟频率
    rprintln!("SYSCLK: {}", rcc.clocks.sysclk()); // 系统时钟
    rprintln!("HCLK:   {}", rcc.clocks.hclk()); // AHB 时钟
    rprintln!("PCLK1:  {}", rcc.clocks.pclk1()); // APB1 时钟
    rprintln!("PCLK2:  {}", rcc.clocks.pclk2()); // APB2 时钟
    rprintln!("ADCCLK: {}", rcc.clocks.adcclk()); // ADC 时钟
    rprintln!("USBCLK valid: {}", rcc.clocks.usbclk_valid()); // USB 时钟是否有效

    loop {}
}
