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
    rprintln!("ADC 电压采集");
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    // 配置时钟：HSE 8MHz，SYSCLK 72MHz，ADCCLK 14MHz
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .adcclk(14.MHz()),
        &mut flash.acr,
    );
    rprintln!("adc freq: {}", rcc.clocks.adcclk());

    // 初始化 ADC1
    let mut adc1 = adc::Adc::new(dp.ADC1, &mut rcc);

    // 配置 PB0 为模拟输入
    let mut gpiob = dp.GPIOB.split(&mut rcc);
    let mut ch0 = gpiob.pb1.into_analog(&mut gpiob.crl);

    // 初始化 SysTick 延时器
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, rcc.clocks.sysclk().raw());

    loop {
        let data: u16 = adc1.read(&mut ch0).unwrap();
        // 参考电压 3.3V，12 位 ADC (0-4095)
        let voltage_mv = data as u32 * 3300 / 4095;
        let voltage_v = voltage_mv as f32 / 1000.0;
        rprintln!("adc1: {}  |  {}mV  |  {:.3}V", data, voltage_mv, voltage_v);
        delay.delay_ms(600u32);
    }
}