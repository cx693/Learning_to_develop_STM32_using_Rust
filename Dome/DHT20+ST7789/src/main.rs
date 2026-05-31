#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

mod dht20;
mod font_ascii;
mod font_cn;
mod st7789;

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{
    gpio,
    i2c,
    pac,
    prelude::*,
    rcc,
    spi::{self, Mode, Phase, Polarity},
};

const SPI_MODE: Mode = Mode {
    phase: Phase::CaptureOnSecondTransition,
    polarity: Polarity::IdleHigh,
};

type Display = st7789::ST7789<
    gpio::Pin<'A', 0, gpio::Output<gpio::PushPull>>,
    gpio::Pin<'A', 1, gpio::Output<gpio::PushPull>>,
>;
type Spi1 = spi::Spi<pac::SPI1, u8, gpio::Floating>;

const BG: u16 = 0x0000;
const FG: u16 = 0xFFFF;
const C_TEMP: u16 = 0xFFE0;
const C_HUMI: u16 = 0x07FF;
const C_TITLE: u16 = 0x07FF;
const C_ERR: u16 = 0xF800;

const LABEL_X: u16 = 24;
const CN_W: u16 = 32;
const ASC_W: u16 = 16;
const H: u16 = 32;
const TEMP_Y: u16 = 40;
const HUMI_Y: u16 = 90;
const VAL_X: u16 = 120;
const VAL_W: u16 = 64;

fn draw_cn32(
    d: &mut Display, s: &mut Spi1,
    x: u16, y: u16, c: char, color: u16,
) -> Result<(), spi::Error> {
    if let Some(g) = font_cn::font_32x32(c) {
        d.draw_bitmap(s, x, y, g, 32, 32, color, BG)?;
    }
    Ok(())
}

fn draw_asc(
    d: &mut Display, s: &mut Spi1,
    x: u16, y: u16, c: char, color: u16,
) -> Result<(), spi::Error> {
    let code = c as usize;
    if code < 32 || code > 126 { return Ok(()); }
    let glyph = &font_ascii::FONT_16X32[code - 32];
    d.draw_bitmap(s, x, y, glyph, 16, 32, color, BG)
}

fn draw_str8(
    d: &mut Display, s: &mut Spi1,
    mut x: u16, y: u16, text: &str, color: u16,
) -> Result<(), spi::Error> {
    for ch in text.chars() {
        if (ch as u32) >= 32 && (ch as u32) <= 126 {
            let glyph = &font_ascii::FONT_16X32[ch as usize - 32];
            d.draw_bitmap(s, x, y, glyph, 16, 32, color, BG)?;
            x += 16;
        }
    }
    Ok(())
}

fn draw_decimal(
    d: &mut Display, s: &mut Spi1,
    x: u16, y: u16, val_x10: u16, color: u16,
) -> Result<(), spi::Error> {
    let v = if val_x10 > 999 { 999 } else { val_x10 };
    let int_part = v / 10;
    let dec_part = v % 10;
    draw_asc(d, s, x, y, (b'0' + (int_part / 10) as u8) as char, color)?;
    draw_asc(d, s, x + ASC_W, y, (b'0' + (int_part % 10) as u8) as char, color)?;
    draw_asc(d, s, x + ASC_W * 2, y, '.', color)?;
    draw_asc(d, s, x + ASC_W * 3, y, (b'0' + dec_part as u8) as char, color)
}

fn clear_val(d: &mut Display, s: &mut Spi1, y: u16) -> Result<(), spi::Error> {
    d.fill_rect(s, VAL_X, y, VAL_W, H, BG)
}

fn draw_layout(d: &mut Display, s: &mut Spi1) -> Result<(), spi::Error> {
    d.fill_screen(s, BG)?;
    draw_str8(d, s, 40, 4, "== DHT20 ==", C_TITLE)?;
    draw_cn32(d, s, LABEL_X, TEMP_Y, '温', FG)?;
    draw_cn32(d, s, LABEL_X + CN_W, TEMP_Y, '\u{5EA6}', FG)?;
    draw_asc(d, s, LABEL_X + CN_W * 2, TEMP_Y, ':', FG)?;
    draw_cn32(d, s, VAL_X + VAL_W, TEMP_Y, '\u{2103}', C_TEMP)?;
    draw_cn32(d, s, LABEL_X, HUMI_Y, '湿', FG)?;
    draw_cn32(d, s, LABEL_X + CN_W, HUMI_Y, '\u{5EA6}', FG)?;
    draw_asc(d, s, LABEL_X + CN_W * 2, HUMI_Y, ':', FG)?;
    draw_asc(d, s, VAL_X + VAL_W, HUMI_Y, '%', C_HUMI)
}

fn draw_err_msg(d: &mut Display, s: &mut Spi1) -> Result<(), spi::Error> {
    d.fill_rect(s, 24, 22, 192, 16, BG)?;
    draw_str8(d, s, 24, 22, "Sensor Error!", C_ERR)
}

