#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

mod st7789;
mod cube3d;
mod font_ascii;

use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m::peripheral::DWT;
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

fn draw_fps(d: &mut Display, spi: &mut Spi1, fps: u32) {
    let mut x: u16 = 190;
    let digits = if fps >= 100 { 3 } else if fps >= 10 { 2 } else { 1 };
    let mut val = fps;
    let mut buf = [0u8; 3];
    for i in (0..digits).rev() { buf[i] = (val % 10) as u8; val /= 10; }
    for i in 0..digits {
        let ch = (b'0' + buf[i]) as char;
        let idx = ch as usize - 32;
        if idx < 95 {
            let _ = d.draw_bitmap(spi, x, 204, &font_ascii::FONT_16X32[idx], 16, 32, 0xFFFF, 0x0000);
        }
        x += 16;
    }
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
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

    let mut spi = dp.SPI1.spi(
        (Some(gpioa.pa5), pac::SPI1::NoMiso, Some(gpioa.pa7)),
        SPI_MODE, 36.MHz(), &mut rcc_cfg,
    );

    let mut display: Display = st7789::ST7789::new(dc, rst);
    display.init(&mut spi, &clocks).unwrap();

    let mut dwt = cp.DWT;
    let mut dcb = cp.DCB;
    dcb.enable_trace();
    dwt.enable_cycle_counter();

    display.fill_screen(&mut spi, 0x0000).unwrap();

    let config = cube3d::CubeConfig {
        size: 60,
        face_colors: [0xF800, 0xFFE0, 0x07E0, 0x07FF, 0x001F, 0xF81F],
        edge_color: 0xFFFF,
        draw_edges: true,
        cx: 120,
        cy: 120,
        fov: 120,
        depth_offset: 180,
    };

    let mut cube = cube3d::Cube3D::new(config);
    cube.angle_x = 30;
    cube.angle_y = 50;

    let mut frame_count: u32 = 0;
    let mut last_cycles: u32 = DWT::cycle_count();
    let mut fps: u32 = 0;
    let mut last_fps_drawn: u32 = u32::MAX;

    loop {
        cube.step();
        cube.project_wireframe();

        if let Some(dr) = cube.take_dirty_rect() {
            let _ = display.draw_edges_in_rect(
                &mut spi, dr.x, dr.y, dr.w, dr.h,
                cube.get_edges(), cube.get_projected(),
                0x0000, cube.get_edge_color(),
            );
        }

        frame_count += 1;
        let now = DWT::cycle_count();
        let elapsed = now.wrapping_sub(last_cycles);
        if elapsed >= 72_000_000 {
            fps = frame_count;
            frame_count = 0;
            last_cycles = now;
        }
        if fps != last_fps_drawn {
            draw_fps(&mut display, &mut spi, fps);
            last_fps_drawn = fps;
        }
    }
}
