#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

mod dht11;

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, rcc};

use dht11::{Delay, Dht11};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("DHT11 温湿度传感器 - PA6");

    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .adcclk(14.MHz()),
        &mut flash.acr,
    );

    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let mut delay = Delay::new(cp.SYST);

    // PA6 推挽输出，初始高电平
    let mut pin = gpioa.pa6.into_push_pull_output_with_state(
        &mut gpioa.crl,
        stm32f1xx_hal::gpio::PinState::High,
    );

    delay.ms(1500);
    rprintln!("DHT11 初始化完成，开始采集...");

    loop {
        // Dht11::read 接收推挽输出引脚，归还推挽输出引脚
        let (result, returned_pin) = Dht11::read(pin, &mut gpioa.crl, &mut delay);
        pin = returned_pin;

        match result {
            Ok((humi, temp)) => {
                rprintln!("湿度: {}%RH, 温度: {}C", humi, temp);
            }
            Err(e) => {
                rprintln!("DHT11 读取失败: {:?}", e);
            }
        }

        delay.ms(2000);
    }
}
