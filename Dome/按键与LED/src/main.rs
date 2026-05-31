#![allow(clippy::empty_loop)]
#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{gpio::PinState, pac, prelude::*, rcc};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain(); // Flash 等待周期配置

    // 外部晶振
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()) // 使用 8MHz 外部晶振
            .sysclk(72.MHz()) // PLL 倍频到 72MHz
            .pclk1(36.MHz()) // APB1 分频到 36MHz
            .pclk2(72.MHz()) // APB2 不分频
            .adcclk(14.MHz()), // ADC 14MHz
        &mut flash.acr,
    );

    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let mut gpiob = dp.GPIOB.split(&mut rcc);
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    let (mut led1, mut led2) = (
        gpioc
            .pc13
            .into_push_pull_output_with_state(&mut gpioc.crh, PinState::High),
        gpioc
            .pc14
            .into_push_pull_output_with_state(&mut gpioc.crh, PinState::Low),
    );

    // 禁用 JTAG，释放 PA15、PB3、PB4 作为普通 GPIO
    // STM32F1 默认 PA13/PA14/PA15/PB3/PB4 是 JTAG/SWD 引脚
    // 使用普通 GPIO 前必须先释放
    let mut afio = dp.AFIO.constrain(&mut rcc);
    let (gpioa_pa15, _gpiob_pb3, _gpiob_pb4) =
        afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

    let key_0 = gpiob.pb12.into_pull_up_input(&mut gpiob.crh);
    let key_1 = gpioa_pa15.into_pull_up_input(&mut gpioa.crh);

    let mut key_up: bool = true;
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = cp.SYST.delay(&rcc.clocks);
    loop {
        let key_result = (key_0.is_low(), key_1.is_low());
        if key_up && (key_result.0 || key_result.1) {
            key_up = false;
            delay.delay_ms(20_u8);
            rprintln!(
                "{}",
                if key_result.0 {
                    "按下了按键0"
                } else {
                    "按下了按键1"
                }
            );
            match key_result {
                (true, _) => led1.toggle(),
                (_, true) => led2.toggle(),
                (_, _) => (),
            }
        } else if !key_result.0 && !key_result.1 {
            key_up = true;
            // nanos() 纳秒; micros() 微秒; millis() 毫秒; secs() 秒; minutes() 分; hours() 时
            delay.delay(20.millis());
        } else {
            // rprintln!("出错！");
            // delay.delay(2.secs());
        }
    }
}
