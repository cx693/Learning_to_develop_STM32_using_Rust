#![allow(dead_code)]

use embedded_hal::i2c::I2c;
use stm32f1xx_hal::rcc::Clocks;

const ADDR: u8 = 0x38;

#[derive(Debug)]
pub enum Error {
    I2c,
    Crc,
    NotReady,
}

pub struct Dht20<I2C> {
    i2c: I2C,
}

impl<I2C: I2c> Dht20<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub fn init(&mut self, clocks: &Clocks) -> Result<(), Error> {
        Self::delay_ms(100, clocks);

        let mut status = [0u8; 1];
        self.i2c
            .write_read(ADDR, &[0x71], &mut status)
            .map_err(|_| Error::I2c)?;

        if status[0] & 0x80 != 0 {
            self.i2c
                .write(ADDR, &[0x1B, 0x1C, 0x0E, 0x09])
                .map_err(|_| Error::I2c)?;
            Self::delay_ms(10, clocks);
            self.i2c
                .write(ADDR, &[0x1B, 0x1C, 0x0E, 0x09])
                .map_err(|_| Error::I2c)?;
            Self::delay_ms(10, clocks);
            self.i2c
                .write(ADDR, &[0x1B, 0x1C, 0x0E, 0x09])
                .map_err(|_| Error::I2c)?;
            Self::delay_ms(10, clocks);
        }

        Ok(())
    }

    pub fn read(&mut self, clocks: &Clocks) -> Result<(u16, u16), Error> {
        self.i2c
            .write(ADDR, &[0xAC, 0x33, 0x00])
            .map_err(|_| Error::I2c)?;

        Self::delay_ms(80, clocks);

        let mut buf = [0u8; 7];
        self.i2c.read(ADDR, &mut buf).map_err(|_| Error::I2c)?;

        if buf[0] & 0x80 != 0 {
            return Err(Error::NotReady);
        }

        if Self::crc8(&buf[..6]) != buf[6] {
            return Err(Error::Crc);
        }

        let raw_h = ((buf[1] as u32) << 12)
            | ((buf[2] as u32) << 4)
            | ((buf[3] as u32) >> 4);
        let humidity = (raw_h * 1000 / 1048576) as u16;

        let raw_t = (((buf[3] & 0x0F) as u32) << 16)
            | ((buf[4] as u32) << 8)
            | (buf[5] as u32);
        let temp_i = (raw_t * 2000 / 1048576) as i32 - 500;
        let temperature = if temp_i < 0 { 0u16 } else { temp_i as u16 };

        Ok((humidity, temperature))
    }

    fn delay_ms(ms: u32, clocks: &Clocks) {
        let cycles = clocks.sysclk().raw() / 1000 * ms;
        cortex_m::asm::delay(cycles);
    }

    fn crc8(data: &[u8]) -> u8 {
        let mut crc: u8 = 0xFF;
        for &byte in data {
            crc ^= byte;
            for _ in 0..8 {
                if crc & 0x80 != 0 {
                    crc = (crc << 1) ^ 0x31;
                } else {
                    crc <<= 1;
                }
            }
        }
        crc
    }
}
