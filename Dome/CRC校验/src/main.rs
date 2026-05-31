#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, rcc};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("CRC 校验 Demo 启动");

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

    let mut crc = dp.CRC.new(&mut rcc);

    crc.reset();
    crc.write(0x12345678);
    let val = crc.read();
    rprintln!("单字 CRC: found={:08x}, expected={:08x}", val, 0xdf8a8a2b_u32);

    crc.reset();
    crc.write(0x00000001);
    crc.write(0x00000002);
    crc.write(0x00000003);
    let val = crc.read();
    rprintln!("多字 CRC: result={:08x}", val);

    crc.reset();
    let val = crc.read();
    rprintln!("复位后初始值: {:08x} (应为 ffffffff)", val);

    rprintln!("CRC 校验 Demo 完成");

    loop {}
}