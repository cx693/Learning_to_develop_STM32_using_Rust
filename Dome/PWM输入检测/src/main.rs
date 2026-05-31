#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    rcc::{self, BusTimerClock},
    timer::pwm_input::*,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("PWM 输入检测启动");

    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz()),
        &mut flash.acr,
    );

    let mut afio = dp.AFIO.constrain(&mut rcc);
    let mut dbg = dp.DBGMCU;

    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    // 禁用 JTAG 释放 PB4/PB5（默认被 JTAG 占用）
    let (_pa15, _pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);
    let pb5 = gpiob.pb5;

    // TIM3 配置为 PWM 输入模式
    // PB4 = IC1（上升沿捕获，测周期）
    // PB5 = IC2（下降沿捕获，测高电平时间）
    let pwm_input = dp.TIM3.remap(&mut afio.mapr).pwm_input(
        (pb4, pb5),
        &mut dbg,
        Configuration::Frequency(10.kHz()),
        &mut rcc,
    );

    let timer_clk = pac::TIM3::timer_clock(&rcc.clocks);
    rprintln!("定时器时钟: {} Hz", timer_clk.raw());

    loop {
        match pwm_input.read_frequency(ReadMode::WaitForNextCapture, timer_clk) {
            Ok(freq) => {
                let freq_hz = freq.raw();
                match pwm_input.read_duty(ReadMode::Instant) {
                    Ok((high, period)) => {
                        let duty_pct = (high as f32 * 100.0) / period as f32;
                        rprintln!(
                            "频率: {} Hz | 占空比: {:.1}% ({}/{})",
                            freq_hz,
                            duty_pct,
                            high,
                            period,
                        );
                    }
                    Err(_) => {
                        rprintln!("频率: {} Hz | 占空比读取失败", freq_hz);
                    }
                }
            }
            Err(Error::FrequencyTooLow) => {
                rprintln!("信号频率过低或无信号");
            }
        }
    }
}
