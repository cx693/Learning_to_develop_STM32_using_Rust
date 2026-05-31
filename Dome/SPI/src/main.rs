#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

mod st7789;

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    rcc,
    spi::{Mode, Phase, Polarity},
};
use st7789::ST7789;

const SPI_MODE: Mode = Mode {
    phase: Phase::CaptureOnSecondTransition,
    polarity: Polarity::IdleHigh,
};

fn rgb565(r: u8, g: u8, b: u8) -> u16 {
    ((r as u16 & 0xF8) << 8) | ((g as u16 & 0xFC) << 3) | (b as u16 >> 3)
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("ST7789 240x240 驱动测试");

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

    let clocks = rcc.clocks.clone();

    let mut gpioa = dp.GPIOA.split(&mut rcc);

    let sck = gpioa.pa5;
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7;
    let dc = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let rst = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);

    let mut spi = dp.SPI1.spi(
        (Some(sck), Some(miso), Some(mosi)),
        SPI_MODE,
        16.MHz(),
        &mut rcc,
    );

    rprintln!("SPI 初始化完成, 16MHz");

    let mut display = ST7789::new(dc, rst);

    rprintln!("ST7789 初始化中...");
    display.init(&mut spi, &clocks).unwrap();
    rprintln!("ST7789 初始化完成");

    let colors = [
        rgb565(255, 0, 0),
        rgb565(0, 255, 0),
        rgb565(0, 0, 255),
        rgb565(255, 255, 0),
        rgb565(0, 255, 255),
        rgb565(255, 0, 255),
        rgb565(255, 255, 255),
        rgb565(0, 0, 0),
    ];
    let color_names = ["红", "绿", "蓝", "黄", "青", "紫", "白", "黑"];

    let mut color_idx: usize = 0;

    loop {
        let c = colors[color_idx % colors.len()];
        rprintln!("填充颜色: {} #{:04X}", color_names[color_idx % color_names.len()], c);
        display.fill_screen(&mut spi, c).unwrap();
        delay_ms(2000, &clocks);
        color_idx += 1;
    }
}

fn delay_ms(ms: u32, clocks: &rcc::Clocks) {
    let cycles = clocks.sysclk().raw() / 1000 * ms;
    asm::delay(cycles);
}
