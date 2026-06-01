#![allow(dead_code)]

use stm32f1xx_hal::{
    pac,
    spi::{Spi, Error as SpiError},
    rcc::Clocks,
};

const ROW_BUF_LEN: usize = 480;

pub struct ST7789<DC, RST> {
    dc: DC,
    rst: RST,
    width: u16,
    height: u16,
    row_buf: [u8; ROW_BUF_LEN],
}

impl<DC: embedded_hal::digital::OutputPin, RST: embedded_hal::digital::OutputPin> ST7789<DC, RST> {
    pub fn new(dc: DC, rst: RST) -> Self {
        ST7789 { dc, rst, width: 240, height: 240, row_buf: [0u8; ROW_BUF_LEN] }
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
        cortex_m::asm::delay(clocks.sysclk().raw() / 1000 * ms);
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
        self.write_data(spi, &[(x0 >> 8) as u8, (x0 & 0xFF) as u8, (x1 >> 8) as u8, (x1 & 0xFF) as u8])?;
        self.write_command(spi, 0x2B)?;
        self.write_data(spi, &[(y0 >> 8) as u8, (y0 & 0xFF) as u8, (y1 >> 8) as u8, (y1 & 0xFF) as u8])?;
        self.write_command(spi, 0x2C)
    }

    fn fill_row_buf(&mut self, w: usize, color: u16) {
        let hi = (color >> 8) as u8;
        let lo = (color & 0xFF) as u8;
        let mut i = 0;
        while i < w * 2 {
            self.row_buf[i] = hi;
            self.row_buf[i + 1] = lo;
            i += 2;
        }
    }

