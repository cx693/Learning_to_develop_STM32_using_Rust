#![allow(dead_code)]

use stm32f1xx_hal::{
    pac,
    spi::{Spi, Error as SpiError},
    rcc::Clocks,
};

pub struct ST7789<DC, RST> {
    dc: DC,
    rst: RST,
    width: u16,
    height: u16,
}

impl<DC: embedded_hal::digital::OutputPin, RST: embedded_hal::digital::OutputPin> ST7789<DC, RST> {
    pub fn new(dc: DC, rst: RST) -> Self {
        ST7789 {
            dc,
            rst,
            width: 240,
            height: 240,
        }
    }

    pub fn init<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, clocks: &Clocks) -> Result<(), SpiError> {
        self.hard_reset(clocks);

        self.write_command(spi, 0x11)?;
        self.delay_ms(120, clocks);

        self.write_command(spi, 0x36)?;
        self.write_data(spi, &[0x00])?;

        self.write_command(spi, 0x3A)?;
        self.write_data(spi, &[0x05])?;

        self.write_command(spi, 0xB2)?;
        self.write_data(spi, &[0x0C, 0x0C, 0x00, 0x33, 0x33])?;

        self.write_command(spi, 0xB7)?;
        self.write_data(spi, &[0x35])?;

        self.write_command(spi, 0xBB)?;
        self.write_data(spi, &[0x19])?;

        self.write_command(spi, 0xC0)?;
        self.write_data(spi, &[0x2C])?;

        self.write_command(spi, 0xC2)?;
        self.write_data(spi, &[0x01])?;

        self.write_command(spi, 0xC3)?;
        self.write_data(spi, &[0x12])?;

        self.write_command(spi, 0xC4)?;
        self.write_data(spi, &[0x20])?;

        self.write_command(spi, 0xC6)?;
        self.write_data(spi, &[0x0F])?;

        self.write_command(spi, 0xD0)?;
        self.write_data(spi, &[0xA4, 0xA1])?;

        self.write_command(spi, 0xE0)?;
        self.write_data(spi, &[0xD0, 0x04, 0x0D, 0x11, 0x13, 0x2B, 0x3F, 0x54, 0x4C, 0x18, 0x0D, 0x0B, 0x1F, 0x23])?;

        self.write_command(spi, 0xE1)?;
        self.write_data(spi, &[0xD0, 0x04, 0x0C, 0x11, 0x13, 0x2C, 0x3F, 0x44, 0x51, 0x2F, 0x1F, 0x1F, 0x20, 0x23])?;

        self.write_command(spi, 0x21)?;

        self.write_command(spi, 0x29)?;
        self.delay_ms(20, clocks);

        Ok(())
    }

    fn delay_ms(&self, ms: u32, clocks: &Clocks) {
        let cycles = clocks.sysclk().raw() / 1000 * ms;
        cortex_m::asm::delay(cycles);
    }

    fn hard_reset(&mut self, clocks: &Clocks) {
        let _ = self.rst.set_low();
        self.delay_ms(10, clocks);
        let _ = self.rst.set_high();
        self.delay_ms(120, clocks);
    }

    fn write_command<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, cmd: u8) -> Result<(), SpiError> {
        let _ = self.dc.set_low();
        spi.write(&[cmd])
    }

    fn write_data<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, data: &[u8]) -> Result<(), SpiError> {
        let _ = self.dc.set_high();
        spi.write(data)
    }

    pub fn set_address_window<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, x0: u16, y0: u16, x1: u16, y1: u16) -> Result<(), SpiError> {
        self.write_command(spi, 0x2A)?;
        self.write_data(spi, &[
            (x0 >> 8) as u8,
            (x0 & 0xFF) as u8,
            (x1 >> 8) as u8,
            (x1 & 0xFF) as u8,
        ])?;

        self.write_command(spi, 0x2B)?;
        self.write_data(spi, &[
            (y0 >> 8) as u8,
            (y0 & 0xFF) as u8,
            (y1 >> 8) as u8,
            (y1 & 0xFF) as u8,
        ])?;

        self.write_command(spi, 0x2C)
    }

    pub fn fill_rect<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, x: u16, y: u16, w: u16, h: u16, color: u16) -> Result<(), SpiError> {
        if w == 0 || h == 0 {
            return Ok(());
        }

        self.set_address_window(spi, x, y, x + w - 1, y + h - 1)?;

        let hi = (color >> 8) as u8;
        let lo = (color & 0xFF) as u8;
        let pixel_pair = [hi, lo];
        let pixel_count = w as u32 * h as u32;

        let _ = self.dc.set_high();
        for _ in 0..pixel_count {
            spi.write(&pixel_pair)?;
        }

        Ok(())
    }

    pub fn fill_screen<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, color: u16) -> Result<(), SpiError> {
        self.fill_rect(spi, 0, 0, self.width, self.height, color)
    }

    pub fn set_pixel<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, x: u16, y: u16, color: u16) -> Result<(), SpiError> {
        self.fill_rect(spi, x, y, 1, 1, color)
    }

    pub fn draw_bitmap<PULL>(
        &mut self,
        spi: &mut Spi<pac::SPI1, u8, PULL>,
        x: u16, y: u16,
        bitmap: &[u8],
        width: u16, height: u16,
        color: u16, bg_color: u16,
    ) -> Result<(), SpiError> {
        let bytes_per_col = ((height + 7) / 8) as usize;
        self.set_address_window(spi, x, y, x + width - 1, y + height - 1)?;
        let _ = self.dc.set_high();
        for row in 0..height {
            for col in 0..width {
                let byte_idx = col as usize * bytes_per_col + row as usize / 8;
                let bit_idx = row % 8;
                let px = if byte_idx < bitmap.len() && (bitmap[byte_idx] & (1 << bit_idx) != 0) {
                    color
                } else {
                    bg_color
                };
                spi.write(&[(px >> 8) as u8, (px & 0xFF) as u8])?;
            }
        }
        Ok(())
    }

    pub fn draw_bitmap_scaled2x<PULL>(
        &mut self,
        spi: &mut Spi<pac::SPI1, u8, PULL>,
        x: u16, y: u16,
        bitmap: &[u8],
        width: u16, height: u16,
        color: u16, bg_color: u16,
    ) -> Result<(), SpiError> {
        let sw = width * 2;
        let sh = height * 2;
        let bytes_per_col = ((height + 7) / 8) as usize;
        self.set_address_window(spi, x, y, x + sw - 1, y + sh - 1)?;
        let _ = self.dc.set_high();
        let hi_c = (color >> 8) as u8;
        let lo_c = (color & 0xFF) as u8;
        let hi_b = (bg_color >> 8) as u8;
        let lo_b = (bg_color & 0xFF) as u8;
        for row in 0..height {
            let mut row_buf = [0u8; 8];
            for col in 0..width {
                let byte_idx = col as usize * bytes_per_col + row as usize / 8;
                let bit_idx = row % 8;
                let on = byte_idx < bitmap.len()
                    && (bitmap[byte_idx] & (1 << bit_idx) != 0);
                let (hi, lo) = if on { (hi_c, lo_c) } else { (hi_b, lo_b) };
                let ci = col as usize * 4;
                row_buf[ci % 8] = hi;
                row_buf[ci % 8 + 1] = lo;
                row_buf[ci % 8 + 2] = hi;
                row_buf[ci % 8 + 3] = lo;
            }
            for _ in 0..2 {
                for col in 0..width {
                    let ci = (col as usize * 4) % 8;
                    spi.write(&row_buf[ci..ci + 4])?;
                }
            }
        }
        Ok(())
    }
}
