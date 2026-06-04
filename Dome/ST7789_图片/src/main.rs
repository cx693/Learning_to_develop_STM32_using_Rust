#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

mod cube3d;
mod font_ascii;
mod font_cn;
mod picture_data;
mod st7789;

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    rcc,
    gpio,
    spi::{Mode, Phase, Polarity},
};

const SPI_MODE: Mode = Mode {
    phase: Phase::CaptureOnSecondTransition,
    polarity: Polarity::IdleHigh,
};

type Spi1 = stm32f1xx_hal::spi::Spi<pac::SPI1, u8, gpio::Floating>;
type Display = st7789::ST7789<
    gpio::Pin<'A', 0, gpio::Output<gpio::PushPull>>,
    gpio::Pin<'A', 1, gpio::Output<gpio::PushPull>>,
>;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("ST7789 GIF 动画");

    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    let mut rcc_cfg = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .adcclk(14.MHz()),
        &mut flash.acr,
    );
    let clocks = rcc_cfg.clocks.clone();
    let mut gpioa = dp.GPIOA.split(&mut rcc_cfg);

    let dc = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let rst = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);

    let mut spi: Spi1 = dp.SPI1.spi(
        (Some(gpioa.pa5), pac::SPI1::NoMiso, Some(gpioa.pa7)),
        SPI_MODE,
        18.MHz(),
        &mut rcc_cfg,
    );

    let mut display: Display = st7789::ST7789::new(dc, rst);
    display.init(&mut spi, &clocks).unwrap();
    rprintln!("ST7789 初始化完成");

    display.fill_screen(&mut spi, 0x0000).unwrap();

    let pic_w = picture_data::PICTURE_64X64_WIDTH;
    let pic_h = picture_data::PICTURE_64X64_HEIGHT;
    let frames = picture_data::PICTURE_64X64_FRAMES;
    let x = (240 - pic_w) / 2;
    let y = (240 - pic_h) / 2;

    rprintln!("GIF: {}帧 {}x{} at ({},{})", frames, pic_w, pic_h, x, y);

    let mut frame_idx: usize = 0;
    loop {
        display.draw_picture(
            &mut spi, x, y,
            &picture_data::PICTURE_64X64[frame_idx],
            pic_w, pic_h,
        ).unwrap();

        frame_idx += 1;
        if frame_idx >= frames {
            frame_idx = 0;
        }

        cortex_m::asm::delay(72_000 * 80);
    }
}
