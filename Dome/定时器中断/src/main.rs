#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

use core::cell::RefCell;

use cortex_m::{asm::wfi, interrupt::Mutex};

use cortex_m_rt::entry;

use panic_halt as _;

use rtt_target::{rprintln, rtt_init_print};

use stm32f1xx_hal::{
    gpio::{GpioExt, Output, PC13, PinState, PushPull},
    pac::{Interrupt, Peripherals, TIM2, interrupt},
    prelude::*,
    rcc,
    timer::{CounterHz, Event},
};

type LedPin = PC13<Output<PushPull>>;

static G_LED: Mutex<RefCell<Option<LedPin>>> = Mutex::new(RefCell::new(None));

static G_TIM: Mutex<RefCell<Option<CounterHz<TIM2>>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn TIM2() {
    cortex_m::interrupt::free(|cs| {
        let mut led_ref = G_LED.borrow(cs).borrow_mut();
        let mut tim_ref = G_TIM.borrow(cs).borrow_mut();

        if let (Some(led), Some(tim)) = (led_ref.as_mut(), tim_ref.as_mut()) {
            led.toggle();
            tim.clear_interrupt(Event::Update);
            rprintln!("TIM2 中断");
        }
    });
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("程序启动");

    let dp = Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();

    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .adcclk(14.MHz()),
        &mut flash.acr,
    );

    let mut gpioc = dp.GPIOC.split(&mut rcc);
    let led = Output::new(gpioc.pc13, &mut gpioc.crh, PinState::High);

    cortex_m::interrupt::free(|cs| {
        *G_LED.borrow(cs).borrow_mut() = Some(led);
    });

    rprintln!("GPIO 初始化完成");

    let mut timer = dp.TIM2.counter_hz(&mut rcc);
    timer.start(6.Hz()).unwrap();
    timer.listen(Event::Update);

    cortex_m::interrupt::free(|cs| {
        *G_TIM.borrow(cs).borrow_mut() = Some(timer);
    });

    rprintln!("TIM2 初始化完成");

    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
    }

    rprintln!("程序运行中");

    loop {
        // Wait For Interrupt，CPU 进入低功耗休眠 --- 生产中建议不写！
        // wfi(); // 启动后会出现 -- rprintln! 无法打印
    }
}
