#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m::asm;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{
    pac,
    prelude::*,
    rcc,
    timer::{Channel, Tim2NoRemap},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("\r\n舵机测试");
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

    let mut afio = dp.AFIO.constrain(&mut rcc);
    let mut gpioa = dp.GPIOA.split(&mut rcc);

    // PA0 -> TIM2 CH1，50Hz（舵机标准频率）
    let pins = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let mut pwm = dp.TIM2.pwm_hz::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 50.Hz(), &mut rcc);
    let max = pwm.get_max_duty();
    pwm.enable(Channel::C1);

    // 舵机脉宽范围：0.5ms ~ 2.5ms（对应 0° ~ 180°）
    // 周期 20ms，占空比 = 脉宽 / 周期
    // duty_0   = max * 0.5 / 20  = max / 40
    // duty_180 = max * 2.5 / 20  = max / 8
    let duty_min = max / 40;   // 0.5ms → 0°
    let duty_max = max / 8;    // 2.5ms → 180°
    let step = (duty_max - duty_min) / 180;  // 每度对应的 duty 增量

    let mut current_duty = duty_min;
    let mut direction_up = true;

    // 72MHz 主频，每步延时约 5ms → 舵机约 0.7 秒完成 0→180
    let delay_cycles: u32 = 72_000 * 5;

    loop {
        pwm.set_duty(Channel::C1, current_duty);

        if direction_up {
            if current_duty >= duty_max {
                direction_up = false;
            } else {
                current_duty += step;
            }
        } else {
            if current_duty <= duty_min {
                direction_up = true;
            } else {
                current_duty -= step;
            }
        }
        rprintln!("--{:>3}度--",(current_duty-duty_min)/step);
        asm::delay(delay_cycles);
    }
}