    pub fn fill_rect<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, x: u16, y: u16, w: u16, h: u16, color: u16) -> Result<(), SpiError> {
        if w == 0 || h == 0 { return Ok(()); }
        self.set_address_window(spi, x, y, x + w - 1, y + h - 1)?;
        self.fill_row_buf(w as usize, color);
        let slice = &self.row_buf[..w as usize * 2];
        let _ = self.dc.set_high();
        for _ in 0..h { spi.write(slice)?; }
        Ok(())
    }

    pub fn fill_screen<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, color: u16) -> Result<(), SpiError> {
        self.fill_rect(spi, 0, 0, self.width, self.height, color)
    }

    pub fn draw_line<PULL>(
        &mut self,
        spi: &mut Spi<pac::SPI1, u8, PULL>,
        x0: i16, y0: i16, x1: i16, y1: i16,
        color: u16,
    ) -> Result<(), SpiError> {
        let mut x0 = x0; let mut y0 = y0;
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx: i16 = if x0 < x1 { 1 } else { -1 };
        let sy: i16 = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let hi = (color >> 8) as u8;
        let lo = (color & 0xFF) as u8;
        let mut batch_y = y0;
        let mut batch_min_x = x0;
        let mut batch_max_x = x0;
        loop {
            if y0 == batch_y {
                if x0 < batch_min_x { batch_min_x = x0; }
                if x0 > batch_max_x { batch_max_x = x0; }
            } else {
                self.flush_line_span(spi, batch_y, batch_min_x, batch_max_x, hi, lo)?;
                batch_y = y0;
                batch_min_x = x0;
                batch_max_x = x0;
            }
            if x0 == x1 && y0 == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy { err += dy; x0 += sx; }
            if e2 <= dx { err += dx; y0 += sy; }
        }
        self.flush_line_span(spi, batch_y, batch_min_x, batch_max_x, hi, lo)
    }

    #[inline(always)]
    fn flush_line_span<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, y: i16, x0: i16, x1: i16, hi: u8, lo: u8) -> Result<(), SpiError> {
        if y < 0 || y >= 240 { return Ok(()); }
        let mut l = if x0 < x1 { x0 } else { x1 };
        let mut r = if x0 > x1 { x0 } else { x1 };
        if l < 0 { l = 0; }
        if r >= 240 { r = 239; }
        let count = (r - l + 1) as usize;
        if count == 0 { return Ok(()); }
        self.set_address_window(spi, l as u16, y as u16, r as u16, y as u16)?;
        let _ = self.dc.set_high();
        let byte_len = count * 2;
        let mut i = 0;
        while i < byte_len { self.row_buf[i] = hi; self.row_buf[i + 1] = lo; i += 2; }
        spi.write(&self.row_buf[..byte_len])
    }

    pub fn fill_triangle<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, x0: i16, y0: i16, x1: i16, y1: i16, x2: i16, y2: i16, color: u16) -> Result<(), SpiError> {
        let mut verts = [(x0, y0), (x1, y1), (x2, y2)];
        if verts[0].1 > verts[1].1 { let t = verts[0]; verts[0] = verts[1]; verts[1] = t; }
        if verts[0].1 > verts[2].1 { let t = verts[0]; verts[0] = verts[2]; verts[2] = t; }
        if verts[1].1 > verts[2].1 { let t = verts[1]; verts[1] = verts[2]; verts[2] = t; }
        let (ax, ay) = verts[0]; let (bx, by) = verts[1]; let (cx, cy) = verts[2];
        if ay == cy { return Ok(()); }
        let hi = (color >> 8) as u8; let lo = (color & 0xFF) as u8;
        let mut x_ac = (ax as i32) << 16; let mut x_ab = (ax as i32) << 16; let mut x_bc = (bx as i32) << 16;
        let dx_ac = if cy != ay { (((cx as i32) - (ax as i32)) << 16) / ((cy as i32) - (ay as i32)) } else { 0 };
        let dx_ab = if by != ay { (((bx as i32) - (ax as i32)) << 16) / ((by as i32) - (ay as i32)) } else { 0 };
        let dx_bc = if cy != by { (((cx as i32) - (bx as i32)) << 16) / ((cy as i32) - (by as i32)) } else { 0 };
        for y in ay..by { if y >= 0 && y < 240 { let _ = self.draw_span(spi, y, x_ac, x_ab, hi, lo); } x_ac += dx_ac; x_ab += dx_ab; }
        for y in by..=cy { if y >= 0 && y < 240 { let _ = self.draw_span(spi, y, x_ac, x_bc, hi, lo); } x_ac += dx_ac; x_bc += dx_bc; }
        Ok(())
    }

    #[inline(always)]
    fn draw_span<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, y: i16, xl: i32, xr: i32, hi: u8, lo: u8) -> Result<(), SpiError> {
        let mut l = (xl >> 16) as i16; let mut r = (xr >> 16) as i16;
        if l > r { core::mem::swap(&mut l, &mut r); }
        if l < 0 { l = 0; } if r >= 240 { r = 239; }
        let count = (r - l + 1) as usize;
        if count == 0 { return Ok(()); }
        self.set_address_window(spi, l as u16, y as u16, r as u16, y as u16)?;
        let _ = self.dc.set_high();
        let mut i = 0; let byte_len = count * 2;
        while i < byte_len { self.row_buf[i] = hi; self.row_buf[i + 1] = lo; i += 2; }
        spi.write(&self.row_buf[..byte_len])
    }

    pub fn fill_quad<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, pts: &[[i16; 2]; 4], color: u16) -> Result<(), SpiError> {
        self.fill_triangle(spi, pts[0][0], pts[0][1], pts[1][0], pts[1][1], pts[2][0], pts[2][1], color)?;
        self.fill_triangle(spi, pts[0][0], pts[0][1], pts[2][0], pts[2][1], pts[3][0], pts[3][1], color)
    }

    pub fn draw_line_batch<PULL>(
        &mut self,
        spi: &mut Spi<pac::SPI1, u8, PULL>,
        edges: &[(usize, usize); 12],
        proj: &[[i16; 2]; 8],
        color: u16,
    ) -> Result<(), SpiError> {
        for &(a, b) in edges.iter() {
            self.draw_line(spi, proj[a][0], proj[a][1], proj[b][0], proj[b][1], color)?;
        }
        Ok(())
    }

    pub fn draw_edges_in_rect<PULL>(
        &mut self,
        spi: &mut Spi<pac::SPI1, u8, PULL>,
        rx: u16, ry: u16, rw: u16, rh: u16,
        edges: &[(usize, usize); 12],
        proj: &[[i16; 2]; 8],
        bg_color: u16,
        edge_color: u16,
    ) -> Result<(), SpiError> {
        if rw == 0 || rh == 0 { return Ok(()); }
        self.set_address_window(spi, rx, ry, rx + rw - 1, ry + rh - 1)?;
        let _ = self.dc.set_high();
        let bg_hi = (bg_color >> 8) as u8;
        let bg_lo = (bg_color & 0xFF) as u8;
        let fg_hi = (edge_color >> 8) as u8;
        let fg_lo = (edge_color & 0xFF) as u8;
        let byte_len = rw as usize * 2;
        let rx_i = rx as i16;
        let rx_end = rx_i + rw as i16;
        let ry_i = ry as i16;
        for row in 0..rh as usize {
            let screen_y = ry_i + row as i16;
            let mut i = 0;
            while i < byte_len {
                self.row_buf[i] = bg_hi;
                self.row_buf[i + 1] = bg_lo;
                i += 2;
            }
            for &(a, b) in edges.iter() {
                let (ax, ay) = (proj[a][0], proj[a][1]);
                let (bx, by) = (proj[b][0], proj[b][1]);
                let ey_min = if ay < by { ay } else { by };
                let ey_max = if ay > by { ay } else { by };
                if screen_y < ey_min || screen_y > ey_max { continue; }
                if ay == by {
                    let mut lx = if ax < bx { ax } else { bx };
                    let mut hx = if ax > bx { ax } else { bx };
                    if lx < rx_i { lx = rx_i; }
                    if hx >= rx_end { hx = rx_end - 1; }
                    let mut px = lx;
                    while px <= hx {
                        let idx = (px - rx_i) as usize * 2;
                        if idx + 1 < byte_len {
                            self.row_buf[idx] = fg_hi;
                            self.row_buf[idx + 1] = fg_lo;
                        }
                        px += 1;
                    }
                } else {
                    let num = (bx as i32 - ax as i32) * (screen_y as i32 - ay as i32);
                    let den = by as i32 - ay as i32;
                    let px = (ax as i32 + num / den) as i16;
                    if px >= rx_i && px < rx_end {
                        let idx = (px - rx_i) as usize * 2;
                        self.row_buf[idx] = fg_hi;
                        self.row_buf[idx + 1] = fg_lo;
                    }
                }
            }
            spi.write(&self.row_buf[..byte_len])?;
        }
        Ok(())
    }

    pub fn draw_bitmap<PULL>(&mut self, spi: &mut Spi<pac::SPI1, u8, PULL>, x: u16, y: u16, bitmap: &[u8], width: u16, height: u16, color: u16, bg_color: u16) -> Result<(), SpiError> {
        let bytes_per_col = ((height + 7) / 8) as usize;
        self.set_address_window(spi, x, y, x + width - 1, y + height - 1)?;
        let _ = self.dc.set_high();
        let hi_c = (color >> 8) as u8; let lo_c = (color & 0xFF) as u8;
        let hi_b = (bg_color >> 8) as u8; let lo_b = (bg_color & 0xFF) as u8;
        let w = width as usize;
        for row in 0..height {
            for col in 0..w {
                let byte_idx = col * bytes_per_col + row as usize / 8;
                let bit_idx = row % 8;
                let on = byte_idx < bitmap.len() && (bitmap[byte_idx] & (1 << bit_idx) != 0);
                let ci = col * 2;
                if on { self.row_buf[ci] = hi_c; self.row_buf[ci + 1] = lo_c; }
                else  { self.row_buf[ci] = hi_b; self.row_buf[ci + 1] = lo_b; }
            }
            spi.write(&self.row_buf[..w * 2])?;
        }
        Ok(())
    }
}