fn clear_err_msg(d: &mut Display, s: &mut Spi1) -> Result<(), spi::Error> {
    d.fill_rect(s, 24, 22, 192, 16, BG)
}

#[allow(dead_code)]
enum Ui {
    Fresh,
    Data { t: u16, h: u16 },
    Err { t: u16, h: u16 },
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("DHT20 + ST7789 32x32");

    let dp = pac::Peripherals::take().unwrap();
    let _cp = cortex_m::Peripherals::take().unwrap();
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
    let res = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
    let mut spi = dp.SPI1.spi(
        (Some(gpioa.pa5), pac::SPI1::NoMiso, Some(gpioa.pa7)),
        SPI_MODE, 16.MHz(), &mut rcc_cfg,
    );
    let mut display = st7789::ST7789::new(dc, res);
    display.init(&mut spi, &clocks).unwrap();

    let mut gpiob = dp.GPIOB.split(&mut rcc_cfg);
    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    let i2c_bus = dp.I2C1.i2c(
        (scl, sda),
        i2c::Mode::standard(100_000.Hz()),
        &mut rcc_cfg,
    ).blocking(1000, 10, 1000, 1000, &clocks);

    let mut dht20_sensor = dht20::Dht20::new(i2c_bus);

    display.fill_screen(&mut spi, BG).unwrap();
    let _ = draw_str8(&mut display, &mut spi, 24, 100, "DHT20 32x32 Demo", C_TITLE);
    let _ = draw_str8(&mut display, &mut spi, 56, 140, "Wait...", 0x8410);
    if let Err(e) = dht20_sensor.init(&clocks) {
        rprintln!("DHT20 init: {:?}", e);
    }

    let mut ui = Ui::Fresh;

    loop {
        let mut res = Err(dht20::Error::I2c);
        for attempt in 0u8..3 {
            match dht20_sensor.read(&clocks) {
                Ok((h, t)) => { res = Ok((h, t)); break; }
                Err(e) => {
                    rprintln!("try{}: {:?}", attempt + 1, e);
                    cortex_m::asm::delay(clocks.sysclk().raw() / 1000 * 1500);
                }
            }
        }

        match (&ui, &res) {
            (Ui::Fresh, Ok((h, t))) => {
                let _ = draw_layout(&mut display, &mut spi);
                let _ = draw_decimal(&mut display, &mut spi, VAL_X, TEMP_Y, *t, C_TEMP);
                let _ = draw_decimal(&mut display, &mut spi, VAL_X, HUMI_Y, *h, C_HUMI);
                ui = Ui::Data { t: *t, h: *h };
                rprintln!("T:{}.{}C H:{}.{}%", t / 10, t % 10, h / 10, h % 10);
            }
            (Ui::Fresh, Err(_)) => {
                let _ = draw_layout(&mut display, &mut spi);
                let _ = draw_err_msg(&mut display, &mut spi);
                ui = Ui::Err { t: 0, h: 0 };
            }
            (Ui::Data { t, h }, Ok((nh, nt))) => {
                if *nt != *t {
                    let _ = clear_val(&mut display, &mut spi, TEMP_Y);
                    let _ = draw_decimal(&mut display, &mut spi, VAL_X, TEMP_Y, *nt, C_TEMP);
                }
                if *nh != *h {
                    let _ = clear_val(&mut display, &mut spi, HUMI_Y);
                    let _ = draw_decimal(&mut display, &mut spi, VAL_X, HUMI_Y, *nh, C_HUMI);
                }
                ui = Ui::Data { t: *nt, h: *nh };
                rprintln!("T:{}.{}C H:{}.{}%", nt / 10, nt % 10, nh / 10, nh % 10);
            }
            (Ui::Data { t, h }, Err(_)) => {
                let _ = draw_err_msg(&mut display, &mut spi);
                let _ = clear_val(&mut display, &mut spi, TEMP_Y);
                let _ = draw_decimal(&mut display, &mut spi, VAL_X, TEMP_Y, *t, C_TEMP);
                let _ = clear_val(&mut display, &mut spi, HUMI_Y);
                let _ = draw_decimal(&mut display, &mut spi, VAL_X, HUMI_Y, *h, C_HUMI);
                ui = Ui::Err { t: *t, h: *h };
            }
            (Ui::Err { .. }, Ok((h, t))) => {
                let _ = clear_err_msg(&mut display, &mut spi);
                let _ = clear_val(&mut display, &mut spi, TEMP_Y);
                let _ = draw_decimal(&mut display, &mut spi, VAL_X, TEMP_Y, *t, C_TEMP);
                let _ = clear_val(&mut display, &mut spi, HUMI_Y);
                let _ = draw_decimal(&mut display, &mut spi, VAL_X, HUMI_Y, *h, C_HUMI);
                ui = Ui::Data { t: *t, h: *h };
                rprintln!("T:{}.{}C H:{}.{}%", t / 10, t % 10, h / 10, h % 10);
            }
            (Ui::Err { .. }, Err(_)) => {}
        }

        cortex_m::asm::delay(clocks.sysclk().raw() / 1000 * 2000);
    }
}
