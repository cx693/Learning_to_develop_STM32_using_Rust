#![allow(clippy::empty_loop)]
#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::{InputPin, OutputPin};
use nb::block;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{pac, prelude::*, rcc, timer::Timer};

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

    // 创建动态引脚（可以在运行时切换输入/输出模式）
    let mut pin = gpioc.pc13.into_dynamic(&mut gpioc.crh);

    let cp = cortex_m::Peripherals::take().unwrap();

    // TOOD:1
    let mut timer = Timer::syst(cp.SYST, &rcc.clocks).counter_hz();
    timer.start(6.Hz()).unwrap();

    // TOOD:2
    // let mut timer = cp.SYST.counter_hz(&rcc.clocks);
    // timer.start(5.Hz()).unwrap();

    // TOOD:3
    // let mut timer: stm32f1xx_hal::timer::SysCounter<72000000> = cp.SYST.counter(&mut rcc.clocks);
    // timer.start(200.millis()).unwrap();

    loop {
        pin.make_floating_input(&mut gpioc.crh);
        block!(timer.wait()).unwrap();
        rprintln!("{}", pin.is_high().unwrap());

        pin.make_push_pull_output(&mut gpioc.crh);
        pin.set_high().unwrap();
        block!(timer.wait()).unwrap();
        pin.set_low().unwrap();
        block!(timer.wait()).unwrap();
    }
}
