#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use rtt_target::{rprint, rprintln, rtt_init_print};
use stm32f1xx_hal::{pac, prelude::*, rcc};

const SCAN_START: u8 = 0x08;
const SCAN_END: u8 = 0x78;

fn device_name(addr: u8) -> Option<&'static str> {
    match addr {
        0x03 => Some("General Call"),
        0x08..=0x0F => Some("HS-mode Master"),
        0x10 => Some("LED Driver (IS31FL3731)"),
        0x11 => Some("LED Driver (IS31FL3731)"),
        0x1C => Some("Accel (MMA845xQ/LIS3DH)"),
        0x1D => Some("Accel (ADXL345/MMA845xQ)"),
        0x1E => Some("Magnetometer (HMC5883L)"),
        0x20..=0x27 => Some("I/O Expander (PCF8574)"),
        0x28 => Some("IMU (MPU6050/BNO055)"),
        0x29 => Some("ToF (VL53L0X/VL53L1X)"),
        0x2A..=0x2F => Some("DAC (MCP4725)"),
        0x38 => Some("Sensor (BH1750/AHT20)"),
        0x39 => Some("Light (APDS-9960/TSL2561)"),
        0x3C => Some("OLED SSD1306 (128x32)"),
        0x3D => Some("OLED SSD1306 (128x64)"),
        0x40..=0x43 => Some("ADC/Temp (INA219/TMP102)"),
        0x44..=0x45 => Some("TH Sensor (SHT30/SHT31)"),
        0x48..=0x4B => Some("ADC (ADS1115/ADS1015)"),
        0x50..=0x57 => Some("EEPROM (AT24Cxx)"),
        0x5A => Some("IR Thermo (MLX90614)"),
        0x5C => Some("TH Sensor (AM2320/DHT12)"),
        0x60 => Some("Sensor (SGP30/SI1145)"),
        0x62 => Some("CO2 Sensor (SCD40/SCD41)"),
        0x68 => Some("RTC/IMU (DS3231/MPU6050)"),
        0x69 => Some("IMU (MPU6050 alt addr)"),
        0x6A..=0x6B => Some("IMU (LSM6DS3/LSM9DS1)"),
        0x76..=0x77 => Some("Env Sensor (BME280/BMP280)"),
        _ => None,
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    let rcc_cfg = rcc::Config::hse(8.MHz())
        .sysclk(72.MHz())
        .pclk1(36.MHz())
        .pclk2(72.MHz())
        .adcclk(14.MHz());

    let mut rcc = dp.RCC.freeze(rcc_cfg, &mut flash.acr);

    cortex_m::peripheral::Peripherals::take()
        .unwrap()
        .DWT
        .enable_cycle_counter();

    let mut gpiob = dp.GPIOB.split(&mut rcc);

    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    let mut i2c = dp.I2C1.blocking_i2c(
        (scl, sda),
        stm32f1xx_hal::i2c::Mode::standard(100.kHz()),
        &mut rcc,
        1000,
        10,
        1000,
        1000,
    );
    rprint!("\r\n");
    rprintln!("=============================");
    rprintln!("  I2C Bus Scanner");
    rprintln!("  Range: 0x{:02X} - 0x{:02X}", SCAN_START, SCAN_END - 1);
    rprintln!("  Speed: 100kHz (Standard)");
    rprintln!("=============================");
    rprintln!();

    rprint!("     ");
    for col in 0..16u8 {
        rprint!("{:02X} ", col);
    }
    rprintln!();

    rprint!("    ");
    for _ in 0..16 {
        rprint!("---");
    }
    rprintln!();

    let mut found_count: u8 = 0;
    let mut found_addrs: [u8; 32] = [0u8; 32];

    for row in 0..8u8 {
        rprint!("{:02X}: ", row * 16);

        for col in 0..16u8 {
            let addr = row * 16 + col;

            if addr < SCAN_START || addr >= SCAN_END {
                rprint!("   ");
                continue;
            }

            let data: [u8; 1] = [0];
            match i2c.write(addr, &data) {
                Ok(_) => {
                    rprint!("{:02X}", addr);
                    if (found_count as usize) < found_addrs.len() {
                        found_addrs[found_count as usize] = addr;
                    }
                    found_count += 1;
                }
                Err(_) => {
                    rprint!("..");
                }
            }

            rprint!(" ");
        }
        rprintln!();
    }

    rprintln!();
    rprintln!("=============================");
    rprintln!("  Scan Done: {} device(s)", found_count);
    rprintln!("=============================");

    if found_count > 0 {
        rprintln!();
        for i in 0..found_count.min(32) {
            let addr = found_addrs[i as usize];
            rprint!("  [{}] 0x{:02X}", i + 1, addr);
            rprint!(" (W:0x{:02X} R:0x{:02X})", addr << 1, (addr << 1) | 1);
            if let Some(name) = device_name(addr) {
                rprint!(" {}", name);
            }
            rprintln!();
        }
    }

    rprintln!();
    rprintln!("Tip: 8-bit addr = 7-bit << 1 | R/W bit");

    loop {}
}
