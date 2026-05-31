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
    serial::{Config, Serial},
    rcc
};
use core::fmt::Write;  // 导入 Write trait

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("开始串口测试");
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
    let mut gpiob = dp.GPIOB.split(&mut rcc);

    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    let rx = gpiob.pb11;

    let serial = Serial::new(
        dp.USART3,
        (tx, rx),
        Config::default().baudrate(115200.bps()),
        &mut rcc,
    );

    let (mut tx, _rx) = serial.split();

    let mut number = 0;
    // 使用 write! 宏格式化输出
    writeln!(tx, "Hello formatted string {}", number).unwrap();
    // Windows 换行: write!(tx, "Hello formatted string {}\r\n", number)


    let mut delay = dp.TIM2.delay_us(&mut rcc); // 使用TIM2 实现

    loop {
        writeln!(tx, "Hello formatted string {}", number).unwrap();
        delay.delay_ms(2_000_u16);
        number += 1;
        rprintln!("调试反馈:Hello formatted string {}",number);
    }
}